use crate::models::{Question, UserProfile};
use async_trait::async_trait;
use sled::Db;
use std::collections::HashMap;
use std::sync::Arc;
use chrono::Utc;

#[async_trait]
pub trait QuestionRepository: Send + Sync {
    async fn add_question(&self, q: Question) -> Result<(), String>;
    async fn get_question(&self, id: &str) -> Result<Option<Question>, String>;
    async fn get_all_questions(&self) -> Result<Vec<Question>, String>;
    async fn delete_question(&self, id: &str) -> Result<(), String>;
    async fn update_question(&self, q: Question) -> Result<(), String>;
    
    async fn save_user_profile(&self, user: UserProfile) -> Result<(), String>;
    async fn get_user_profile(&self, uid: &str) -> Result<Option<UserProfile>, String>;
    
    // New sync method
    async fn sync_data(&self) -> Result<(), String>;
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
        self.db.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_question(&self, id: &str) -> Result<Option<Question>, String> {
        if let Ok(Some(value)) = self.db.get(id) {
            let s = std::str::from_utf8(&value).unwrap_or("{}");
            let q = serde_json::from_str::<Question>(s).map_err(|e| e.to_string())?;
            return Ok(Some(q));
        }
        Ok(None)
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
        self.db.flush().map_err(|e| e.to_string())?;
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
        self.db.flush().map_err(|e| e.to_string())?;
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

    async fn sync_data(&self) -> Result<(), String> {
        Ok(()) // No-op for purely local
    }
}

// --- FIREBASE IMPLEMENTATION (Remote) ---
pub struct FirebaseRepository {
    client: reqwest::Client,
    base_url: String,
}

impl FirebaseRepository {
    pub fn new(url: &str) -> Self {
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
        // println!("Firebase DB: PUT {}", url);
        
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

    async fn get_question(&self, id: &str) -> Result<Option<Question>, String> {
        let url = format!("{}/questions/{}.json", self.base_url, id);
        // println!("Firebase DB: GET {}", url);

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

        let q: Question = serde_json::from_value(json_value)
            .map_err(|e| format!("Parse error: {}", e))?;
        Ok(Some(q))
    }

    async fn get_all_questions(&self) -> Result<Vec<Question>, String> {
        let url = format!("{}/questions.json", self.base_url);
        // println!("Firebase DB: GET {}", url);

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
        // NOTE: In the new logic, we likely use add_question with deleted=true
        // But for completeness, this is a hard delete on Firebase
        let url = format!("{}/questions/{}.json", self.base_url, id);
        // println!("Firebase DB: DELETE {}", url);

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
        // println!("Firebase DB: PUT {}", url);

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
        // println!("Firebase DB: GET {}", url);

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

    async fn sync_data(&self) -> Result<(), String> {
        Ok(())
    }
}

// --- SYNCED REPOSITORY (Hybrid) ---
pub struct SyncedRepository {
    local: Arc<SledRepository>,
    remote: Arc<FirebaseRepository>,
}

impl SyncedRepository {
    pub fn new(local_path: &str, remote_url: &str) -> Self {
        Self {
            local: Arc::new(SledRepository::new(local_path)),
            remote: Arc::new(FirebaseRepository::new(remote_url)),
        }
    }
}

#[async_trait]
impl QuestionRepository for SyncedRepository {
    async fn add_question(&self, mut q: Question) -> Result<(), String> {
        // 1. Prepare Data
        q.last_modified = Utc::now().timestamp_millis();
        q.synced = false;
        q.deleted = false;

        // 2. Save Local (Critical)
        self.local.add_question(q.clone()).await?;
        
        // 3. Try Immediate Sync (Optimistic)
        // If successful, update 'synced = true' and save local again
        // println!("Attempting immediate sync...");
        match self.remote.add_question(q.clone()).await {
            Ok(_) => {
                let mut synced_q = q.clone();
                synced_q.synced = true;
                self.local.add_question(synced_q).await?;
                // println!("Immediate sync success.");
            },
            Err(e) => {
                println!("Immediate sync failed (will retry later): {}", e);
            }
        }
        Ok(())
    }

    async fn get_question(&self, id: &str) -> Result<Option<Question>, String> {
        self.local.get_question(id).await
    }

    async fn get_all_questions(&self) -> Result<Vec<Question>, String> {
        let all = self.local.get_all_questions().await?;
        // Filter out deleted items for the UI
        let mut filtered: Vec<Question> = all.into_iter().filter(|q| !q.deleted).collect();
        // Sort by ID descending (newest first), assuming ID is a timestamp-based string
        filtered.sort_by(|a, b| b.id.cmp(&a.id));
        Ok(filtered)
    }

    async fn delete_question(&self, id: &str) -> Result<(), String> {
        // Soft Delete Logic
        if let Some(mut q) = self.local.get_question(id).await? {
            q.deleted = true;
            q.last_modified = Utc::now().timestamp_millis();
            q.synced = false;

            // Save local
            self.local.add_question(q.clone()).await?;

            // Try sync
            match self.remote.add_question(q.clone()).await {
                Ok(_) => {
                    let mut synced_q = q.clone();
                    synced_q.synced = true;
                    self.local.add_question(synced_q).await?;
                },
                Err(e) => {
                    println!("Delete sync failed (will retry later): {}", e);
                }
            }
        }
        Ok(())
    }

    async fn update_question(&self, q: Question) -> Result<(), String> {
        // Same as add_question but preserves ID
        self.add_question(q).await
    }

    async fn save_user_profile(&self, user: UserProfile) -> Result<(), String> {
        self.local.save_user_profile(user.clone()).await?;
        let _ = self.remote.save_user_profile(user).await;
        Ok(())
    }

    async fn get_user_profile(&self, uid: &str) -> Result<Option<UserProfile>, String> {
        let local_profile = self.local.get_user_profile(uid).await?;
        if local_profile.is_some() {
            return Ok(local_profile);
        }
        if let Ok(Some(remote_profile)) = self.remote.get_user_profile(uid).await {
            self.local.save_user_profile(remote_profile.clone()).await?;
            return Ok(Some(remote_profile));
        }
        Ok(None)
    }

    async fn sync_data(&self) -> Result<(), String> {
        println!("Starting Sync...");

        // 1. PUSH: Local -> Remote
        // Find all local items that are NOT synced
        let local_all = self.local.get_all_questions().await?;
        let pending_push: Vec<_> = local_all.iter().filter(|q| !q.synced).collect();
        let pushed_count = pending_push.len();
        
        println!("Pushing {} items...", pushed_count);
        for q in pending_push {
            // Push to remote
            if let Err(e) = self.remote.add_question(q.clone()).await {
                println!("Failed to push question {}: {}", q.id, e);
            } else {
                // Mark as synced locally
                let mut synced_q = q.clone();
                synced_q.synced = true;
                self.local.add_question(synced_q).await?;
            }
        }

        // 2. PULL: Remote -> Local
        // Fetch all from remote
        println!("Pulling from remote...");
        let remote_all = self.remote.get_all_questions().await?;
        
        // Re-read local to get latest state
        let local_map: HashMap<String, Question> = self.local.get_all_questions().await?
            .into_iter()
            .map(|q| (q.id.clone(), q))
            .collect();

        let mut updates_count = 0;
        for remote_q in remote_all {
            let should_update = match local_map.get(&remote_q.id) {
                Some(local_q) => {
                    // Update if remote is newer
                    remote_q.last_modified > local_q.last_modified
                },
                None => true, // New item from remote
            };

            if should_update {
                // Save to local, marking it as synced (since it came from remote)
                let mut to_save = remote_q.clone();
                to_save.synced = true; 
                self.local.add_question(to_save).await?;
                updates_count += 1;
            }
        }

        println!("Sync Complete. Pushed: {}, Pulled/Updated: {}", pushed_count, updates_count);
        Ok(())
    }
}

pub struct Database {
    pub repo: Arc<dyn QuestionRepository>,
}