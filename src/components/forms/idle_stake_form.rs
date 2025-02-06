use dioxus::prelude::*;
use ore_boost_api::state::Stake;

use crate::components::{Col, StakeTab, StakeTabs, IdleDepositForm, IdleWithdrawForm};
use crate::gateway::{UiTokenAmount, GatewayResult};

#[component]
pub fn IdleStakeForm(
    class: Option<String>,
    ore_balance: Resource<GatewayResult<UiTokenAmount>>,
    ore_stake: Resource<GatewayResult<Stake>>
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
                    IdleDepositForm {
                        ore_balance: ore_balance,
                        ore_stake: ore_stake,
                    }
                },
                StakeTab::Withdraw => rsx! {
                    IdleWithdrawForm {
                        ore_balance: ore_balance,
                        ore_stake: ore_stake,
                    }
                }
            }
        }
    }
}
