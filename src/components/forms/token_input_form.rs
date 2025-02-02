use dioxus::prelude::*;

use crate::{components::{Col, LoadingValue, Row}, config::Token, gateway::{GatewayResult, UiTokenAmount}};

#[derive(Clone, PartialEq, Eq)]
pub enum TokenInputError {
    InsufficientBalance(Token),
}

impl ToString for TokenInputError {
    fn to_string(&self) -> String {
        match self {
            TokenInputError::InsufficientBalance(token) => format!("Not enough {}", token.ticker),
        }
    }
}

#[component]
pub fn TokenInputForm(
    class: Option<String>,
    title: String,
    token: Option<Token>,
    balance: Resource<GatewayResult<UiTokenAmount>>,
    mut value: Signal<String>,
    err: Signal<Option<TokenInputError>>,
    toolbar_shortcuts: Option<bool>,
) -> Element {
    let class = class.unwrap_or("".to_string());

    let color = if let Some(token) = token.clone() {
        match err.cloned() {
            Some(TokenInputError::InsufficientBalance(err_token)) if err_token.ticker == token.ticker => {
                "text-red-500"
            }
            _ => "text-elements-primary"
        }
    } else {
        "text-elements-primary"
    };

    rsx! {
        Col {
            class: "{class}",
            gap: 4,
            Row {
                class: "justify-between",
                span {
                    class: "text-elements-lowEmphasis my-auto pl-1",
                    "{title}"
                }
                if let Some(token) = token.clone() {
                    if let Some(Ok(balance)) = balance.cloned() {
                        Toolbar {
                            balance: balance.clone(),
                            token: token.clone(),
                            value: value.clone(),
                            toolbar_shortcuts,
                        }
                    }
                }
            }
            Row {
                gap: 4,
                if let Some(token) = token.clone() {
                    Row {
                        class: "my-auto",
                        gap: 2,
                        img {
                            class: "w-8 h-8 rounded-full",
                            src: "{token.image}",
                        }
                        span {
                            class: "font-semibold my-auto",
                            "{token.ticker}"
                        }
                    }
                    input {
                        class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none {color}",
                        placeholder: "0",
                        r#type: "number",
                        inputmode: "decimal",
                        value: value.read().clone(),
                        oninput: move |e: FormEvent| value.set(e.value()),
                    }
                } else {
                    LoadingValue {}
                }
            }
        }
    }
}

#[component]
fn Toolbar(
    balance: UiTokenAmount,
    token: Token,
    value: Signal<String>,
    toolbar_shortcuts: Option<bool>,
) -> Element {
    let (half_value, max_value) = if let Some(ui_amount) = balance.ui_amount {
        (
            format!("{:.1$}", (ui_amount / 2.0), balance.decimals as usize),
            format!("{:.1$}", (ui_amount), balance.decimals as usize)
        )
    } else {
        ("0".to_string(), "0".to_string())
    };

    rsx! {
        Row {
            gap: 2,
            ToolbarBalance {
                balance: balance.clone(),
                token: token.clone(),
            }
            if toolbar_shortcuts.unwrap_or(false) {
                ToolbarButton {
                    title: "HALF".to_string(),
                    shortcut_value: half_value.to_string(),
                    value: value.clone(),
                }
                ToolbarButton {
                    title: "MAX".to_string(),
                    shortcut_value: max_value.to_string(),
                    value: value.clone(),
                }
            }
        }
    }
}

#[component]
fn ToolbarBalance(
    balance: UiTokenAmount,
    token: Token,
) -> Element {
    rsx! {
        Row {
            class: "py-1 px-1 font-medium text-elements-lowEmphasis my-auto",
            gap: 2,
            span { 
                class: "my-auto text-xs font-medium", 
                "{balance.ui_amount_string} {token.ticker}" 
            }
        }
    }
}
#[component]
fn ToolbarButton(
    title: String,
    shortcut_value: String,
    value: Signal<String>,
) -> Element {
    rsx! {
        button {
            class: "flex flex-row gap-2 py-1 px-2 rounded controls-tertiary my-auto text-xs font-semibold font-sans",
            onclick: move |_| value.set(
                shortcut_value.clone()
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            ),
            "{title}"
        }
    }
}
