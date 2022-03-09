use std::env;
use std::error::Error;

use graphql_client::{GraphQLQuery, Response};
use reqwest;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "src/schema.json",
query_path = "src/indexed_block_query.graphql",
response_derives = "Debug",
)]
pub struct IndexedBlockNumber;

pub struct ApiFacade;

impl ApiFacade {
    pub fn get_not_indexed_block_count() -> Result<i64, Box<dyn Error>> {
        let indexed_block = ApiFacade::get_indexed_block_number()?;
        let latest_block = ApiFacade::get_latest_block_number()?;
        Ok(latest_block - indexed_block)
    }

    pub fn get_indexed_block_number() -> Result<i64, Box<dyn Error>> {
        let indexed_block = get_indexed_block_num()?;
        println!("Indexed block: {}", &indexed_block);
        Ok(indexed_block)
    }

    pub fn get_latest_block_number() -> Result<i64, Box<dyn Error>> {
        let latest_block = get_latest_block_num()?;
        println!("Latest block: {}", &latest_block);
        Ok(latest_block)
    }
}


fn get_indexed_block_num() -> Result<i64, Box<dyn Error>> {
    let subgraph_url = &env::var("SUBGRAPH_URL").unwrap();
    let request_body = IndexedBlockNumber::build_query(indexed_block_number::Variables);
    let client = reqwest::blocking::Client::new();
    let res = client.post(subgraph_url).json(&request_body).send()?;
    let response_body: Response<indexed_block_number::ResponseData> = res.json()?;
    let block_num = response_body.data.unwrap().meta.unwrap().block.number;

    Ok(block_num)
}

#[tokio::main]
async fn get_latest_block_num() -> Result<i64, Box<dyn Error>> {
    let mainnet_url = &env::var("MAINNET_URL").unwrap();
    let transport = web3::transports::Http::new(mainnet_url)?;
    let web3 = web3::Web3::new(transport);
    let block_num = web3.eth().block_number().await?;

    Ok(i64::try_from(block_num).unwrap())
}
