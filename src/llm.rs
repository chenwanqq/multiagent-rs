use anyhow::Result;
use async_trait::async_trait;
use erniebot_rs::chat::ChatEndpoint;
use lazy_static::lazy_static;

#[async_trait]
pub trait LLM: Send {
    async fn ask(&self, prompt: &str) -> Result<String>;
}

pub struct ErnieLLM {
    chat_endpoint: ChatEndpoint,
}

impl ErnieLLM {
    pub fn new(chat_endpoint: ChatEndpoint) -> Self {
        Self { chat_endpoint }
    }
}

#[async_trait]
impl LLM for ErnieLLM {
    async fn ask(&self, prompt: &str) -> Result<String> {
        let options = Vec::new();
        let message = vec![erniebot_rs::chat::Message {
            role: erniebot_rs::chat::Role::User,
            content: prompt.to_string(),
            ..Default::default()
        }];
        let response = self.chat_endpoint.ainvoke(&message, &options).await?;
        Ok(response.get_chat_result()?)
    }
}

pub fn get_default_llm() -> Box<dyn LLM> {
    lazy_static! {
        static ref CHAT_ENDPOINT: ChatEndpoint =
            ChatEndpoint::new(erniebot_rs::chat::ChatModel::ErnieBotTurbo).unwrap();
    }
    Box::new(ErnieLLM::new(CHAT_ENDPOINT.clone()))
}
