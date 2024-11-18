#![allow(dead_code)]

use serde_json::Value;
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt};

// Method that reads ./config.json, looks for the "setting" key, and prints the value.
async fn read_and_parse(file_name: &str) -> String {
    let mut file = File::open(file_name).await.unwrap();
    let mut buf = Vec::new();
    file.read_buf(&mut buf).await.unwrap();
    let json = String::from_utf8(buf).unwrap();

    let config: Value = serde_json::from_str(&json).unwrap();
    config.get("setting").unwrap().to_string()
}

async fn read_and_parse_better(file_name: &str) -> Result<String, &str> {
    let mut file = File::open(file_name).await.map_err(|_| "Failed to open file")?;
    let mut buf = Vec::new();
    file.read_buf(&mut buf).await.map_err(|_| "Failed to read file")?;
    let json = String::from_utf8(buf).map_err(|_| "Failed to parse file as UTF-8")?;

    let config: Value = serde_json::from_str(&json).map_err(|_| "Failed to parse JSON")?;
    match config.get("setting") {
        Some(value) => Ok(value.to_string()),
        None => Err("Setting not found"),
    }
}

async fn read_and_parse_even_better(file_name: &str) -> anyhow::Result<String> {
    let mut file = File::open(file_name).await?;
    let mut buf = Vec::new();
    file.read_buf(&mut buf).await?;
    let json = String::from_utf8(buf)?;

    let config: Value = serde_json::from_str(&json)?;
    match config.get("setting") {
        Some(value) => Ok(value.to_string()),
        None => Err(anyhow::anyhow!("Setting not found")),
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("failed to open file")]
    Io(#[from] std::io::Error),
    #[error("failed to parse file as UTF-8")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("failed to parse JSON")]
    Json(#[from] serde_json::Error),
    #[error("other error")]
    Unknown,
}

async fn read_and_parse_best(file_name: &str) -> Result<String, ParseError> {
    let mut file = File::open(file_name).await?;
    let mut buf = Vec::new();
    file.read_buf(&mut buf).await?;
    let json = String::from_utf8(buf)?;

    let config: Value = serde_json::from_str(&json)?;
    match config.get("setting") {
        Some(value) => Ok(value.to_string()),
        None => Err(ParseError::Unknown),
    }
}

#[tokio::main]
async fn main() {
    let cs = read_and_parse("./config.json").await;
    println!("{}", cs);

    match read_and_parse_better("./config.json").await {
        Ok(value) => println!("{}", value),
        Err(e) => println!("Error: {}", e),
    }

    match read_and_parse_even_better("./config.json").await {
        Ok(value) => println!("{}", value),
        Err(e) => println!("Error: {}", e),
    }

    match read_and_parse_best("./config.json").await {
        Ok(value) => println!("{}", value),
        Err(e) => match e {
            ParseError::Io(e) => println!("IO error: {}", e),
            ParseError::Utf8(e) => println!("UTF-8 error: {}", e),
            ParseError::Json(e) => println!("JSON error: {}", e),
            ParseError::Unknown => println!("Unknown error"),
        },
    }
}
