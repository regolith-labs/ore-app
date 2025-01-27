use dioxus::prelude::*;

use crate::components::{Col, Row, Spinner, TransactionStatus};
use crate::hooks::use_transaction_status;
use crate::time::Duration;


pub fn ToastDisplay() -> Element {
    let transaction_status = use_transaction_status();

    // If terminal status, hide after 5 seconds
    use_effect(move || {
        let mut transaction_status_signal = transaction_status.clone();
        if let Some(transaction_status) = transaction_status.cloned() {
            match transaction_status {
                TransactionStatus::Denied | TransactionStatus::Error | TransactionStatus::Timeout | TransactionStatus::Done(_) => {
                    spawn(async move {
                        async_std::task::sleep(Duration::from_secs(5)).await;
                        transaction_status_signal.set(None);
                    });
                }
                _ => {}
            }
        }
    });

    log::info!("toast: {:?}", transaction_status.cloned());
    let toast_class = "fixed bottom-4 right-4 ml-auto z-100 bg-surface-floating py-4 px-6 rounded";
    let title_class = "text-elements-highEmphasis font-medium text-lg my-auto";
    let detail_class = "text-elements-lowEmphasis";

    rsx! {
        if let Some(transaction_status) = transaction_status.cloned() {
            match transaction_status {
                TransactionStatus::Waiting => {
                    rsx! {
                        Row {
                            class: "{toast_class}",
                            gap: 2, 
                            Spinner {
                                class: "my-auto",
                            }
                            span {
                                class: "{title_class}",
                                "Waiting for signature..."
                            }
                        }
                    }
                }
                TransactionStatus::Denied => {
                    rsx! {
                        Row {
                            class: "{toast_class}",
                            span {
                                class: "{title_class}",
                                "Signature denied"
                            }
                        }
                    }
                }
                TransactionStatus::Error => {
                    rsx! {
                        Row {
                            class: "{toast_class}",
                            span {
                                class: "{title_class}",
                                "Transaction failed"
                            }
                        }
                    }
                }
                TransactionStatus::Timeout => {
                    rsx! {
                        Row {
                            class: "{toast_class}",
                            span {
                                class: "{title_class}",
                                "Transaction timed out"
                            }
                            span {
                                class: "{detail_class}",
                                "Transaction was submitted but could not be confirmed. Please try again."
                            }
                        }
                    }
                }
                TransactionStatus::Done(sig) => {
                    rsx! {
                        Row {
                            class: "{toast_class}",
                            span {
                                class: "{title_class}",
                                "Done"
                            }
                            span {
                                class: "{detail_class}",
                                "{sig}"
                            }
                        }
                    }
                }
                _ => rsx! {}
            }
        }
    }
}
