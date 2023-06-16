use crate::actions::{ActionProvider, SwarmAction};
use async_trait::async_trait;
use manifest::Variables;
use std::time::Duration;

pub struct ForEachAction {
    action_provider: ActionProvider,
    name: String,
    for_: String,
    in_: String,
    do_: String,
    sleep: Option<Duration>,
}

impl ForEachAction {
    pub(crate) fn new(
        action_provider: ActionProvider,
        name: String,
        for_: String,
        in_: String,
        do_: String,
        sleep: Option<Duration>,
    ) -> Self {
        Self {
            action_provider,
            name,
            for_,
            in_,
            do_,
            sleep,
        }
    }
}

#[async_trait]
impl SwarmAction for ForEachAction {
    async fn execute(&mut self, vars: &Variables) -> anyhow::Result<()> {
        let value = vars.get_value(&self.in_).unwrap();
        let arr = value.as_array().unwrap();
        let mut action = self.action_provider.get_action(&self.do_).unwrap();

        for item in arr {
            let mut vars = vars.clone();
            vars.set(&self.for_, item.clone());
            action.execute(&vars).await?;
            if let Some(sleep) = self.sleep {
                tokio::time::sleep(sleep).await;
            }
        }

        Ok(())
    }
}
