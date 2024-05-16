use anyhow::Result;
use async_trait::async_trait;
use multiagent_rs::action::Action;

fn parse_code(result: &str) -> Option<String> {
    let mut start_line: i32 = -1;
    let mut end_line: i32 = -1;
    for (cnt, line) in result.lines().enumerate() {
        if line.starts_with("```python") && start_line == -1 {
            start_line = cnt as i32;
        } else if line.starts_with("```") {
            end_line = cnt as i32;
            break;
        }
    }
    if start_line == -1 || end_line == -1 {
        return None;
    }
    let result = result
        .lines()
        .skip((start_line+1) as usize)
        .take((end_line - start_line-1) as usize)
        .collect::<Vec<&str>>()
        .join("\n");
    Some(result)
}

struct SimpleWriteCode {}

impl SimpleWriteCode {
    const PROMPT_TEMPLATE: &'static str = "
    Write a python function that can {instruction}.
    Return ```python your_code_here ``` with NO other texts,
    your code:
    ";
}

#[async_trait]
impl Action for SimpleWriteCode {
    async fn run(&self, instruction: &str) -> Result<String> {
        let prompt = SimpleWriteCode::PROMPT_TEMPLATE.replace("{instruction}", instruction);
        let response = self.ask(&prompt).await?;
        if let Some(code) = parse_code(&response) {
            Ok(code)
        } else {
            Err(anyhow::anyhow!("Failed to parse code"))
        }
    }

    fn name(&self) -> String {
        "SimpleWriteCode".to_string()
    }

    fn desc(&self) -> String {
        "generate python code".to_string()
    }
}

#[tokio::main]
async fn main() {
    let action = SimpleWriteCode {};
    let instruction = "write a function that can add two numbers";
    let code = action.run(instruction).await.unwrap();
    println!("-----");
    println!("{}", code);
}
