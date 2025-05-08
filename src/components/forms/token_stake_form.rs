use dioxus::prelude::*;
use ore_boost_api::state::Stake;

use crate::components::{Col, StakeTab, StakeTabs, TokenDepositForm, TokenWithdrawForm};
use crate::config::Token;
use crate::gateway::{GatewayResult, UiTokenAmount};

#[component]
pub fn TokenStakeForm(
    class: Option<String>,
    balance: Signal<GatewayResult<UiTokenAmount>>,
    stake: Signal<GatewayResult<Stake>>,
    token: Signal<Option<Token>>,
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
                    TokenDepositForm {
                        balance,
                        stake,
                        token,
                    }
                },
                StakeTab::Withdraw => rsx! {
                    TokenWithdrawForm {
                        balance,
                        stake,
                        token,
                    }
                }
            }
        }
    }
}
