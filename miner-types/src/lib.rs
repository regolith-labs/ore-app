use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessage {
    pub member: ore_pool_types::Member,
    pub challenge: ore_pool_types::MemberChallenge,
    pub cutoff_time: i64,
    pub cores: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OutputMessage {
    Init,
    Solution(drillx::Solution),
    Expired(LastHashAt),
    TimeRemaining(Seconds, CpuUtilization),
}
type LastHashAt = i64;
type Seconds = i64;
type CpuUtilization = Vec<f32>;
