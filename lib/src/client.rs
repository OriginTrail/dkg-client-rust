use reqwest::Client;
use std::fs::File;
use std::io::Read;

pub struct DkgClient {
    pub http_client: Client,
    pub endpoint: String,
}

impl DkgClient {
    pub async fn node_info(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let res = self
            .http_client
            .get(format!("{}api/latest/info", self.endpoint))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(res)
    }

    pub async fn trail(
        &self,
        trail_query: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let res = self
            .http_client
            .post(format!("{}api/latest/trail", self.endpoint))
            .json("")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(res)
    }

    pub async fn network_query(
        &self,
        query: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let nq_handler = self
            .http_client
            .post(format!("{}api/latest/network/query", self.endpoint))
            .json(&query)
            .send()
            .await?
            .text()
            .await?;

        println!("{}", nq_handler);

        let get_res = self
            .http_client
            .get(format!(
                "{}api/latest/network/query/responses/{}",
                self.endpoint, nq_handler
            ))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(get_res)
    }

    pub async fn publish(&self, filename: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let import_handler = self
            .http_client
            .post(format!("{}api/latest/import", self.endpoint))
            .body(contents)
            .send()
            .await?
            .text()
            .await?;

        println!("{}", import_handler);

        let get_res = self
            .http_client
            .get(format!(
                "{}api/latest/import/responses/{}",
                self.endpoint, import_handler
            ))
            .send()
            .await?
            .text()
            .await?;

        Ok(get_res)
    }
}
