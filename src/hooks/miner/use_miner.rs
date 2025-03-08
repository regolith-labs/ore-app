use dioxus::prelude::*;

use ore_miner_types::{InputMessage, OutputMessage};

type FromMiner = Signal<OutputMessage>;
type ToMiner = Coroutine<InputMessage>;

pub fn use_miner() -> (FromMiner, ToMiner) {
    let from = use_context::<Signal<OutputMessage>>();
    let to = use_coroutine_handle::<InputMessage>();
    (from, to)
}

#[derive(Clone, Debug, PartialEq)]
pub enum MinerStatus {
    Stopped,
    Registering,
    FetchingChallenge,
    Hashing,
    SubmittingSolution,
}

pub fn use_miner_status_provider() {
    use_context_provider(|| Signal::new(MinerStatus::Stopped));
}

pub fn use_miner_status() -> Signal<MinerStatus> {
    use_context()
}

pub fn use_miner_is_active() -> Memo<bool> {
    let miner_status = use_miner_status();
    use_memo(move || {
        log::info!("Miner status: {:?}", miner_status.cloned());
        match miner_status.cloned() {
            MinerStatus::FetchingChallenge
            | MinerStatus::Hashing
            | MinerStatus::SubmittingSolution => true,
            _ => false,
        }
    })
}

pub fn use_miner_cores_provider() {
    use_context_provider(|| Signal::new(1usize));
}

pub fn use_miner_cores() -> Signal<usize> {
    use_context()
}

pub fn _use_miner_cpu_utilization() -> Signal<Vec<f32>> {
    let (from_miner, _to_miner) = use_miner();
    let mut signal = use_signal(|| vec![]);

    // Always update the signal when we receive CPU utilization data
    use_memo(move || {
        if let OutputMessage::TimeRemaining(_, vec) = &*from_miner.read() {
            signal.set(vec.clone());
        }
    });

    signal
}

#[cfg(not(feature = "web"))]
pub fn use_system_cpu_utilization() -> Signal<Vec<f32>> {
    let signal = use_signal(|| vec![]);
    // Set up a continuous monitor that updates CPU usage every second
    let mut signal_clone = signal.clone();
    use_future(move || {
        async move {
            let mut sys = sysinfo::System::new();
            sys.refresh_cpu_usage();
            async_std::task::sleep(crate::time::Duration::from_millis(100)).await;
            loop {
                // Get CPU utilization - single refresh is sufficient
                let cpus = {
                    sys.refresh_cpu_usage();

                    // Wait a moment for the measurement to be valid
                    async_std::task::sleep(crate::time::Duration::from_millis(100)).await;

                    // Collect CPU usage data
                    sys.cpus()
                        .into_iter()
                        .map(|cpu| cpu.cpu_usage())
                        .collect::<Vec<_>>()
                };

                // Update the signal with the latest CPU utilization data
                signal_clone.set(cpus);

                // Sleep before the next update
                async_std::task::sleep(crate::time::Duration::from_millis(1000)).await;
            }
        }
    });

    signal
}

// For web, we don't need this function as we'll handle it differently in the MinePower component
#[cfg(feature = "web")]
pub fn use_system_cpu_utilization() -> Signal<Vec<f32>> {
    use_signal(|| vec![])
}
