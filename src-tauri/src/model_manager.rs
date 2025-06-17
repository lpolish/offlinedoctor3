use anyhow::{anyhow, Result};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub size: u64,
    pub description: String,
    pub download_url: String,
    pub filename: String,
    pub is_downloaded: bool,
}

pub struct ModelManager {
    models_dir: PathBuf,
}

impl ModelManager {
    pub fn new(app_data_dir: PathBuf) -> Result<Self> {
        let models_dir = app_data_dir.join("models");
        fs::create_dir_all(&models_dir)?;

        Ok(ModelManager { models_dir })
    }

    pub fn get_available_models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                name: "Llama 3.2 3B Instruct (Q4)".to_string(),
                size: 2_100_000_000, // ~2.1GB
                description: "Compact model suitable for general medical queries".to_string(),
                download_url: "https://huggingface.co/bartowski/Llama-3.2-3B-Instruct-GGUF/resolve/main/Llama-3.2-3B-Instruct-Q4_K_M.gguf".to_string(),
                filename: "llama-3.2-3b-instruct-q4.gguf".to_string(),
                is_downloaded: self.is_model_downloaded("llama-3.2-3b-instruct-q4.gguf"),
            },
            ModelInfo {
                name: "Llama 3.1 8B Instruct (Q4)".to_string(),
                size: 4_700_000_000, // ~4.7GB
                description: "Higher quality model for complex medical reasoning".to_string(),
                download_url: "https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf".to_string(),
                filename: "llama-3.1-8b-instruct-q4.gguf".to_string(),
                is_downloaded: self.is_model_downloaded("llama-3.1-8b-instruct-q4.gguf"),
            },
            ModelInfo {
                name: "OpenBioLLM 8B (Q4)".to_string(),
                size: 4_800_000_000, // ~4.8GB
                description: "Medical-specific model trained on biomedical literature".to_string(),
                download_url: "https://huggingface.co/aaditya/OpenBioLLM-Llama3-8B-GGUF/resolve/main/openbiollm-llama3-8b.Q4_K_M.gguf".to_string(),
                filename: "openbiollm-llama3-8b-q4.gguf".to_string(),
                is_downloaded: self.is_model_downloaded("openbiollm-llama3-8b-q4.gguf"),
            },
        ]
    }

    fn is_model_downloaded(&self, filename: &str) -> bool {
        self.models_dir.join(filename).exists()
    }

    pub fn get_model_path(&self, filename: &str) -> PathBuf {
        self.models_dir.join(filename)
    }

    pub async fn download_model(
        &self,
        model: &ModelInfo,
        progress_callback: Option<Box<dyn Fn(u64, u64) + Send>>,
    ) -> Result<PathBuf> {
        let model_path = self.models_dir.join(&model.filename);

        if model_path.exists() {
            return Ok(model_path);
        }

        println!("Downloading model: {}", model.name);

        let response = reqwest::get(&model.download_url).await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to download model: HTTP {}",
                response.status()
            ));
        }

        let total_size = response.content_length().unwrap_or(0);
        let mut downloaded = 0u64;
        let mut file = fs::File::create(&model_path)?;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk)?;
            downloaded += chunk.len() as u64;

            if let Some(ref callback) = progress_callback {
                callback(downloaded, total_size);
            }
        }

        println!("Model downloaded successfully: {}", model_path.display());
        Ok(model_path)
    }

    pub fn delete_model(&self, filename: &str) -> Result<()> {
        let model_path = self.models_dir.join(filename);
        if model_path.exists() {
            fs::remove_file(model_path)?;
        }
        Ok(())
    }

    pub fn get_downloaded_models(&self) -> Vec<ModelInfo> {
        self.get_available_models()
            .into_iter()
            .filter(|model| model.is_downloaded)
            .collect()
    }

    pub fn get_default_model_path(&self) -> Option<PathBuf> {
        // Return the first downloaded model, prioritizing smaller ones first
        let downloaded = self.get_downloaded_models();
        downloaded
            .first()
            .map(|model| self.get_model_path(&model.filename))
    }
}
