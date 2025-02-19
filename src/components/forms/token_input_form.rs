use dioxus::prelude::*;

use crate::{components::{CarrotDownIcon, Col, LoadingValue, LoadingValueSize, Row, TokenPicker}, config::Token, gateway::{GatewayResult, UiTokenAmount}, hooks::MIN_SOL_BALANCE};

#[derive(Clone, PartialEq, Eq)]
pub enum TokenInputError {
    InsufficientBalance(Token),
    InsufficientSol,
}

impl ToString for TokenInputError {
    fn to_string(&self) -> String {
        match self {
            TokenInputError::InsufficientBalance(token) => format!("Not enough {}", token.ticker),
            TokenInputError::InsufficientSol => format!("Not enough SOL (Minimum {:.1} SOL)", MIN_SOL_BALANCE)
        }
    }
}

#[component]
pub fn TokenInputForm(
    class: Option<String>,
    title: String,
    token: Signal<Option<Token>>,
    balance: Resource<GatewayResult<UiTokenAmount>>,
    value: Signal<String>,
    mut update: Signal<String>,
    err: Signal<Option<TokenInputError>>,
    toolbar_shortcuts: Option<bool>,
    with_picker: Option<bool>,
    loading: Option<bool>,
    disabled: Option<bool>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    let disabled = disabled.unwrap_or(false);
    let loading = loading.unwrap_or(false);
    let with_picker = with_picker.unwrap_or(false);
    let display_picker = use_signal(|| false);
    let color = if let Some(token) = token.cloned() {
        match err.cloned() {
            Some(TokenInputError::InsufficientBalance(err_token)) if err_token.ticker == token.ticker => {
                "text-red-500"
            }
            Some(TokenInputError::InsufficientSol) => {
                "text-red-500"
            }
            _ => "text-elements-primary"
        }
    } else {
        "text-elements-primary"
    };

    let placeholder = if disabled {
        "".to_string()
    } else {
        "0".to_string()
    };

    let disabled_class = if disabled {
        "hover:cursor-not-allowed"
    } else {
        ""
    };

    rsx! {
        Col {
            class: "{class}",
            gap: 2,
            Row {
                class: "justify-between",
                span {
                    class: "text-elements-lowEmphasis my-auto pl-1",
                    "{title}"
                }
                if let Some(token) = token.cloned() {
                    Toolbar {
                        balance: balance.clone(),
                        token: token.clone(),
                        update: update.clone(),
                        toolbar_shortcuts,
                    }
                }
            }
            Row {
                class: "justify-between",
                if let Some(token) = token.cloned() {
                    TokenDisplay { 
                        token,
                        with_picker,
                        display_picker,
                    }
                    if loading {
                        LoadingValue {
                            size: LoadingValueSize::Large,
                        }
                    } else {
                        input {
                            class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-12 pr-1 my-auto w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none {color} {disabled_class}",
                            placeholder: placeholder.clone(),
                            r#type: "number",
                            step: "any",
                            inputmode: "decimal",
                            disabled: disabled,
                            value: value.read().clone(),
                            oninput: move |e: FormEvent| update.set(e.value()),
                        }
                    }
                } else {
                    LoadingValue {
                        size: LoadingValueSize::Large,
                    }
                }
            }
        }

        // Token picker
        if display_picker.cloned() {
            TokenPicker {
                show: display_picker,
                token,
            }
        }
    }
}

#[component]
fn TokenDisplay(token: Token, with_picker: bool, display_picker: Signal<bool>) -> Element {
    rsx! {
        if with_picker {
            button {
                class: "flex flex-row gap-2 items-center gap-2 p-2 -ml-2 hover:bg-controls-secondaryHover rounded cursor-pointer shrink-0",
                onclick: move |_| display_picker.set(true),
                img {
                    class: "w-8 h-8 rounded-full",
                    src: "{token.image}",
                }
                span {
                    class: "font-semibold my-auto",
                    "{token.ticker}"
                }
                CarrotDownIcon {
                    class: "w-4 my-auto opacity-50" 
                }
            }
        } else {
            Row {
                class: "my-auto h-12",
                gap: 2,
                img {
                    class: "w-8 h-8 rounded-full my-auto",
                    src: "{token.image}",
                }
                span {
                    class: "font-semibold my-auto",
                    "{token.ticker}"
                }
            }
        }
    }
}

#[component]
fn Toolbar(
    balance: Resource<GatewayResult<UiTokenAmount>>,
    token: Token,
    update: Signal<String>,
    toolbar_shortcuts: Option<bool>,
) -> Element {

    // Get half and max values
    let (half_value, max_value) = if let Some(Ok(balance)) = balance.cloned() {
        if let Some(ui_amount) = balance.ui_amount {
            (
                format!("{:.1$}", (ui_amount / 2.0), balance.decimals as usize),
                balance.ui_amount_string.clone()
            )
        } else {
            ("0".to_string(), "0".to_string())
        }
    } else {
        ("0".to_string(), "0".to_string())
    };

    rsx! {
        Row {
            gap: 2,
            if let Some(balance) = balance.cloned() {
                if let Ok(balance) = balance {
                    ToolbarBalance {
                        ui_amount_string: balance.ui_amount_string.clone(),
                        token: token.clone(),
                    }
                } else {
                    ToolbarBalance {
                        ui_amount_string: "0".to_string(),
                        token: token.clone(),
                    }
                }
            } else {
                LoadingValue {}
            }
            if toolbar_shortcuts.unwrap_or(false) {
                ToolbarButton {
                    title: "HALF".to_string(),
                    shortcut_value: half_value.to_string(),
                    update: update.clone(),
                }
                ToolbarButton {
                    title: "MAX".to_string(),
                    shortcut_value: max_value.to_string(),
                    update: update.clone(),
                }
            }
        }
    }
}

#[component]
fn ToolbarBalance(
    ui_amount_string: String,
    token: Token,
) -> Element {
    rsx! {
        span { 
            class: "my-auto text-xs font-medium py-1 px-1 font-medium text-elements-lowEmphasis", 
            "{ui_amount_string} {token.ticker}" 
        }
    }
}
#[component]
fn ToolbarButton(
    title: String,
    shortcut_value: String,
    update: Signal<String>,
) -> Element {
    rsx! {
        button {
            class: "flex flex-row gap-2 py-1 px-2 rounded controls-tertiary my-auto text-xs font-semibold font-sans",
            onclick: move |_| update.set(shortcut_value.clone()),
            "{title}"
        }
    }
}
