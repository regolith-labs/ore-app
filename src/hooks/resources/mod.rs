mod use_boost;
mod use_liquidity_pair;
mod use_stake;
mod use_token_balance;

pub use use_boost::*;
pub use use_liquidity_pair::*;
pub use use_stake::*;
pub use use_token_balance::*;

pub fn use_cache_provider() {
    use_boosts_provider();
    use_liquidity_pairs_provider();
    use_stakes_provider();
}
