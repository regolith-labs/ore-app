use crate::config::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct LiquidityPair {
    pub token_a: Token,
    pub token_b: Token,
    pub balance_a_f64: f64,
    pub balance_b_f64: f64,
    pub total_value_usd: f64,
    pub shares: u64,
}

impl LiquidityPair {
    pub fn get_stake_amounts(&self, stake_balance: u64) -> (f64, f64, String, u8) {
        let stake_share = stake_balance as f64 / self.shares as f64;
        let stake_amount_a = self.balance_a_f64 * stake_share;
        let stake_amount_b = self.balance_b_f64 * stake_share;
        if self.token_a.ticker == "ORE" {
            (stake_amount_a, stake_amount_b, self.token_b.ticker.clone(), self.token_b.decimals)
        } else {
            (stake_amount_b, stake_amount_a, self.token_a.ticker.clone(), self.token_a.decimals)
        }
    }
}