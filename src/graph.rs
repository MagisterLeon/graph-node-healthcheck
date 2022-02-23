use std::env;
use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use reqwest;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "src/schema.json",
query_path = "src/indexed_block_query.graphql",
response_derives = "Debug",
)]
pub struct IndexedBlockNumber;

pub(crate) fn get_indexed_block_number() -> Result<i64, Box<dyn Error>> {
    let mainnet_url = &env::var("SUBGRAPH_URL").unwrap();
    let request_body = IndexedBlockNumber::build_query(indexed_block_number::Variables);
    let client = reqwest::blocking::Client::new();
    let res = client.post(mainnet_url).json(&request_body).send()?;
    let response_body: Response<indexed_block_number::ResponseData> = res.json()?;
    let block_num = response_body.data.unwrap().meta.unwrap().block.number;

    Ok(block_num)
}