/*
Transaction builder
- unix timestamp
inAmount: u64,: total amount to sell
inAmountPerCycle: ttal amount to slel per buy
cycleFrequency: i64: number of seconds between peridic buys
startAt: i64: unix timestamp of when to start




pub fn open_dca_v2(
    ctx: Context<OpenDcaOnBehalf>,
    application_idx: u64,
    in_amount: u64,
    in_amount_per_cycle: u64,
    cycle_frequency: i64,
    min_out_amount: Option<u64>,
    max_out_amount: Option<u64>,
    start_at: Option<i64>,
) -> Result<()> {



 const [dca] = await PublicKey.findProgramAddressSync(
  [
    Buffer.from("dca"),
    userPubKey.toBuffer(),
    inTokenPubKey.toBuffer(),
    outTokenPubKey.toBuffer(),
    new BN(parseInt((Date.now() / 1000).toString())).toArrayLike(
      Buffer,
      "le",
      8
    ),
  ],
  new PublicKey("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M")
);
*/
