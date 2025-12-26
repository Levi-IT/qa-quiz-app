use tauri::State;
use serde::{Deserialize, Serialize};

// 1. Model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: String,
    pub content: String,
    pub answer_a: String,
    pub answer_b: String,
    pub correct_answer: String,
}

// 2. AppState chứa Database
struct AppState {
    db: sled::Db,
}

// --- CÁC COMMANDS ---

#[tauri::command]
fn add_question(state: State<AppState>, content: String, a: String, b: String, correct: String) -> Result<Vec<Question>, String> {
    let id = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis().to_string();
    let q = Question { id: id.clone(), content, answer_a: a, answer_b: b, correct_answer: correct };
    
    // Logic lưu vào Sled
    let json = serde_json::to_string(&q).map_err(|e| e.to_string())?;
    state.db.insert(id.as_bytes(), json.as_bytes()).map_err(|e| e.to_string())?;
    
    // Trả về list
    get_all_questions(state)
}

#[tauri::command]
fn get_all_questions(state: State<AppState>) -> Result<Vec<Question>, String> {
    let mut list = Vec::new();
    for item in state.db.iter() {
        if let Ok((_, value)) = item {
            let s = std::str::from_utf8(&value).unwrap_or("{}");
            if let Ok(q) = serde_json::from_str::<Question>(s) {
                list.push(q);
            }
        }
    }
    Ok(list)
}

#[tauri::command]
fn delete_question(state: State<AppState>, id: String) -> Result<Vec<Question>, String> {
    state.db.remove(id).map_err(|e| e.to_string())?;
    get_all_questions(state)
}

// 3. Entry Point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = sled::open("my_quiz_db").expect("Không mở được DB Sled");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState { db }) 
        .invoke_handler(tauri::generate_handler![
            add_question, 
            get_all_questions, 
            delete_question
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}