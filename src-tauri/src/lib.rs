mod auth;
mod db;
mod models;

use crate::db::{Database, SyncedRepository};
use crate::models::{Question, UserProfile};
use std::sync::Arc;
use tauri::State;

// --- AUTH COMMANDS ---

#[tauri::command]
async fn register_user(
    state: State<'_, Database>,
    email: String,
    password: String,
    name: String,
    rank: String,
    unit: String,
) -> Result<String, String> {
    // 1. Create Auth Account (Google Identity Toolkit)
    let auth_res = auth::register_user(&email, &password)
        .await
        .map_err(|e| e.to_string())?;

    // 2. Create Profile Data
    let is_admin = email.to_lowercase().contains("admin");
    let user = UserProfile {
        uid: auth_res.local_id,
        name,
        rank,
        unit,
        is_admin,
        email: email.clone(),
    };

    // 3. Save Profile to DB (Realtime Database)
    state.repo.save_user_profile(user).await?;

    Ok("Success".to_string())
}

#[tauri::command]
async fn login_user(
    state: State<'_, Database>,
    email: String,
    password: String,
) -> Result<UserProfile, String> {
    // 1. Verify Credentials
    let auth_res = auth::login_user(&email, &password)
        .await
        .map_err(|e| e.to_string())?;

    // 2. Fetch Profile from DB
    let profile_opt = state.repo.get_user_profile(&auth_res.local_id).await?;

    match profile_opt {
        Some(p) => Ok(p),
        None => {
            Ok(UserProfile {
                uid: auth_res.local_id,
                name: "Chiến sĩ".to_string(),
                rank: "Học viên".to_string(),
                unit: "N/A".to_string(),
                is_admin: false,
                email,
            })
        }
    }
}

// --- QUESTION COMMANDS ---

#[tauri::command]
async fn sync_data(state: State<'_, Database>) -> Result<String, String> {
    state.repo.sync_data().await?;
    Ok("Synced".to_string())
}

#[tauri::command]
async fn add_question(
    state: State<'_, Database>,
    content: String,
    category: String,
    created_by: String,
    a: String,
    b: String,
    c: String,
    d: String,
    correct: String,
) -> Result<Vec<Question>, String> {
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();
    let q = Question {
        id: id.clone(),
        content,
        category,
        created_by,
        answer_a: a,
        answer_b: b,
        answer_c: c,
        answer_d: d,
        correct_answer: correct,
        last_modified: 0,
        deleted: false,
        synced: false,
    };

    state.repo.add_question(q).await?;
    state.repo.get_all_questions().await
}

#[tauri::command]
async fn get_all_questions(state: State<'_, Database>) -> Result<Vec<Question>, String> {
    state.repo.get_all_questions().await
}

#[tauri::command]
async fn delete_question(state: State<'_, Database>, id: String) -> Result<Vec<Question>, String> {
    state.repo.delete_question(&id).await?;
    state.repo.get_all_questions().await
}

#[tauri::command]
async fn update_question(
    state: State<'_, Database>,
    id: String,
    content: String,
    category: String,
    created_by: String,
    a: String,
    b: String,
    c: String,
    d: String,
    correct: String,
) -> Result<Vec<Question>, String> {
    let q = Question {
        id,
        content,
        category,
        created_by,
        answer_a: a,
        answer_b: b,
        answer_c: c,
        answer_d: d,
        correct_answer: correct,
        last_modified: 0,
        deleted: false,
        synced: false,
    };
    state.repo.update_question(q).await?;
    state.repo.get_all_questions().await
}

// 3. Entry Point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load .env file
    dotenv::dotenv().ok();

    // Initialize Firebase Repository
    let db_url = std::env::var("FIREBASE_DB_URL").expect("FIREBASE_DB_URL must be set in .env");
    println!("Initializing Synced Repository with Remote URL: {}", db_url);
    
    // Use a local folder inside target/ to avoid triggering rebuilds on change
    let synced_repo = SyncedRepository::new("target/my_quiz_db", &db_url);
    
    // Wrap in Arc and Database struct for Tauri State
    let db_state = Database {
        repo: Arc::new(synced_repo),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(db_state)
        .invoke_handler(tauri::generate_handler![
            sync_data,
            add_question,
            get_all_questions,
            delete_question,
            update_question,
            register_user,
            login_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}