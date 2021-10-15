use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::{thread, time};

#[derive(Debug, Serialize, Deserialize)]
struct HandlerStruct {
    handler_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryHandlerStruct {
    query_id: String,
}

pub struct DkgClient {
    pub http_client: Client,
    pub endpoint: String,
}

impl DkgClient {
    pub async fn node_info(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let node_infos = self
            .http_client
            .get(format!("{}api/latest/info", self.endpoint))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(node_infos)
    }

    pub async fn network_query(
        &self,
        query: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let query_handler = self
            .http_client
            .post(format!("{}api/latest/network/query", self.endpoint))
            .body(query)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<QueryHandlerStruct>()
            .await?;

        thread::sleep(time::Duration::from_millis(1000));

        let query_response = self
            .http_client
            .get(format!(
                "{}api/latest/network/query/responses/{}",
                self.endpoint, query_handler.query_id
            ))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(query_response)
    }

    pub async fn publish(
        &self,
        filename: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut form_params = HashMap::new();
        form_params.insert("file".to_string(), contents);
        form_params.insert("standard_id".to_string(), "GRAPH".to_string());

        let import_handler = self
            .http_client
            .post(format!("{}api/latest/import", self.endpoint))
            .form(&form_params)
            .send()
            .await?
            .json::<HandlerStruct>()
            .await?;

        thread::sleep(time::Duration::from_millis(1000));

        let import_response = self
            .http_client
            .get(format!(
                "{}api/latest/import/result/{}",
                self.endpoint, import_handler.handler_id
            ))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        println!("Waiting for data to be imported...");
        thread::sleep(time::Duration::from_millis(10000));

        let dataset_id: String =
            String::from(import_response["data"]["dataset_id"].as_str().unwrap());
        let replication_body = format!("{{\"dataset_id\":\"{}\"}}", dataset_id);

        let replication_handler = self
            .http_client
            .post(format!("{}api/latest/replicate", self.endpoint))
            .body(replication_body)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<HandlerStruct>()
            .await?;

        thread::sleep(time::Duration::from_millis(1000));

        let replication_result = self
            .http_client
            .get(format!(
                "{}api/latest/replicate/result/{}",
                self.endpoint, replication_handler.handler_id
            ))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(replication_result)
    }

    pub async fn trail(
        &self,
        query: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let trail = self
            .http_client
            .post(format!("{}api/latest/trail", self.endpoint))
            .body(query)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<serde_json::Value>() //.json::<serde_json::Value>()
            .await?;

        thread::sleep(time::Duration::from_millis(1000));

        Ok(trail)
    }

    pub async fn export(
        &self,
        dataset_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut form_params = HashMap::new();
        form_params.insert("dataset_id".to_string(), dataset_id);
        form_params.insert("standard_id".to_string(), "GRAPH".to_string());

        let export_handler = self
            .http_client
            .post(format!("{}api/latest/export", self.endpoint))
            .form(&form_params)
            .send()
            .await?
            .json::<HandlerStruct>()
            .await?;

        thread::sleep(time::Duration::from_millis(1000));

        let query_response = self
            .http_client
            .get(format!(
                "{}api/latest/export/result/{}",
                self.endpoint, export_handler.handler_id
            ))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(query_response)
    }
}
