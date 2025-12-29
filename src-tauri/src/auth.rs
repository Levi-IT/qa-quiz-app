use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Serialize)]
struct AuthRequest<'a> {
    email: &'a str,
    password: &'a str,
    return_secure_token: bool,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct AuthResponse {
    #[serde(rename = "localId")]
    pub local_id: String, // UID
    pub email: String,
    #[serde(rename = "idToken")]
    pub id_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
}

fn get_api_key() -> String {
    env::var("FIREBASE_API_KEY").expect("FIREBASE_API_KEY must be set in .env")
}

pub async fn register_user(email: &str, password: &str) -> Result<AuthResponse, Box<dyn Error>> {
    println!("Firebase Auth: Registering user {}", email);
    let api_key = get_api_key();
    let url = format!("https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}", api_key);
    let client = reqwest::Client::new();
    
    let req = AuthRequest {
        email,
        password,
        return_secure_token: true,
    };

    let res = client.post(&url).json(&req).send().await?;
    
    if res.status().is_success() {
        let auth_res = res.json::<AuthResponse>().await?;
        println!("Firebase Auth: Registration successful for UID: {}", auth_res.local_id);
        Ok(auth_res)
    } else {
        let err_text = res.text().await?;
        println!("Firebase Auth: Registration failed: {}", err_text);
        Err(format!("Auth Error: {}", err_text).into())
    }
}

pub async fn login_user(email: &str, password: &str) -> Result<AuthResponse, Box<dyn Error>> {
    println!("Firebase Auth: Logging in user {}", email);
    let api_key = get_api_key();
    let url = format!("https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}", api_key);
    let client = reqwest::Client::new();

    let req = AuthRequest {
        email,
        password,
        return_secure_token: true,
    };

    let res = client.post(&url).json(&req).send().await?;

    if res.status().is_success() {
        let auth_res = res.json::<AuthResponse>().await?;
        println!("Firebase Auth: Login successful for UID: {}", auth_res.local_id);
        Ok(auth_res)
    } else {
        let err_text = res.text().await?;
        println!("Firebase Auth: Login failed: {}", err_text);
        Err(format!("Auth Error: {}", err_text).into())
    }
}