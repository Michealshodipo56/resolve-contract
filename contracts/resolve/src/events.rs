use soroban_sdk::{contractevent, contracttype, Address};

use crate::types::{Outcome, Side};

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClaimKind {
    Payout,
    Refund,
}

#[contractevent]
pub struct MarketCreated {
    #[topic]
    pub market_id: u64,
    #[topic]
    pub creator: Address,
    pub resolver: Address,
    pub token: Address,
    pub close_at: u64,
    pub resolution_timeout: u64,
}

#[contractevent]
pub struct Staked {
    #[topic]
    pub market_id: u64,
    #[topic]
    pub user: Address,
    pub side: Side,
    pub amount: i128,
    pub yes_pool: i128,
    pub no_pool: i128,
}

#[contractevent]
pub struct MarketResolved {
    #[topic]
    pub market_id: u64,
    #[topic]
    pub resolver: Address,
    pub outcome: Outcome,
}

#[contractevent]
pub struct MarketInvalidated {
    #[topic]
    pub market_id: u64,
    #[topic]
    pub caller: Address,
}

#[contractevent]
pub struct Claimed {
    #[topic]
    pub market_id: u64,
    #[topic]
    pub user: Address,
    pub amount: i128,
    pub kind: ClaimKind,
}
