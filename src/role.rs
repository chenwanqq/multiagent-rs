use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::action::Action;

pub enum Mode {
    REACT,
    PlanAndAct,
    ByOrder,
}

#[derive(Serialize, Deserialize)]
struct ActionDescription {
    index: usize,
    name: String,
    description: String,
}

#[async_trait]
pub trait Role {
    /// This is the name of the role
    fn name(&self) -> String;
    /// This is the description of the role
    fn profile(&self) -> String;
    /// This is the mode of the role
    fn mode(&self) -> Mode {
        Mode::REACT
    }
    /// This is the list of actions that the role can perform
    fn get_actions(&self) -> Vec<Box<dyn Action>>;

    /// main entry of a role
    async fn run(&self) -> String {
        match self.mode() {
            Mode::REACT => self._react().await,
            Mode::PlanAndAct => todo!(),
            Mode::ByOrder => todo!(),
        }
    }
    /// get the description of the actions
    fn _get_actions_description(&self, actions: &Vec<Box<dyn Action>>) -> String {
        let descriptions: Vec<ActionDescription> = actions
            .iter()
            .enumerate()
            .map(|(index, action)| ActionDescription {
                index: index,
                name: action.name(),
                description: action.desc(),
            })
            .collect();
        serde_json::to_string(&descriptions).unwrap()
    }

    /// do an action
    async fn _act(&self, instruction: &str, action: &Box<dyn Action>) -> String {
        action.run(instruction).await.unwrap()
    }

    /// basic steps for react
    /// think: select an action, return the index of the action
    async fn _think(&self) -> usize;
    async fn _observe(&self) -> String;
    async fn _react(&self) -> String;
    /*
    TODO: implement these functions
    /// basic steps for plan and act
    async fn _plan(&self) -> String;
    async fn _plan_and_act(&self) -> String;
    /// basic steps for by order
    async fn _by_order(&self) -> String;
    */
}
