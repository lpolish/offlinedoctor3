use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: String, // "user" or "assistant"
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub conversation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: String,
    pub conversation_id: String,
    pub message_id: String,
}

#[derive(Clone)]
pub struct AIEngine {
    llama_process: Arc<Mutex<Option<Child>>>,
    server_port: u16,
    model_path: PathBuf,
    pub is_ready: Arc<Mutex<bool>>,
}

impl AIEngine {
    pub fn new(model_path: PathBuf) -> Self {
        Self {
            llama_process: Arc::new(Mutex::new(None)),
            server_port: 8080,
            model_path,
            is_ready: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        // First, check if model exists
        if !self.model_path.exists() {
            return Err(anyhow!("Model file not found at {:?}", self.model_path));
        }

        // Start llama.cpp server
        self.start_llama_server().await?;
        
        // Wait for server to be ready
        self.wait_for_server().await?;
        
        // Mark as ready
        *self.is_ready.lock().unwrap() = true;
        
        Ok(())
    }

    async fn start_llama_server(&self) -> Result<()> {
        let binary_path = self.get_llama_binary_path()?;
        
        let mut command = Command::new(binary_path);
        command
            .arg("-m").arg(&self.model_path)
            .arg("--port").arg(self.server_port.to_string())
            .arg("--host").arg("127.0.0.1")
            .arg("--ctx-size").arg("4096")
            .arg("--batch-size").arg("512")
            .arg("--threads").arg("4")
            .arg("--n-gpu-layers").arg("0") // CPU only for maximum compatibility
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let child = command.spawn()
            .map_err(|e| anyhow!("Failed to start llama.cpp server: {}", e))?;

        *self.llama_process.lock().unwrap() = Some(child);
        
        Ok(())
    }

    fn get_llama_binary_path(&self) -> Result<PathBuf> {
        // Try to find llama.cpp binary in various locations
        let binary_name = if cfg!(windows) { "llama-server.exe" } else { "llama-server" };
        
        // Check system PATH first
        if let Ok(path) = which::which(binary_name) {
            return Ok(path);
        }
        
        // Check common installation directories
        let common_paths = vec![
            "/usr/local/bin",
            "/usr/bin",
            "/opt/llama.cpp/bin",
        ];
        
        for path_str in common_paths {
            let path = PathBuf::from(path_str).join(binary_name);
            if path.exists() {
                return Ok(path);
            }
        }
        
        Err(anyhow!("llama.cpp binary not found. Please install llama.cpp or ensure 'llama-server' is in your PATH."))
    }

    async fn wait_for_server(&self) -> Result<()> {
        let client = reqwest::Client::new();
        let health_url = format!("http://127.0.0.1:{}/health", self.server_port);
        
        for _ in 0..30 { // Wait up to 30 seconds
            tokio::time::sleep(Duration::from_secs(1)).await;
            
            if let Ok(response) = client.get(&health_url).send().await {
                if response.status().is_success() {
                    return Ok(());
                }
            }
        }
        
        Err(anyhow!("llama.cpp server failed to start within timeout"))
    }

    pub async fn generate_response(&self, prompt: &str, conversation_context: &[ChatMessage]) -> Result<String> {
        if !*self.is_ready.lock().unwrap() {
            return Err(anyhow!("AI engine not ready"));
        }

        let full_prompt = self.build_medical_prompt(prompt, conversation_context);
        
        let client = reqwest::Client::new();
        let completion_url = format!("http://127.0.0.1:{}/completion", self.server_port);
        
        let request_body = serde_json::json!({
            "prompt": full_prompt,
            "n_predict": 512,
            "temperature": 0.7,
            "top_p": 0.9,
            "top_k": 40,
            "repeat_penalty": 1.1,
            "stop": ["Human:", "User:", "\n\n"]
        });

        let response = client
            .post(&completion_url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send request to llama.cpp: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("llama.cpp server returned error: {}", response.status()));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse response: {}", e))?;

        let content = response_json["content"]
            .as_str()
            .unwrap_or("I apologize, but I encountered an error generating a response.")
            .trim()
            .to_string();

        Ok(content)
    }

    fn build_medical_prompt(&self, user_message: &str, conversation_context: &[ChatMessage]) -> String {
        let mut prompt = String::new();
        
        // System prompt for medical context
        prompt.push_str("You are an AI medical assistant designed to help healthcare professionals, particularly resident doctors in remote locations. You provide information about medical conditions, symptoms, differential diagnoses, and treatment options. Always remind users that your responses are for educational purposes and should not replace clinical judgment or proper medical evaluation.\n\n");
        
        // Add conversation context
        for message in conversation_context.iter().rev().take(10).rev() { // Last 10 messages
            match message.role.as_str() {
                "user" => prompt.push_str(&format!("Human: {}\n", message.content)),
                "assistant" => prompt.push_str(&format!("Assistant: {}\n", message.content)),
                _ => {}
            }
        }
        
        // Add current user message
        prompt.push_str(&format!("Human: {}\nAssistant: ", user_message));
        
        prompt
    }

    pub fn shutdown(&self) -> Result<()> {
        if let Some(mut child) = self.llama_process.lock().unwrap().take() {
            child.kill().map_err(|e| anyhow!("Failed to kill llama.cpp process: {}", e))?;
        }
        *self.is_ready.lock().unwrap() = false;
        Ok(())
    }
}

impl Drop for AIEngine {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}
