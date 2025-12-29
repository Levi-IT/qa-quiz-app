use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: String,
    pub content: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub created_by: String,
    pub answer_a: String,
    pub answer_b: String,
    pub answer_c: String,
    pub answer_d: String,
    pub correct_answer: String,
    
    #[serde(default)]
    pub last_modified: i64,
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub synced: bool,
}
    
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct UserProfile {
        pub uid: String,
        pub name: String,
        pub rank: String,
        pub unit: String,
        pub is_admin: bool,
        pub email: String,
    }
    