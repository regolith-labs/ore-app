const COMPUTE_UNIT_BUFFER: f64 = 0.2;

pub struct ComputeUnits {
    pub base: u32,
    pub adjusted: u32,
}

impl ComputeUnits {
    const fn new(base: u32) -> Self {
        let buffer = (base as f64 * COMPUTE_UNIT_BUFFER) as u32;
        Self {
            base,
            adjusted: base + buffer,
        }
    }
}

/// Pre-calculated compute units for all transaction types
pub struct TransactionComputeUnits {
    pub idle_withdraw: ComputeUnits,
    pub idle_deposit: ComputeUnits,
    pub _swap: ComputeUnits,
    pub pool_register: ComputeUnits,
    pub pair_deposit: ComputeUnits,
    pub pair_withdraw: ComputeUnits,
    pub boost_claim: ComputeUnits,
    pub boost_claim_all: ComputeUnits,
    pub pool_claim: ComputeUnits,
    // Add other transaction types as needed
}

impl TransactionComputeUnits {
    pub const fn new() -> Self {
        Self {
            idle_withdraw: ComputeUnits::new(17784), // * buffer = 17,194 -> done
            idle_deposit: ComputeUnits::new(28580),  // * buffer = 20,692 -> done
            _swap: ComputeUnits::new(10000),
            pool_register: ComputeUnits::new(16763), // * buffer = 20,115
            pair_deposit: ComputeUnits::new(318950), // * buffer = 381,802 -> done
            pair_withdraw: ComputeUnits::new(323033), // * buffer = 387,987 -> done
            boost_claim: ComputeUnits::new(13778),   // * buffer = 16,533 -> 167,732 -> done
            boost_claim_all: ComputeUnits::new(17110), // * buffer = 16,713 -> donee
            pool_claim: ComputeUnits::new(42925),    // * buffer = 51,510
        }
    }
}

/// Global instance of pre-calculated compute units
pub const TRANSACTION_COMPUTE_UNITS: TransactionComputeUnits = TransactionComputeUnits::new();
