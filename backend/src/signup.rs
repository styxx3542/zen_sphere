use sqlx;
use anyhow::{anyhow, Result};
use bcrypt;
use serde::{Deserialize, Serialize};
struct Password {
    value: String,
}



type EncryptedPassword = String;
impl Password {
    fn new(value: String) -> Self {
        Self { value }
    }

    fn is_valid_length(password: &str, min_length: usize) -> bool {
        password.len() >= min_length
    }

    fn has_uppercase(password: &str) -> bool {
        password.chars().any(|c| c.is_ascii_uppercase())
    }

    fn has_lowercase(password: &str) -> bool {
        password.chars().any(|c| c.is_ascii_lowercase())
    }

    fn has_digit(password: &str) -> bool {
        password.chars().any(|c| c.is_ascii_digit())
    }

    fn has_special_character(password: &str) -> bool {
        password.chars().any(|c| !c.is_ascii_alphanumeric())
    }


    fn is_valid_password(password: &str) -> bool {
        Self::is_valid_length(password, 8) //todo - move this to a setting
            && Self::has_uppercase(password)
            &&Self::has_lowercase(password)
            && Self::has_digit(password)
            && Self::has_special_character(password)
    }
    
    fn build_password(password: String) -> Result<EncryptedPassword>{
        if !Password::is_valid_password(&password){
            return Err(anyhow!("Password doesn't match specifications"));
        }
        Ok(bcrypt::hash(password, 12).map_err(anyhow::Error::msg)?)
    }
}

#[derive(Serialize, Deserialize)]
struct Username {
    value: String,
}

impl Username {
    fn new(value: String) -> Self {
        Self { value }
    }

async fn is_username_unique(pool: &sqlx::PgPool, username: &str) -> Result<bool> {
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE username = $1", username)
        .fetch_one(pool)
        .await.map_err(anyhow::Error::msg)?;

    Ok(count == Some(0))
}
    async fn validate(&self, pool: &sqlx::PgPool) -> Result<bool>{
        Ok(Self::is_username_unique(&pool, &self.value).await?)
    }
}

fn validate_email(email:&str) -> bool {
    //todo - implement this
    true
}
#[derive(FromForm)]
pub struct SignupForm {
    pub username: String,
    pub password: String,
    pub email: String,
}

pub async fn signup(username: &str, password: &str,email: &str, pool: &sqlx::PgPool) -> Result<()> {
    let username = Username::new(username.to_string());
    if !username.validate(pool).await? {
        return Err(anyhow!("Username is not unique"));
    }
    if !validate_email(email){
        return Err(anyhow!("Email is not valid"));
    }
    let password_hash = Password::build_password(password.to_string())?;
    sqlx::query!(
        "INSERT INTO users (username, password_hash, email) VALUES ($1, $2, $3)",
        username.value,
        password_hash,
        email
    ).execute(pool).await.map_err(anyhow::Error::msg)?;
    Ok(()) 
}



