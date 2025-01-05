use std::error::Error;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Claims {
    username: String
} 

pub(crate) fn get_user_id(token: &str) -> Result<String, Box<dyn Error + Send + Sync>> {

    let parts: Vec<&str> = token.split('.').collect();

    let payload = URL_SAFE_NO_PAD.decode(parts[1])?;

    let payload_str = String::from_utf8(payload)?;

    let claims: Claims = serde_json::from_str(&payload_str)?;
    
    println!("{:?}", claims);

    Ok(claims.username)
}