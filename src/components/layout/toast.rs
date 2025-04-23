use dioxus::prelude::*;

use crate::components::{Col, Row, Spinner, TransactionStatus, Updater};
use crate::gateway::GatewayError;
use crate::hooks::use_transaction_status;
use crate::time::Duration;

pub fn ToastDisplay() -> Element {
    let transaction_status = use_transaction_status();

    // If terminal status, hide after 5 seconds
    use_effect(move || {
        let mut transaction_status_signal = transaction_status.clone();
        if let Some(transaction_status) = transaction_status.read().clone() {
            match transaction_status {
                TransactionStatus::Denied
                | TransactionStatus::Error(_)
                | TransactionStatus::Timeout
                | TransactionStatus::Done(_) => {
                    spawn(async move {
                        async_std::task::sleep(Duration::from_secs(8)).await;
                        transaction_status_signal.set(None);
                    });
                }
                _ => {}
            }
        }
    });

    let toast_class = "fixed bottom-4 left-4 ml-auto z-100 bg-surface-floating p-4 rounded";
    let title_class = "text-elements-highEmphasis font-medium text-lg";
    let detail_class = "text-elements-lowEmphasis";

    rsx! {
        if let Some(transaction_status) = transaction_status.read().clone() {
            match transaction_status {
                TransactionStatus::Waiting => {
                    rsx! {
                        Row { class: "{toast_class} border-l-4 border-elements-lowEmphasis", gap: 2,
                            Spinner { class: "my-auto" }
                            span { class: "{title_class} my-auto", "Waiting for signature..." }
                        }
                    }
                }
                TransactionStatus::Denied => {
                    rsx! {
                        Col { class: "{toast_class} border-l-4 border-red-500",
                            span { class: "{title_class} my-auto", "Signature denied" }
                        }
                    }
                }
                // Remove option from transcation status & sub gatewayerror::Uknown for None into TransactionStatus::Error
                TransactionStatus::Error(err) => {
                    // Display different error messages based on the GatewayError type
                    let error_message = match err {
                        GatewayError::InsufficientSOL => "Insufficient SOL balance",
                        GatewayError::Unknown => "Transaction failed",
                        _ => "Transaction failed"
                    };
                    
                    rsx! {
                        Col { class: "{toast_class} border-l-4 border-red-500",
                            span { class: "{title_class} my-auto", "{error_message}" }
                        }
                    }
                }
                TransactionStatus::Timeout => {
                    rsx! {
                        Col { class: "{toast_class} border-l-4 border-red-500",
                            span { class: "{title_class} my-auto", "Transaction timed out" }
                            span { class: "{detail_class}", "Please try again." }
                        }
                    }
                }
                TransactionStatus::Sending(_attempt) => {
                    rsx! {
                        Col { class: "{toast_class} border-l-4 border-blue-500",
                            Row { gap: 2,
                                Spinner { class: "my-auto" }
                                span { class: "{title_class} my-auto", "Submitting transaction" }
                            }
                            span { class: "{detail_class} ml-8", "Waiting for confirmation..." }
                        }
                    }
                }
                TransactionStatus::Done(sig) => {
                    rsx! {
                        a {
                            class: "flex flex-col {toast_class} border-l-4 border-green-500 hover:cursor-pointer",
                            href: "https://solscan.io/tx/{sig}",
                            target: "_blank",
                            span { class: "{title_class}", "Transaction confirmed!" }
                            span { class: "{detail_class}", "View on Solscan" }
                        }
                    }
                }
            }
        }

        // app updater
        Updater {}
    }
}
