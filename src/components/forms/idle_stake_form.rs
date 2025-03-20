use dioxus::prelude::*;
use ore_boost_api::state::Stake;

use crate::components::{Col, IdleDepositForm, IdleWithdrawForm, StakeTab, StakeTabs};
use crate::gateway::{GatewayResult, UiTokenAmount};

#[component]
pub fn IdleStakeForm(
    class: Option<String>,
    balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Signal<GatewayResult<Stake>>,
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
                        balance,
                        stake,
                    }
                },
                StakeTab::Withdraw => rsx! {
                    IdleWithdrawForm {
                        balance,
                        stake,
                    }
                }
            }
        }
    }
}
