use cosmwasm_schema::cw_serde;

use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub connection_id: String,
    pub port_id: String,
    pub update_period: u64,
    pub core_address: String,
    pub gov_helper_address: String,
}

pub const PROPOSALS_VOTES_REPLY_ID: u64 = 1;
pub const PROPOSALS_VOTES_REMOVE_REPLY_ID: u64 = 2;

pub const QUERY_ID: Item<u64> = Item::new("query_id");

pub const CONFIG: Item<Config> = Item::new("config");
pub const ACTIVE_PROPOSALS: Item<Vec<u64>> = Item::new("active_proposals");
pub const VOTERS: Item<Vec<String>> = Item::new("voters");
