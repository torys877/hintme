use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use std::env;

struct Config {
    api_key: String,
    api_url: String,
    model: String,
    seed: i128,
    system_content: String,
    args: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenaiRequest {
    model: String,
    seed: i128,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct OpenaiResponse {
    choices: Vec<OpenaiResultChoice>,
}

#[derive(Deserialize)]
struct OpenaiResultChoice {
    message: Message,
}

#[tokio::main]
async fn main() {
    let config = get_config();
    let response = send_request(config).await;
    let response: OpenaiResponse = response.unwrap().json().await.unwrap();
    println!("{}", response.choices.first().unwrap().message.content);
}

fn get_config() -> Config {
    dotenv::dotenv().ok();
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);

    Config {
        api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in env variables"),
        api_url: env::var("OPENAI_API_URl").expect("OPENAI_API_URl url must be set"),
        model: env::var("OPENAI_MODEL").unwrap_or("gpt-3.5-turbo".to_string()),
        seed: env::var("OPENAI_SEED")
            .unwrap_or("777".to_string())
            .parse::<i128>()
            .unwrap_or(777),
        system_content: env::var("OPENAI_SYSTEM_MESSAGE").unwrap_or(
            "Behave as linux terminal. Answer with commands I ask about. Output string I can copy and paste in my terminal. No surrounding symbols. No formatting."
                .to_string(),
        ),
        args,
    }
}

async fn send_request(config: Config) -> Result<Response, Error> {
    // Create correct authentification token
    let auth_token = format!("Bearer {}", config.api_key);

    // Add headers for authorization and content type we are working with
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&auth_token).expect("Invalid header value"),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Create body request
    let body_request = OpenaiRequest {
        model: config.model, //set openai model
        seed: config.seed,   // set seed parameter
        messages: vec![
            Message {
                role: "system".to_string(),
                content: config.system_content,
            },
            Message {
                role: "user".to_string(),
                content: config.args.join(" "),
            },
        ],
    };

    // Create client, send request and return response
    let client = reqwest::Client::new();
    client
        .post(config.api_url)
        .headers(headers)
        .json(&body_request)
        .send()
        .await
}
