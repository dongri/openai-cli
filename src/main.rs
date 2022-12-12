use openai_api_rs::v1::completion::{self, CompletionRequest};
use openai_api_rs::v1::api::Client;
use std::env;
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "exit" || input == "quit" || input == "bye"{
            break;
        }
        print!("");
        let req = CompletionRequest {
            model: completion::GPT3_TEXT_DAVINCI_003.to_string(),
            prompt: Some(input.to_string()),
            suffix: None,
            max_tokens: Some(4000),
            temperature: Some(0.3),
            top_p: None,
            n: Some(1),
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            best_of: None,
            logit_bias: None,
            user: None,
          };

        let result = client.completion(req).await?;
        println!("> OpenAI 🤖{:}", result.choices[0].text);
    }
    Ok(())
}
