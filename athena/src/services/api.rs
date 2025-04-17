use anyhow::Result;
use reqwest::StatusCode;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use talaria::api::*;
use talaria::console::*;

#[derive(Clone)]
pub struct Api {
    api_base: Url,
    client: Client,
    token: String,
}

impl Api {
    pub fn new(api_base: &str, token: &str) -> Api {
        Api {
            api_base: Url::parse(api_base).unwrap_or(Url::parse("http://localhost:8000").unwrap()),
            client: Client::new(),
            token: token.to_string(),
        }
    }

    pub fn set_token(&mut self, api_token: &str) {
        self.token = api_token.to_string();
    }

    pub fn set_api_base(&mut self, api_base: &str) -> bool {
        match Url::parse(api_base) {
            Ok(url) => {
                self.api_base = url;
                true
            }
            Err(_) => false,
        }
    }

    fn make_api_path(&self, endpoint: &str) -> Result<Url> {
        Ok(self.api_base.join(&format!("/api/admin{}", endpoint))?)
    }

    async fn get<T>(&self, endpoint: &str, query_params: Vec<(&str, &str)>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        Ok(self
            .client
            .get(self.make_api_path(endpoint)?)
            .query(&query_params)
            .header("Authorization", &self.token)
            .send()
            .await?
            .json::<T>()
            .await?)
    }

    async fn post<O, I>(&self, endpoint: &str, data: I) -> Result<O>
    where
        I: Serialize,
        O: for<'de> Deserialize<'de>,
    {
        Ok(self
            .client
            .post(self.make_api_path(endpoint)?)
            .json(&data)
            .header("Authorization", &self.token)
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub async fn list_agents(&self) -> Result<Vec<AgentInfo>> {
        Ok(self.get("/list_agents", vec![]).await?)
    }

    pub async fn get_tartarus_info(&self) -> Result<TartarusInfo> {
        Ok(self.get("/tartarus_info", vec![]).await?)
    }

    pub async fn get_tartarus_stats(&self) -> Result<TartarusStats> {
        Ok(self.get("/tartarus_stats", vec![]).await?)
    }

    pub async fn console(
        &self,
        command_context: CommandContext,
    ) -> Result<Result<ConsoleResponse, ConsoleError>> {
        Ok(self.post("/console/monolith", command_context).await?)
    }

    pub async fn check_host(&self, host: &str) -> bool {
        let url = match Url::parse(host) {
            Ok(url) => url,
            Err(_) => return false,
        };

        let path = url
            .join(&format!("/api/admin{}", "/tartarus_info"))
            .unwrap_or(Url::parse("http://0.0.0.0").unwrap());

        match self.client.get(path).send().await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn check_auth(&self, host: &str, api_token: &str) -> bool {
        let url = match Url::parse(host) {
            Ok(url) => url,
            Err(_) => return false,
        };

        match self
            .client
            .get(
                self.make_api_path("/tartarus_info")
                    .unwrap_or(Url::parse("http://0.0.0.0").unwrap()),
            )
            .header("Authorization", api_token)
            .send()
            .await
        {
            Ok(response) => match response.status() {
                StatusCode::OK => return true,
                _ => return false,
            },
            Err(_) => {
                return false;
            }
        }
    }
}
