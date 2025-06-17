use sled::{Db, IVec};
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::ai_engine::ChatMessage;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Database {
    db: Db,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let db = sled::open(db_path)?;
        Ok(Database { db })
    }

    pub fn create_conversation(&self, title: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let conversation = Conversation {
            id: id.clone(),
            title: title.to_string(),
            created_at: now,
            updated_at: now,
        };
        
        let key = format!("conversation:{}", id);
        let value = serde_json::to_vec(&conversation)?;
        self.db.insert(key, value)?;
        
        Ok(id)
    }

    pub fn get_conversations(&self) -> Result<Vec<Conversation>> {
        let mut conversations = Vec::new();
        
        for result in self.db.scan_prefix("conversation:") {
            let (_key, value) = result?;
            let conversation: Conversation = serde_json::from_slice(&value)?;
            conversations.push(conversation);
        }
        
        // Sort by updated_at descending (most recent first)
        conversations.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        
        Ok(conversations)
    }

    pub fn add_message(&self, conversation_id: &str, role: &str, content: &str) -> Result<String> {
        let message_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let message = ChatMessage {
            id: message_id.clone(),
            role: role.to_string(),
            content: content.to_string(),
            timestamp: now,
        };
        
        let key = format!("message:{}:{}", conversation_id, message_id);
        let value = serde_json::to_vec(&message)?;
        self.db.insert(key, value)?;
        
        // Update conversation's updated_at timestamp
        self.update_conversation_timestamp(conversation_id)?;
        
        Ok(message_id)
    }

    pub fn get_conversation_messages(&self, conversation_id: &str) -> Result<Vec<ChatMessage>> {
        let mut messages = Vec::new();
        let prefix = format!("message:{}:", conversation_id);
        
        for result in self.db.scan_prefix(&prefix) {
            let (_key, value) = result?;
            let message: ChatMessage = serde_json::from_slice(&value)?;
            messages.push(message);
        }
        
        // Sort by timestamp ascending (chronological order)
        messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        Ok(messages)
    }

    pub fn delete_conversation(&self, conversation_id: &str) -> Result<()> {
        // Delete the conversation
        let conv_key = format!("conversation:{}", conversation_id);
        self.db.remove(conv_key)?;
        
        // Delete all messages in the conversation
        let message_prefix = format!("message:{}:", conversation_id);
        let keys_to_delete: Vec<IVec> = self.db
            .scan_prefix(&message_prefix)
            .map(|result| result.map(|(key, _)| key))
            .collect::<Result<Vec<_>, _>>()?;
        
        for key in keys_to_delete {
            self.db.remove(key)?;
        }
        
        Ok(())
    }

    pub fn update_conversation_title(&self, conversation_id: &str, title: &str) -> Result<()> {
        let key = format!("conversation:{}", conversation_id);
        
        if let Some(value) = self.db.get(&key)? {
            let mut conversation: Conversation = serde_json::from_slice(&value)?;
            conversation.title = title.to_string();
            conversation.updated_at = Utc::now();
            
            let updated_value = serde_json::to_vec(&conversation)?;
            self.db.insert(key, updated_value)?;
        } else {
            return Err(anyhow!("Conversation not found"));
        }
        
        Ok(())
    }

    fn update_conversation_timestamp(&self, conversation_id: &str) -> Result<()> {
        let key = format!("conversation:{}", conversation_id);
        
        if let Some(value) = self.db.get(&key)? {
            let mut conversation: Conversation = serde_json::from_slice(&value)?;
            conversation.updated_at = Utc::now();
            
            let updated_value = serde_json::to_vec(&conversation)?;
            self.db.insert(key, updated_value)?;
        }
        
        Ok(())
    }

    pub fn get_conversation(&self, conversation_id: &str) -> Result<Option<Conversation>> {
        let key = format!("conversation:{}", conversation_id);
        
        if let Some(value) = self.db.get(&key)? {
            let conversation: Conversation = serde_json::from_slice(&value)?;
            Ok(Some(conversation))
        } else {
            Ok(None)
        }
    }

    pub fn clear_all_data(&self) -> Result<()> {
        self.db.clear()?;
        Ok(())
    }
}
