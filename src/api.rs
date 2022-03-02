use std::env;
use std::error::Error;

use graphql_client::{GraphQLQuery, Response};
use reqwest;
use web3::types::U64;
use async_trait::async_trait;


#[derive(GraphQLQuery)]
#[graphql(
schema_path = "src/schema.json",
query_path = "src/indexed_block_query.graphql",
response_derives = "Debug",
)]
pub struct IndexedBlockNumber;

#[async_trait]
pub trait Api {
    fn get_indexed_block_num(&self) -> Result<i64, Box<dyn Error>>;
    async fn get_latest_block_num(&self) -> web3::Result<U64>;
}

pub struct BlockApi();

#[async_trait]
impl Api for BlockApi {
    fn get_indexed_block_num(&self) -> Result<i64, Box<dyn Error>> {
        let subgraph_url = &env::var("SUBGRAPH_URL").unwrap();
        let request_body = IndexedBlockNumber::build_query(indexed_block_number::Variables);
        let client = reqwest::blocking::Client::new();
        let res = client.post(subgraph_url).json(&request_body).send()?;
        let response_body: Response<indexed_block_number::ResponseData> = res.json()?;
        let block_num = response_body.data.unwrap().meta.unwrap().block.number;

        Ok(block_num)
    }

    async fn get_latest_block_num(&self) -> web3::Result<U64> {
        let mainnet_url = &env::var("MAINNET_URL").unwrap();
        let transport = web3::transports::Http::new(mainnet_url)?;
        let web3 = web3::Web3::new(transport);

        web3.eth().block_number().await
    }
}