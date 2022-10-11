use std::io::Read;
use std::fs::File;
use std::error::Error;
use std::env;

use serde::{Deserialize, Serialize};

fn get_api_endpoint() -> Option<String> {
    let mut temp_dir = env::temp_dir();
    temp_dir.push("com.github.antony-jr.spirit");
    let port_file = match temp_dir.as_path().to_str() {
        Some(p) => p,
        _ => ""
    };

    if port_file.len() == 0 {
        return None;
    }

    let file = File::open(port_file);
    if file.is_err() {
        return None;
    }
    let mut file = file.unwrap();

    let mut contents = String::new();
    let result = file.read_to_string(&mut contents);
    if result.is_err() {
        return None;
    }

    let port: u16 = match contents.trim().parse() {
        Ok(n) => n,
        _ => 0
    };

    if port <= 1024 {
        return None;
    }

    Some(format!("http://127.1:{}/spirit/v1/", port))
}

fn get_config_file() -> Option<String> {
    let home = home::home_dir();
    if home.is_none() {
        return None;
    }
    let mut home = home.unwrap();
    home.push(".spirit/");
    home.push("bash_feedback.json");

    let path = home.as_path();
    if !path.exists() {
        return None;
    }
    
    let config_path = match path.to_str() {
        Some(p) => p,
        _ => ""
    };

    if config_path.len() == 0 {
        return None;
    }

    Some(format!("{}", config_path))
}

#[derive(Deserialize, Debug)]
struct Config {
    error: String,
    nonerror: String
}

impl Config {
    fn new() -> Config {
        match get_config_file() { 
            Some(path) => {
                 let mut contents = String::new();
                 let file = File::open(path);
                 if file.is_err() {
                     return Config { 
                         error: String::from("error"),
                         nonerror: String::from("default")
                     }; 
                 }

                 let mut file = file.unwrap(); 
                 let result = file.read_to_string(&mut contents);
                 if result.is_err() {
                     return Config { 
                         error: String::from("error"),
                         nonerror: String::from("default")
                     };
                 }

                 match serde_json::from_str(contents.as_str()) {
                     Ok(config) => config,
                     _ => Config {
                         error: String::from("error"),
                         nonerror: String::from("default")
                     }
                 }
            },
            _ => Config {
                error: String::from("error"),
                nonerror: String::from("default")
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Action {
    opt: String,
    action: String
}

#[derive(Deserialize, Serialize)]
struct Response {
    status: String,
    action: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect(); 
    if args.len() == 1 {
        return Ok(());
    }

    let api_url = get_api_endpoint();
    if api_url.is_none() {
        return Ok(());
    }

    let api_url = api_url.unwrap(); 
    let action_api = format!("{}action", api_url);
    let client = reqwest::Client::new();
    let react = &args[1];
    
    // Get the action in question first
    // then, let's see if we have to give a request
    let response = client.get(action_api.as_str())
                         .send()
                         .await;

    if response.is_err() {
        return Ok(());
    }
    let response = response.unwrap();
    let status = response.json::<Response>().await;

    if status.is_err() {
        return Ok(());
    }
    let status = status.unwrap();

    let config = Config::new();
   
    let mut json_body = Action {
        opt: String::from("set"),
        action: config.nonerror.clone() 
    };

    if react == "error" {
        json_body.action = config.error.clone();
    }

    if status.action == json_body.action {
        return Ok(());
    }

    let response = client.post(action_api.as_str())
                         .json(&json_body)
                         .send()
                         .await;

    if response.is_err() {
        return Ok(());
    }
   
    Ok(())
}
