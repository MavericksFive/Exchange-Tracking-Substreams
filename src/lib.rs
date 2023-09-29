mod abi;
mod pb;
mod utils;

use pb::eth::stable_swap::v1 as StableSwap;
use substreams::pb::substreams::store_delta::Operation;
use substreams::prelude::*;
use substreams::{store::{StoreSetProto, Deltas}, Hex};
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;
use utils::math::to_big_decimal;

use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

use utils::constants::{CONTRACT_ADDRESS, START_BLOCK};
use utils::helpers::{append_0x, generate_id};
substreams_ethereum::init!();

/// Extracts token exchanges events from the pool contract
#[substreams::handlers::map]
fn map_exchanges(blk: eth::Block) -> Result<Option<StableSwap::Exchanges>, substreams::errors::Error> {
    let exchanges: Vec<_> = blk
        .events::<abi::stable_swap::events::TokenExchange>(&[&CONTRACT_ADDRESS])
        .map(|(exchange, log)| {
            substreams::log::info!("Token Exchange seen");

            StableSwap::Exchange {
                buyer: Some(StableSwap::Account {
                    address: append_0x(Hex::encode(&exchange.buyer).to_string().as_str()),}),
                sold_id: to_big_decimal(&exchange.sold_id.to_string().as_str())
                .unwrap()
                .to_string(),
                tokens_sold: to_big_decimal(&exchange.tokens_sold.to_string().as_str())
                .unwrap()
                .to_string(),
                bought_id: to_big_decimal(&exchange.bought_id.to_string().as_str())
                .unwrap()
                .to_string(),
                tokens_bought: to_big_decimal(&exchange.tokens_bought.to_string().as_str())
                .unwrap()
                .to_string(),
                trx_hash: Hex::encode(&log.receipt.transaction.hash),
                block_number: blk.number,
                timestamp: blk.timestamp_seconds(),
                log_index: log.index()
            }
        })
        .collect();
    if exchanges.len() == 0 {
        return Ok(None);
    }

    Ok(Some(StableSwap::Exchanges { exchanges }))
}

const NULL_ADDRESS: &str = "0000000000000000000000000000000000000000";

/// Store the total balance of NFT tokens for the specific CONTRACT_ADDRESS by holder
#[substreams::handlers::store]
fn store_pool(block: eth::Block, o: StoreSetProto<StableSwap::Pool>) {
    if block.number == START_BLOCK {
        let pool = &StableSwap::Pool {
            name: "sBTC Swap".to_string(),
            pool_address: "0x7fC77b5c7614E1533320Ea6DDc2Eb61fa00A9714".to_string(),
            address_token_one: "0xEB4C2781e4ebA804CE9a9803C67d0893436bB27D".to_string(),
            address_token_two: "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599".to_string(),
            address_token_three: "0xfE18be6b3Bd88A2D2A7f928d00292E7a9963CfC6".to_string(),
        };
        o.set(0, format!("Pool: {}", pool.pool_address), &pool);
    };
}

#[substreams::handlers::map]
fn graph_out(
    exchanges: StableSwap::Exchanges,
    pools: Deltas<DeltaProto<StableSwap::Pool>>,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    for exchange in exchanges.exchanges {
        let buyer_address = exchange.buyer.unwrap().address.clone();
        tables
            .create_row(
                "Exchange",
                format!("{}", &exchange.trx_hash),
            )
            .set("buyer", &buyer_address)
            .set("sold_id", &exchange.sold_id)
            .set("tokens_sold", &exchange.tokens_sold)
            .set("bought_id", &exchange.bought_id)
            .set("tokens_bought", &exchange.tokens_bought)
            .set("trx_hash", &exchange.trx_hash)
            .set("timestamp", exchange.timestamp)
            .set("block_number", exchange.block_number)
            .set("log_index", exchange.log_index)
            .set("pool", append_0x(Hex::encode(CONTRACT_ADDRESS).to_string().as_str()));

            //handles accounts
            tables.create_row("Account", &buyer_address);

    }

    for delta in pools.deltas {
        match delta.operation {
            Operation::Create => {
                let pool_row = tables.create_row("Pool", &delta.new_value.pool_address);
                pool_row.set("name", delta.new_value.name);
                pool_row.set("pool_address", delta.new_value.pool_address);
                pool_row.set("address_token_one", delta.new_value.address_token_one);
                pool_row.set("address_token_two", delta.new_value.address_token_two);
                pool_row.set("address_token_three", delta.new_value.address_token_three);
            }
            Operation::Update => todo!(),
            Operation::Delete => todo!(),
            x => panic!("unsupported operation {:?}", x),
        };
    }

    Ok(tables.to_entity_changes())
}
