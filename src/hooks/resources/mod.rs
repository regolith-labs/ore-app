
mod use_boost;
mod use_boost_apy;
mod use_boost_tvl;
mod use_liquidity_pair;
mod use_stake;
mod use_token_balance;
mod use_ore_holders;
mod use_ore_price;
mod use_member;

pub use use_boost::*;
pub use use_boost_apy::*;
pub use use_boost_tvl::*;
pub use use_liquidity_pair::*;
pub use use_stake::*;
pub use use_token_balance::*;
pub use use_ore_price::*;
pub use use_ore_holders::*;
pub use use_member::*;

pub const MIN_SOL_BALANCE: f64 = 0.2;

pub fn use_cache_provider() {
    use_boosts_provider();
    use_liquidity_pairs_provider();
    use_stakes_provider();
    use_ore_price_provider();
    use_members_provider();
    use_boost_yield_provider();
    use_token_balance_provider();
    // TODO ADD TOKEENS 
}