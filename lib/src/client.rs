use std::collections::HashMap;
pub fn resolve(endpoint: &str, uri: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(endpoint)?
            .json::<HashMap<String, String>>()?;
    Ok(resp)
}

pub fn discover(endpoint: &str, topics: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>  {
    let resp = reqwest::blocking::get(endpoint)?
                .json::<HashMap<String, String>>()?;
        Ok(resp)
}

pub fn query(endpoint: &str, topics: &str, query: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>  {
    let resp = reqwest::blocking::get(endpoint)?
                .json::<HashMap<String, String>>()?;
        Ok(resp)
}


fn main() {
    let resp = resolve("https://mainnet.ot-node.com/", "id").unwrap();
    println!("{:#?}", resp);

    let resp = discover("https://mainnet.ot-node.com/", "topics").unwrap();
    println!("{:#?}", resp);

    let resp = query("https://mainnet.ot-node.com/", "topics", "query").unwrap();
    println!("{:#?}", resp);
}
