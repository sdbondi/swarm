use async_trait::async_trait;
use manifest::Variables;

mod json_rpc;

mod provider;
pub use provider::*;

#[async_trait]
pub trait SwarmAction {
    async fn execute(&mut self, vars: &Variables) -> anyhow::Result<()>;
}
