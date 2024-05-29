//     let ch = self.ch.clone();
//     let flag = Arc::new(AtomicBool::new(false));
//     let result = Arc::new(Mutex::new(MiningResult::default()));
//     let power_percent = ((self.power_level.read().0 + 1) as f64) / 8f64;
//     let concurrency = num_cpus::get() as u64;
//     let tuned_concurrency = ((concurrency as f64) * power_percent).round() as u64;
//     let handles: Vec<_> = (0..tuned_concurrency)
//         .map(|i| {
//             std::thread::spawn({
//                 let flag = flag.clone();
//                 let result = result.clone();
//                 move || {
//                     let nonce =
//                         u64::MAX.saturating_div(tuned_concurrency).saturating_mul(i);
//                     if let Some(res) =
//                         find_next_hash_par(hash, difficulty, signer, nonce, flag.clone())
//                     {
//                         flag.store(true, Ordering::Relaxed);
//                         let mut w_result = result.lock().unwrap();
//                         *w_result = res;
//                     }
//                 }
//             })
//         })
//         .collect();
//     async_std::task::spawn(async move {
//         for h in handles {
//             h.join().unwrap();
//         }
//         let res = {
//             let r_result = result.lock().unwrap();
//             r_result.clone()
//         };
//         ch.send(res).await.ok();
//     });

fn find_next_hash_par(
    hash: KeccakHash,
    difficulty: KeccakHash,
    signer: Pubkey,
    nonce: u64,
    flag: Arc<AtomicBool>,
) -> Option<MiningResult> {
    let mut next_hash: KeccakHash;
    let mut nonce = nonce;
    loop {
        if nonce % 10_000 == 0 && flag.load(Ordering::Relaxed) {
            return None;
        }
        next_hash = hashv(&[
            hash.as_ref(),
            signer.as_ref(),
            nonce.to_le_bytes().as_slice(),
        ]);
        if next_hash.le(&difficulty) {
            break;
        }
        nonce += 1;
    }
    Some(MiningResult {
        hash: next_hash,
        nonce,
    })
}
