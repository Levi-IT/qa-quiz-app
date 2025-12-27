use crate::models::{Question, UserProfile};
use async_trait::async_trait;
use sled::Db;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub trait QuestionRepository: Send + Sync {
    async fn add_question(&self, q: Question) -> Result<(), String>;
    async fn get_all_questions(&self) -> Result<Vec<Question>, String>;
    async fn delete_question(&self, id: &str) -> Result<(), String>;
    async fn update_question(&self, q: Question) -> Result<(), String>;
    
    async fn save_user_profile(&self, user: UserProfile) -> Result<(), String>;
    async fn get_user_profile(&self, uid: &str) -> Result<Option<UserProfile>, String>;
}

// --- SLED IMPLEMENTATION (Local) ---
pub struct SledRepository {
    db: Db,
}

impl SledRepository {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).expect("Failed to open Sled DB");
        Self { db }
    }
}

#[async_trait]
impl QuestionRepository for SledRepository {
    async fn add_question(&self, q: Question) -> Result<(), String> {
        let json = serde_json::to_string(&q).map_err(|e| e.to_string())?;
        self.db
            .insert(q.id.as_bytes(), json.as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_all_questions(&self) -> Result<Vec<Question>, String> {
        let mut list = Vec::new();
        for item in self.db.iter() {
            if let Ok((_, value)) = item {
                let s = std::str::from_utf8(&value).unwrap_or("{}");
                if let Ok(q) = serde_json::from_str::<Question>(s) {
                    list.push(q);
                }
            }
        }
        Ok(list)
    }

    async fn delete_question(&self, id: &str) -> Result<(), String> {
        self.db.remove(id).map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn update_question(&self, q: Question) -> Result<(), String> {
        self.add_question(q).await 
    }

    async fn save_user_profile(&self, user: UserProfile) -> Result<(), String> {
        let json = serde_json::to_string(&user).map_err(|e| e.to_string())?;
        self.db
            .insert(format!("user:{}", user.uid).as_bytes(), json.as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_user_profile(&self, uid: &str) -> Result<Option<UserProfile>, String> {
        let key = format!("user:{}", uid);
        if let Ok(Some(value)) = self.db.get(key) {
            let s = std::str::from_utf8(&value).unwrap_or("{}");
            let u = serde_json::from_str::<UserProfile>(s).map_err(|e| e.to_string())?;
            return Ok(Some(u));
        }
        Ok(None)
    }
}

// --- FIREBASE IMPLEMENTATION (Remote via Reqwest) ---
pub struct FirebaseRepository {
    client: reqwest::Client,
    base_url: String,
}

impl FirebaseRepository {
    pub fn new(url: &str) -> Self {
        // Ensure URL doesn't end with slash for consistency
        let base_url = url.trim_end_matches('/').to_string();
        Self { 
            client: reqwest::Client::new(),
            base_url 
        }
    }
}

#[async_trait]
impl QuestionRepository for FirebaseRepository {
    async fn add_question(&self, q: Question) -> Result<(), String> {
        let url = format!("{}/questions/{}.json", self.base_url, q.id);
        println!("Firebase DB: PUT {}", url);
        
        let res = self.client.put(&url)
            .json(&q)
            .send()
            .await
            .map_err(|e| format!("Network Error: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Firebase Error: {}", res.status()));
        }
        Ok(())
    }

    async fn get_all_questions(&self) -> Result<Vec<Question>, String> {
        let url = format!("{}/questions.json", self.base_url);
        println!("Firebase DB: GET {}", url);

        let res = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Network Error: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Firebase Error: {}", res.status()));
        }

        let json_value: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

        if json_value.is_null() {
            return Ok(Vec::new());
        }

        let map: HashMap<String, Question> = serde_json::from_value(json_value)
            .map_err(|e| format!("Parse error: {}", e))?;

        Ok(map.into_values().collect())
    }

    async fn delete_question(&self, id: &str) -> Result<(), String> {
        let url = format!("{}/questions/{}.json", self.base_url, id);
        println!("Firebase DB: DELETE {}", url);

        let res = self.client.delete(&url)
            .send()
            .await
            .map_err(|e| format!("Network Error: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Firebase Error: {}", res.status()));
        }
        Ok(())
    }

    async fn update_question(&self, q: Question) -> Result<(), String> {
        self.add_question(q).await 
    }

    async fn save_user_profile(&self, user: UserProfile) -> Result<(), String> {
        let url = format!("{}/users/{}.json", self.base_url, user.uid);
        println!("Firebase DB: PUT {}", url);

        let res = self.client.put(&url)
            .json(&user)
            .send()
            .await
            .map_err(|e| format!("Network Error: {}", e))?;

        if !res.status().is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Firebase Error: {} - {}", text, url));
        }
        Ok(())
    }

    async fn get_user_profile(&self, uid: &str) -> Result<Option<UserProfile>, String> {
        let url = format!("{}/users/{}.json", self.base_url, uid);
        println!("Firebase DB: GET {}", url);

        let res = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Network Error: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Firebase Error: {}", res.status()));
        }

        let json_value: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

        if json_value.is_null() {
            return Ok(None);
        }

        let u: UserProfile = serde_json::from_value(json_value)
            .map_err(|e| format!("Parse error: {}", e))?;
        Ok(Some(u))
    }
}

pub struct Database {
    pub repo: Arc<dyn QuestionRepository>,
}