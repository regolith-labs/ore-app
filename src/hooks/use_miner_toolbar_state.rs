use dioxus::prelude::*;
use solana_sdk::blake3::Hash as Blake3Hash;

#[derive(Copy, Clone, Debug)]
pub enum MinerStatus {
    NotStarted,
    Activating,
    Active,

    // TODO Add error field
    Error,
}

#[derive(Copy, Clone, Debug)]
pub enum MinerStatusMessage {
    Searching,
    Submitting(u64),
    Error,
    SignatureDenied,
}

pub struct MinerToolbarState {
    pub status: MinerStatus,
    pub status_message: MinerStatusMessage,
    pub display_hash: Blake3Hash,
}

pub fn use_miner_toolbar_state() -> Signal<MinerToolbarState> {
    use_context::<Signal<MinerToolbarState>>()
}

pub fn use_miner_toolbar_state_provider() {
    use_context_provider(|| {
        Signal::new(MinerToolbarState {
            status: MinerStatus::NotStarted,
            status_message: MinerStatusMessage::Searching,
            display_hash: Blake3Hash::new_unique(),
        })
    });
}

pub trait ReadMinerToolbarState {
    fn status(&self) -> MinerStatus;
    fn status_message(&self) -> MinerStatusMessage;
    fn display_hash(&self) -> String;
}

impl ReadMinerToolbarState for Signal<MinerToolbarState> {
    fn status_message(&self) -> MinerStatusMessage {
        self.read().status_message
    }

    fn status(&self) -> MinerStatus {
        self.read().status
    }

    fn display_hash(&self) -> String {
        self.read().display_hash.to_string()
    }
}

pub trait UpdateMinerToolbarState {
    // fn set_is_open(&mut self, is_open: bool);
    fn set_display_hash(&mut self, hash: Blake3Hash);
    fn set_status_message(&mut self, status_message: MinerStatusMessage);
    fn set_status(&mut self, status: MinerStatus);
    fn start(&mut self);
    fn pause(&mut self);
}

impl UpdateMinerToolbarState for Signal<MinerToolbarState> {
    fn start(&mut self) {
        let old = self.read();
        let new = MinerToolbarState {
            status: MinerStatus::Activating,
            status_message: old.status_message,
            display_hash: old.display_hash,
        };
        drop(old);
        self.set(new);
    }

    fn pause(&mut self) {
        let old = self.read();
        let new = MinerToolbarState {
            status: MinerStatus::NotStarted,
            status_message: old.status_message,
            display_hash: old.display_hash,
        };
        drop(old);
        self.set(new);
    }

    fn set_display_hash(&mut self, hash: Blake3Hash) {
        let old = self.read();
        let new = MinerToolbarState {
            status: old.status,
            status_message: old.status_message,
            display_hash: hash,
        };
        drop(old);
        self.set(new);
    }

    fn set_status_message(&mut self, status_message: MinerStatusMessage) {
        let old = self.read();
        let new = MinerToolbarState {
            status: old.status,
            status_message,
            display_hash: old.display_hash,
        };
        drop(old);
        self.set(new);
    }

    fn set_status(&mut self, status: MinerStatus) {
        let old = self.read();
        let new = MinerToolbarState {
            status,
            status_message: old.status_message,
            display_hash: old.display_hash,
        };
        drop(old);
        self.set(new);
    }
}
