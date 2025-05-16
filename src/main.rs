use ollama_rs::error::OllamaError;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use std::env;
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("Program started with arguments: {:?}", args);
    if args.len() < 3 {
        eprintln!(
            "Usage: {} <comma-separated-ids> <duration-in-minutes>",
            args[0]
        );
        std::process::exit(1);
    }
    let ollama = Ollama::default();
    println!("Ollama instance createdi, uri: {}", ollama.url_str());
    let res = ollama.list_local_models();
    println!("Available models: {:?}", res.await.unwrap());
    let request = GenerationRequest::new(
        "llama3.2:1b".to_string(),
        "Why is the sky blue?".to_string(),
    );

    match ollama.generate(request).await {
        Ok(response) => {
            println!("Success: {}", response.response);
            Ok(())
        }
        Err(e) => match e {
            OllamaError::InternalError(model_name) => {
                println!("Model not found: {}", model_name.message);
                Ok(())
            }
            OllamaError::JsonError(status) => {
                println!("API Error {}", status);
                Ok(())
            }
            OllamaError::ReqwestError(io_error) => {
                println!("I/O Error: {}", io_error);
                Ok(())
            }
            other_error => {
                println!("Unexpected error: {}", other_error);
                Ok(())
            }
        },
    }
}
