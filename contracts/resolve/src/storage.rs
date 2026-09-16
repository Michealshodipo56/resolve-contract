use soroban_sdk::{Address, Env};

use crate::errors::Error;
use crate::types::{
    DataKey, Market, Position, INSTANCE_BUMP_THRESHOLD, INSTANCE_BUMP_TO,
    PERSISTENT_BUMP_THRESHOLD, PERSISTENT_BUMP_TO,
};

pub fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_THRESHOLD, INSTANCE_BUMP_TO);
}

pub fn next_market_id(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::NextMarketId)
        .unwrap_or(1)
}

pub fn set_next_market_id(env: &Env, id: u64) {
    env.storage().instance().set(&DataKey::NextMarketId, &id);
}

pub fn save_market(env: &Env, market: &Market) {
    let key = DataKey::Market(market.id);
    env.storage().persistent().set(&key, market);
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_THRESHOLD, PERSISTENT_BUMP_TO);
}

pub fn get_market(env: &Env, market_id: u64) -> Result<Market, Error> {
    let key = DataKey::Market(market_id);
    let market = env
        .storage()
        .persistent()
        .get(&key)
        .ok_or(Error::MarketNotFound)?;
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_THRESHOLD, PERSISTENT_BUMP_TO);
    Ok(market)
}

pub fn save_position(env: &Env, market_id: u64, user: &Address, position: &Position) {
    let key = DataKey::Position(market_id, user.clone());
    env.storage().persistent().set(&key, position);
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_THRESHOLD, PERSISTENT_BUMP_TO);
}

pub fn get_position(env: &Env, market_id: u64, user: &Address) -> Position {
    let key = DataKey::Position(market_id, user.clone());
    let position = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or_else(Position::empty);
    if env.storage().persistent().has(&key) {
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_BUMP_THRESHOLD, PERSISTENT_BUMP_TO);
    }
    position
}
