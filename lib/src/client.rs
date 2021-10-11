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
        let res = self
            .http_client
            .get(format!("{}api/latest/info", self.endpoint))
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
        //println!("{}", query);

        let nq_handler = self
            .http_client
            .post(format!("{}api/latest/network/query", self.endpoint))
            .body(query)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<QueryHandlerStruct>() //.json::<serde_json::Value>()
            .await?;

        println!("Handler id for query: {}", nq_handler.query_id);
        thread::sleep(time::Duration::from_millis(1000));

        let get_res = self
            .http_client
            .get(format!(
                "{}api/latest/network/query/responses/{}",
                self.endpoint, nq_handler.query_id
            ))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        println!("{}", get_res);

        Ok(get_res)
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
            .json::<HandlerStruct>() //.json::<serde_json::Value>()
            .await?;

        println!(
            "Hendler ID for import request is: {}",
            import_handler.handler_id
        );

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

        thread::sleep(time::Duration::from_millis(1000));

        let dataset_id: String =
            String::from(import_response["data"]["dataset_id"].as_str().unwrap());
        println!("Imported dataset id is :{}", &dataset_id);

        println!("Starting replication!");

        let replication_body = format!("{{\"dataset_id\":\"{}\"}}", dataset_id);
        println!("Replication body: {}", replication_body);

        let replication_handler = self
            .http_client
            .post(format!("{}api/latest/replicate", self.endpoint))
            .body(replication_body)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<HandlerStruct>() //.json::<serde_json::Value>()
            .await?;

        println!(
            "Hendler ID for replicate request is: {}",
            replication_handler.handler_id
        );

        thread::sleep(time::Duration::from_millis(1000));

        let get_res = self
            .http_client
            .get(format!(
                "{}api/latest/replicate/result/{}",
                self.endpoint, replication_handler.handler_id
            ))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        println!("{}", get_res);

        Ok(get_res)
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

        println!("Trail response: {}", trail);
        thread::sleep(time::Duration::from_millis(1000));

        Ok(trail)
    }
}
