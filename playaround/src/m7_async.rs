use reqwest;
use std::io::{Error, ErrorKind};
 
async fn my_async_call(url: &str) -> Result<serde_json::Value, Error> {
    
    let response = reqwest::get(url).await.map_err(|e| {
        Error::new(ErrorKind::Other, format!("Failed to fetch data: {}", e))
    })?;

    let json_response = response.json::<serde_json::Value>().await.map_err(|e| {
        Error::new(ErrorKind::Other, format!("Failed to parse JSON: {}", e))
    })?;
    
    
    Ok(json_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calls_async_function() {
        let api_url: &str = "https://jsonplaceholder.typicode.com/albums";
        let result = my_async_call(api_url).await;
        match result {
            Ok(json) => {
                println!("Received JSON: {:?}", json);
                assert!(json.is_array(), "Expected an array response");
            }
            Err(e) => {
                panic!("Error occurred: {}", e);
            }
        };
    }
}