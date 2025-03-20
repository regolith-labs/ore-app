use dioxus::prelude::*;
use ore_boost_api::state::Stake;

use crate::{
    components::{Col, PairDepositForm, PairWithdrawForm, StakeTab, StakeTabs},
    config::BoostMeta,
    gateway::{GatewayResult, UiTokenAmount},
    utils::LiquidityPair,
};

#[component]
pub fn PairStakeForm(
    class: Option<String>,
    boost_meta: BoostMeta,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Signal<GatewayResult<Stake>>,
    token_a_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_b_balance: Resource<GatewayResult<UiTokenAmount>>,
) -> Element {
    let class = class.unwrap_or_default();
    let tab = use_signal(|| StakeTab::Deposit);

    rsx! {
        Col {
            class: "{class}",
            StakeTabs {
                tab: tab
            }
            match *tab.read() {
                StakeTab::Deposit => rsx! {
                    PairDepositForm {
                        boost_meta: boost_meta,
                        liquidity_pair: liquidity_pair,
                        lp_balance: lp_balance,
                        stake: stake,
                        token_a_balance: token_a_balance,
                        token_b_balance: token_b_balance,
                    }
                },
                StakeTab::Withdraw => rsx! {
                    PairWithdrawForm {
                        boost_meta: boost_meta,
                        liquidity_pair: liquidity_pair,
                        lp_balance: lp_balance,
                        stake: stake,
                        token_a_balance: token_a_balance,
                        token_b_balance: token_b_balance,
                    }
                }
            }
        }
    }
}
