use std::collections::hash_map::HashMap;
use log::{info, error};
use crate::cli::send::send;
use crate::cli::get::get;

use super::get::get_to_pin;

pub async fn handle_input() {
    let argv = std::env::args().collect::<Vec<String>>().into_iter().filter(|arg| arg.starts_with("--")).map(|arg| {
        let arg = arg.replace("--", "");
        let arg = arg.split("==").collect::<Vec<&str>>();
        let key = arg[0].to_string();
        let value = arg[1].split(",").map(|v| v.to_string()).collect::<Vec<String>>();
        (key, value)
    }).collect::<HashMap<String, Vec<String>>>();
    let mode = match argv.get("mode") {
        Some(mode) => mode[0].clone(),
        None => "help".to_string(),
    };
    let success = match mode.as_str() {
        "get" => handle_get(argv).await,
        "send" => handle_send(argv).await,
        "pin" => pin_to_instance(argv).await,
        _ => handle_help().await,
    };
    if success.is_err() {
        info!("Error: {}", success.err().unwrap());
    } else {
        info!("Success");
    }
}

async fn handle_help() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

async fn handle_send(argv: HashMap<String, Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    let has_input =  argv.get("input").is_some();
    let has_key = argv.get("key").is_some();
    let has_file_location = argv.get("filename").is_some();

    if !has_file_location {
        info!("No file provided. Exiting.");
        return Ok(());
    }
    
    if !has_input {
        info!("No input provided. Exiting.");
        return Ok(());
    }
    if !has_key {
        info!("No key provided. Exiting.");
        return Ok(());
    }

    let input = argv.get("input").unwrap()[0].clone();
    let key = argv.get("key").unwrap()[0].clone();
    let reps = match argv.get("reps") {
        Some(reps) => reps[0].parse::<usize>().unwrap(),
        None => {
            info!("No replicas provided, defaulting to 1");
            1
        },
    };

    let file_location = match argv.get("filename") {
        Some(file_location) => file_location[0].clone(),
        None => {
            info!("No file provided, defaulting to blank.png");
            "NONE".to_string()
        },
    };

    if file_location == "NONE".to_string() {
        error!("No file provided. Exiting.");
        return Ok(());
    }

    let shards = match argv.get("shards") {
        Some(shards) => shards[0].parse::<usize>().unwrap(),
        None => {
            info!("No shards provided, defaulting to 1");
            1
        },
    };
    send(&input, &key, reps, shards, file_location.as_str()).await
}

async fn handle_get(argv: HashMap<String, Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    let has_key = argv.get("key").is_some();
    let has_hashes = argv.get("filename").is_some();

    if !has_key {
        info!("No key provided. Exiting.");
        return Ok(());
    }

    if !has_hashes {
        info!("No hashes provided. Exiting.");
        return Ok(());
    }
    let hashes = argv.get("filename").unwrap()[0].clone();
    let key = argv.get("key").unwrap()[0].clone().to_string();
    let output = get(hashes, &key).await;
    match output {
        Ok(output) => info!("{}", output),
        Err(e) => return Err(e),
    }
    Ok(())
}

async fn pin_to_instance(argv: HashMap<String, Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    let has_file_location = argv.get("filename").is_some();
    if !has_file_location {
        info!("No file provided. Exiting.");
        return Ok(());
    }

    let file_location = match argv.get("filename") {
        Some(file_location) => file_location[0].clone(),
        None => {
            info!("No file provided, defaulting to blank.png");
            "NONE".to_string()
        },
    };

    if file_location == "NONE".to_string() {
        error!("No file provided. Exiting.");
        return Ok(());
    }

    match get_to_pin(file_location).await {
        Ok(_) => info!("Shards pinned to IPFS."),
        Err(e) => return Err(e),
    }

    Ok(())
}
