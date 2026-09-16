use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Market ID was not found in storage.
    MarketNotFound = 1,
    /// Market is not open for staking (past deadline or already settled).
    MarketClosed = 2,
    /// Market has not been resolved or invalidated yet.
    MarketNotResolved = 3,
    /// Outcome value is not a recognized enum variant.
    InvalidOutcome = 4,
    /// Caller is not the designated resolver for this market.
    UnauthorizedResolver = 5,
    /// Market has already been resolved or invalidated.
    AlreadyResolved = 6,
    /// Position has already been claimed or refunded.
    AlreadyClaimed = 7,
    /// Caller has no winning stake for the resolved outcome.
    NotWinner = 8,
    /// Position is not eligible for a refund (market not invalid / zero-sided refund path).
    NotRefundable = 9,
    /// Amount must be strictly positive.
    InvalidAmount = 10,
    /// Close time has not been reached yet.
    DeadlineNotReached = 11,
    /// Resolution timeout has not elapsed; invalidation is not yet allowed.
    ResolutionTimeoutNotReached = 12,
    /// Side must be Yes or No.
    InvalidSide = 13,
    /// Market configuration is logically invalid.
    InvalidMarketConfig = 14,
    /// Question string is empty or exceeds the maximum length.
    InvalidQuestion = 15,
    /// Arithmetic overflow during payout or pool accounting.
    Overflow = 16,
    /// No position exists for this user on this market.
    NoPosition = 17,
    /// Market still has an open staking window; resolution is not allowed yet.
    MarketStillOpen = 18,
}
