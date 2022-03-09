# Lightweight healhtcheck API for "The Graph" node

This project can be used as healthcheck for [The Graph](https://github.com/graphprotocol/graph-node) node run on
local server.

## Configuration
First, ensure that `.env` file is present in main directory (check `.env.template`)

Next generate `schema.json` file for your subgraph and put it into `src`.
```
cargo install graphql_client_cli --force
graphql-client introspect-schema --output src/schema.json http://127.0.0.1:8000/subgraphs/name/<name of your subgraph>
```

## Run
```
cargo run
```

## Docker
```
docker build --tag graph-healthcheck -f Dockerfile .
docker run --env MAINNET_URL=http://host.docker.internal:8545 --env SUBGRAPH_URL=http://host.docker.internal:8000/subgraphs/name/<name of your subgraph> -p 7010:7010 graph-healthcheck
```
