mod claim_confirm;
mod claim_done;
mod claim_edit;
mod claim_preview;

use dioxus::prelude::*;

use crate::{
    components::claim_modal::{
        claim_confirm::ClaimConfirm, claim_done::ClaimDone, claim_edit::ClaimEdit,
        claim_preview::ClaimPreview,
    },
    gateway::AsyncResult,
    hooks::use_proof,
};

pub struct IsModalOpen(pub bool);

pub enum ClaimStep {
    Preview,
    Edit,
    Confirm,
    Done,
}

#[derive(Props)]
pub struct ClaimModalProps<'a> {
    pub balance_handle: &'a UseFuture<()>,
}

#[component]
pub fn ClaimModal<'a>(cx: Scope<'a, ClaimModalProps<'a>>) -> Element {
    let (proof_rw, proof_) = use_proof(cx);
    let proof = *proof_rw.read().unwrap();
    let claim_step = use_state(cx, || ClaimStep::Preview);
    let is_modal_open = use_shared_state::<IsModalOpen>(cx).unwrap();
    let amount_input = use_state(cx, || "".to_string());
    let balance_ = cx.props.balance_handle;

    let parsed_amount: u64 = match amount_input.get().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore::TOKEN_DECIMALS.into())) as u64,
        Err(_) => 0,
    };

    let amount = match &proof {
        AsyncResult::Ok(proof) => proof.claimable_rewards,
        _ => 0,
    };

    let is_open = is_modal_open.read().0;
    use_effect(cx, &is_open, |_| {
        if !is_open {
            claim_step.set(ClaimStep::Preview);
        }
        async move {}
    });

    let modal_opacity = if is_modal_open.read().0 {
        "opacity-100"
    } else {
        "opacity-0 pointer-events-none"
    };

    let bg_opacity = if is_modal_open.read().0 {
        "opacity-80"
    } else {
        "opacity-0 pointer-events-none"
    };

    let size = match claim_step.get() {
        ClaimStep::Preview | ClaimStep::Done => "w-11/12 max-w-[48rem] rounded",
        ClaimStep::Edit | ClaimStep::Confirm => "h-full w-full",
    };

    render! {
        button {
            class: "absolute transition-opacity flex flex-row left-0 top-0 h-screen w-screen bg-black z-[100] {bg_opacity}",
            onclick: |_e| {
                *is_modal_open.write() = IsModalOpen(false);
            }
        }
        div {
            class: "absolute transition-transform transition-opacity bg-white z-[100] top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 drop-shadow {modal_opacity} {size}",
            match claim_step.get() {
                ClaimStep::Preview => {
                    render! {
                        ClaimPreview {
                            claim_step: claim_step,
                        }
                    }
                }
                ClaimStep::Edit => {
                    render! {
                        ClaimEdit {
                            claim_step: claim_step,
                            amount_input: amount_input,
                            max_rewards: amount,
                            parsed_amount: parsed_amount,
                        }
                    }
                }
                ClaimStep::Confirm => {
                    render! {
                        ClaimConfirm {
                            claim_step: claim_step,
                            amount: parsed_amount,
                            balance_handle: balance_,
                            proof_handle: proof_,
                        }
                    }
                }
                ClaimStep::Done => {
                    render! {
                        ClaimDone {
                            amount_input: amount_input
                        }
                    }
                }
            }
        }
    }
}
