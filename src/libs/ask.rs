// source: src/libs/ask.rs
// src/libs/ask.rs

pub async fn run(prompt: &str, config: &MarkConfig) {
    let api_key = config.llm.api_key_from_env();

    let body = json!({
        "model": config.llm.model,
        "messages": [{ "role": "user", "content": prompt }]
    });

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .unwrap();

    let text = res.text().await.unwrap();
    println!("📥 Response:\n{text}");
}