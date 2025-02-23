use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessage {
    pub member: ore_pool_types::Member,
    pub challenge: ore_pool_types::MemberChallenge,
    pub cutoff_time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum OutputMessage {
    Init,
    Solution(drillx::Solution),
    Expired(LastHashAt),
}
type LastHashAt = i64;
