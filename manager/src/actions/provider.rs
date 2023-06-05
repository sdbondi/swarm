use crate::actions::json_rpc;
use manifest::{Action, SwarmAction};

pub struct ActionProvider {
    actions: Vec<SwarmAction>,
}

impl ActionProvider {
    pub(crate) fn new(actions: Vec<SwarmAction>) -> Self {
        Self { actions }
    }

    pub fn get_action(&self, name: &str) -> Option<impl crate::actions::SwarmAction> {
        let config = self.actions.iter().find(|action| action.name == name)?;
        match &config.action {
            Action::JsonRpc {
                url,
                method,
                params,
            } => Some(json_rpc::instantiate_action(
                config.name.clone(),
                url.clone(),
                method.clone(),
                params.clone(),
            )),
            Action::FsRm { .. } => unimplemented!(),
        }
    }
}
