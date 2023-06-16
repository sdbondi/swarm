use crate::actions::for_each::ForEachAction;
use crate::actions::{fs_rm, json_rpc};
use manifest::{Action, SwarmAction};

#[derive(Debug, Clone)]
pub struct ActionProvider {
    actions: Vec<SwarmAction>,
}

impl ActionProvider {
    pub(crate) fn new(actions: Vec<SwarmAction>) -> Self {
        Self { actions }
    }

    pub fn get_action(
        &self,
        name: &str,
    ) -> Option<Box<dyn crate::actions::SwarmAction + Send + Sync + 'static>> {
        let config = self.actions.iter().find(|action| action.name == name)?;
        match &config.action {
            Action::JsonRpc {
                url,
                method,
                params,
            } => Some(Box::new(json_rpc::JsonRpcAction::new(
                config.name.clone(),
                url.clone(),
                method.clone(),
                params.clone(),
            ))),
            Action::FsRm { path, force } => Some(Box::new(fs_rm::FsRmAction {
                path: path.clone(),
                force: *force,
            })),
            Action::ForEach {
                for_,
                in_,
                do_,
                pause,
            } => Some(Box::new(ForEachAction::new(
                self.clone(),
                config.name.clone(),
                for_.clone(),
                in_.clone(),
                do_.clone(),
                *pause,
            ))),
        }
    }
}
