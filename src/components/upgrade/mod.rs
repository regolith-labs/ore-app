mod edit;

use edit::*;

use dioxus::prelude::*;

pub enum UpgradeStep {
    Edit,
}

#[component]
pub fn Upgrade() -> Element {
    let amount_input = use_signal(|| "".to_string());
    let parsed_amount: u64 = match amount_input.read().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore::TOKEN_DECIMALS_V1.into())) as u64,
        Err(_) => 0,
    };
    rsx! {
        UpgradeEdit {
            amount_input: amount_input,
            parsed_amount: parsed_amount
        }
    }
}
