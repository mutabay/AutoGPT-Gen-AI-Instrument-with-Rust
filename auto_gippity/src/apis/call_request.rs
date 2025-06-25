use crate::helpers::cli;
use crate::models::general::llm::{ChatCompletion, Message, APIResponse};
use dotenv::dotenv;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

// Call LLM (i.e. GPT-4)
pub async fn call_llm(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in .env file");
    let model = env::var("OPENAI_MODEL").expect("OPENAI_MODEL must be set in .env file");
    let api_org = env::var("OPENAI_ORG").expect("OPENAI_ORG must be set in .env file");
    let base_url =
        env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

    let url: String = format!("{}/chat/completions", base_url);

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create Open AI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client
    let client: Client = Client::builder().default_headers(headers).build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: model,
        messages,
        temperature: 0.1,
    };

    // // Troubleshooting
    // let res_raw = client
    //     .post(&url)
    //     .json(&chat_completion)
    //     .send()
    //     .await?;

    // Extract API Response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Send Response
    Ok(res.choices[0].message.content.clone())
}




#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_llm() {
        // Prepare a sample message
        let messages = vec![Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response.".to_string(),
        }];

        // Call the function (requires valid .env and API key)
        let result: Result<String, Box<dyn std::error::Error + Send>> = call_llm(messages).await;
        match result {
            Ok(response) => {
                println!("Response: {}", response);
                assert!(!response.is_empty(), "Response should not be empty");
            }
            Err(e) => {
                eprintln!("Error calling LLM: {}", e);
                panic!("Failed to call LLM: {}", e);
            }
        }
    }
}
