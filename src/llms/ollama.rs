use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;

#[cfg(test)]
use async_std::task;

use super::llm_client::LlmClient;

#[derive(Clone, Debug)]
/// Tester LLM client
pub(crate) struct OllamaClient {}
/// b"{\"model\":\"codellama\",\"created_at\":\"2023-09-02T03:32:16.880889Z\",\"response\":\" for\",\"done\":false}\n"
///
pub(crate) struct OllamaConfig {
    pub(crate) api_base: String,
    pub(crate) default_model: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OllamaCompletion {
    pub(crate) model: String,
    pub(crate) created_at: String,
    pub(crate) response: String,
    pub(crate) done: bool,
}
/// {
///"model": "codellama",
///"created_at": "2023-09-02T03:34:05.724325Z",
///"done": true,
///"context": [
///    1,
///    2
///],
///"total_duration": 6977083666,
///"load_duration": 1343208,
///"sample_count": 294,
///"sample_duration": 207196000,
///"prompt_eval_count": 10,
///"prompt_eval_duration": 1482070000,
///"eval_count": 293,
///"eval_duration": 5264659000
///}
#[derive(Debug, Deserialize)]
pub(crate) struct OllamaEnding {
    pub(crate) model: String,
    pub(crate) created_at: String,
    pub(crate) done: bool,
    pub(crate) context: Vec<i32>,
    pub(crate) total_duration: i64,
    pub(crate) load_duration: i64,
    pub(crate) sample_count: i64,
    pub(crate) sample_duration: i64,
    pub(crate) prompt_eval_count: i64,
    pub(crate) prompt_eval_duration: i64,
    pub(crate) eval_count: i64,
    pub(crate) eval_duration: i64,
}

impl OllamaClient {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {})
    }
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn completions(&self, _prompt: &str) -> Result<String> {
        let mut completions: Vec<String> = Vec::new();
        let mut map = HashMap::new();
        map.insert("model", "codellama");
        map.insert("prompt", _prompt);

        let client = reqwest::Client::new();
        let mut res = client
            .post("http://localhost:11434/api/generate")
            .json(&map)
            .send()
            .await?;

        while let Some(chunk) = res.chunk().await? {
            debug!("Chunk: {:?}", chunk);
            let result: Result<OllamaCompletion, serde_json::Error> =
                serde_json::from_slice(&chunk);
            match result {
                Ok(completion) => {
                    debug!("response {:?}", completion.response);
                    completions.push(completion.response);
                }
                Err(_) => {
                    continue;
                }
            }
        }
        Ok(completions.join(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        task::block_on(async {
            let client = OllamaClient::new().unwrap();
            let result = client.completions("Hi there! ").await.unwrap();
            assert_eq!(result, "foo bar");
        });
    }
}
