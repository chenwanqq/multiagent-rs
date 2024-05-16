use crate::llm::{get_default_llm, LLM};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Action: Send + Sync {
    fn llm(&self) -> Box<dyn LLM> {
        get_default_llm()
    }
    /// use llm to ask a question
    async fn ask(&self, prompt: &str) -> Result<String> {
        let llm = self.llm();
        let response = llm.ask(prompt).await;
        println!("{:?}", response);
        response
    }
    /// main entry of an action
    async fn run(&self, instruction: &str) -> Result<String>;
    /// This is the name of the action
    fn name(&self) -> String;
    /// This is the description of the action
    fn desc(&self) -> String;
}
