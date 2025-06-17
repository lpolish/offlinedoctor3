mod ai_engine;
mod database;
mod model_manager;

use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};
use ai_engine::{AIEngine, ChatRequest, ChatResponse, ChatMessage};
use database::{Database, Conversation};
use model_manager::{ModelManager, ModelInfo};
use anyhow::Result;

// Application state
pub struct AppState {
    pub ai_engine: Arc<Mutex<Option<AIEngine>>>,
    pub database: Arc<Mutex<Option<Database>>>,
    pub model_manager: Arc<Mutex<Option<Arc<ModelManager>>>>,
}

#[tauri::command]
async fn initialize_app(app_handle: AppHandle) -> Result<String, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    // Initialize database
    let db_path = app_data_dir.join("offline_doctor.db");
    let database = Database::new(db_path)
        .map_err(|e| format!("Failed to initialize database: {}", e))?;

    // Initialize model manager
    let model_manager = ModelManager::new(app_data_dir.clone())
        .map_err(|e| format!("Failed to initialize model manager: {}", e))?;

    // Store in app state
    let state = app_handle.state::<AppState>();
    *state.database.lock().unwrap() = Some(database);
    *state.model_manager.lock().unwrap() = Some(Arc::new(model_manager));

    Ok("Application initialized successfully".to_string())
}

#[tauri::command]
async fn get_available_models(app_handle: AppHandle) -> Result<Vec<ModelInfo>, String> {
    let state = app_handle.state::<AppState>();
    let model_manager = {
        let model_manager_guard = state.model_manager.lock().unwrap();
        model_manager_guard.clone()
    };
    
    if let Some(model_manager) = model_manager {
        Ok(model_manager.get_available_models())
    } else {
        Err("Model manager not initialized".to_string())
    }
}

#[tauri::command]
async fn download_model(app_handle: AppHandle, model: ModelInfo) -> Result<String, String> {
    let state = app_handle.state::<AppState>();
    
    // Clone the model manager to avoid holding the lock across await
    let model_manager = {
        let model_manager_guard = state.model_manager.lock().unwrap();
        model_manager_guard.clone()
    };
    
    if let Some(model_manager) = model_manager {
        let model_path = model_manager
            .download_model(&model, None)
            .await
            .map_err(|e| format!("Failed to download model: {}", e))?;
        
        Ok(format!("Model downloaded to: {}", model_path.display()))
    } else {
        Err("Model manager not initialized".to_string())
    }
}

#[tauri::command]
async fn initialize_ai_engine(app_handle: AppHandle, model_filename: String) -> Result<String, String> {
    let state = app_handle.state::<AppState>();
    
    // Get model path without holding the lock
    let model_path = {
        let model_manager_guard = state.model_manager.lock().unwrap();
        if let Some(model_manager) = model_manager_guard.as_ref() {
            model_manager.get_model_path(&model_filename)
        } else {
            return Err("Model manager not initialized".to_string());
        }
    };
    
    if !model_path.exists() {
        return Err(format!("Model file not found: {}", model_path.display()));
    }

    let ai_engine = AIEngine::new(model_path);
    
    ai_engine
        .initialize()
        .await
        .map_err(|e| format!("Failed to initialize AI engine: {}", e))?;

    // Now store the initialized engine
    *state.ai_engine.lock().unwrap() = Some(ai_engine);
    
    Ok("AI engine initialized successfully".to_string())
}

#[tauri::command]
async fn send_chat_message(app_handle: AppHandle, request: ChatRequest) -> Result<ChatResponse, String> {
    let state = app_handle.state::<AppState>();
    
    // Get conversation ID or create new one
    let conversation_id = if let Some(id) = request.conversation_id {
        id
    } else {
        let database = {
            let db_guard = state.database.lock().unwrap();
            db_guard.as_ref().cloned()
        };
        
        if let Some(database) = database {
            let title = if request.message.len() > 50 {
                format!("{}...", &request.message[..47])
            } else {
                request.message.clone()
            };
            database
                .create_conversation(&title)
                .map_err(|e| format!("Failed to create conversation: {}", e))?
        } else {
            return Err("Database not initialized".to_string());
        }
    };

    // Store user message
    let _user_message_id = {
        let database = {
            let db_guard = state.database.lock().unwrap();
            db_guard.as_ref().cloned()
        };
        
        if let Some(database) = database {
            database
                .add_message(&conversation_id, "user", &request.message)
                .map_err(|e| format!("Failed to store user message: {}", e))?
        } else {
            return Err("Database not initialized".to_string());
        }
    };

    // Get conversation history
    let conversation_history = {
        let database = {
            let db_guard = state.database.lock().unwrap();
            db_guard.as_ref().cloned()
        };
        
        if let Some(database) = database {
            database
                .get_conversation_messages(&conversation_id)
                .map_err(|e| format!("Failed to get conversation history: {}", e))?
        } else {
            return Err("Database not initialized".to_string());
        }
    };

    // Generate AI response
    let ai_response = {
        let ai_engine = {
            let ai_guard = state.ai_engine.lock().unwrap();
            ai_guard.as_ref().cloned()
        };
        
        if let Some(ai_engine) = ai_engine {
            ai_engine
                .generate_response(&request.message, &conversation_history)
                .await
                .map_err(|e| format!("Failed to generate AI response: {}", e))?
        } else {
            return Err("AI engine not initialized".to_string());
        }
    };

    // Store AI response
    let assistant_message_id = {
        let database = {
            let db_guard = state.database.lock().unwrap();
            db_guard.as_ref().cloned()
        };
        
        if let Some(database) = database {
            database
                .add_message(&conversation_id, "assistant", &ai_response)
                .map_err(|e| format!("Failed to store AI response: {}", e))?
        } else {
            return Err("Database not initialized".to_string());
        }
    };

    Ok(ChatResponse {
        message: ai_response,
        conversation_id,
        message_id: assistant_message_id,
    })
}

#[tauri::command]
async fn get_conversations(app_handle: AppHandle) -> Result<Vec<Conversation>, String> {
    let state = app_handle.state::<AppState>();
    let database = {
        let db_guard = state.database.lock().unwrap();
        db_guard.clone()
    };
    
    if let Some(database) = database {
        database
            .get_conversations()
            .map_err(|e| format!("Failed to get conversations: {}", e))
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
async fn get_conversation_messages(app_handle: AppHandle, conversation_id: String) -> Result<Vec<ChatMessage>, String> {
    let state = app_handle.state::<AppState>();
    let database = {
        let db_guard = state.database.lock().unwrap();
        db_guard.clone()
    };
    
    if let Some(database) = database {
        database
            .get_conversation_messages(&conversation_id)
            .map_err(|e| format!("Failed to get conversation messages: {}", e))
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
async fn delete_conversation(app_handle: AppHandle, conversation_id: String) -> Result<String, String> {
    let state = app_handle.state::<AppState>();
    let database = {
        let db_guard = state.database.lock().unwrap();
        db_guard.clone()
    };
    
    if let Some(database) = database {
        database
            .delete_conversation(&conversation_id)
            .map_err(|e| format!("Failed to delete conversation: {}", e))?;
        Ok("Conversation deleted successfully".to_string())
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState {
            ai_engine: Arc::new(Mutex::new(None)),
            database: Arc::new(Mutex::new(None)),
            model_manager: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            initialize_app,
            get_available_models,
            download_model,
            initialize_ai_engine,
            send_chat_message,
            get_conversations,
            get_conversation_messages,
            delete_conversation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
