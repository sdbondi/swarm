use crate::actions::SwarmAction;
use async_trait::async_trait;
use manifest::Variables;
use serde_json as json;
use serde_json::json;

pub(crate) fn instantiate_action(
    name: String,
    url: String,
    method: String,
    params: Option<json::Value>,
) -> JsonRpcAction {
    JsonRpcAction {
        request_id: 1,
        name,
        url,
        method,
        params,
    }
}

pub struct JsonRpcAction {
    request_id: usize,
    name: String,
    url: String,
    method: String,
    params: Option<json::Value>,
}

impl JsonRpcAction {
    fn next_request_id(&mut self) -> usize {
        let id = self.request_id;
        self.request_id += 1;
        id
    }
}

#[async_trait]
impl SwarmAction for JsonRpcAction {
    async fn execute(&mut self, vars: &Variables) -> anyhow::Result<()> {
        let client = reqwest::Client::builder().build()?;

        let params = self.params.as_ref().map(|p| substitute_params(p, vars));
        let request_json = json!(
            {
                "jsonrpc": "2.0",
                "id": self.next_request_id(),
                "method": vars.substitute(&self.method),
                "params": params,
            }
        );
        println!("[{}] {}: {}", self.name, self.method, request_json);
        let builder = client.post(vars.substitute(&self.url));
        // TODO: headers
        // if let Some(header) = &self.headers {
        // builder = builder.header(AUTHORIZATION, format!("Bearer {}", token));
        // }
        let resp = builder.json(&request_json).send().await?;
        let response: json::Value = resp.json().await?;

        println!("[{}] {}: {:?}", self.name, self.method, response);
        log::info!("[{}] {}: {:?}", self.name, self.method, response);
        Ok(())
    }
}

fn substitute_params(params: &json::Value, vars: &Variables) -> json::Value {
    match params {
        json::Value::Object(map) => {
            let mut new_map = json::Map::new();
            for (k, v) in map {
                new_map.insert(k.clone(), substitute_params(v, vars));
            }
            json::Value::Object(new_map)
        }
        json::Value::Array(arr) => {
            let mut new_arr = Vec::with_capacity(arr.len());
            for v in arr {
                new_arr.push(substitute_params(v, vars));
            }
            json::Value::Array(new_arr)
        }
        json::Value::String(s) => json::Value::String(vars.substitute(s)),
        _ => params.clone(),
    }
}
