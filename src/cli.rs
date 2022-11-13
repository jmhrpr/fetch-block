use clap::Parser;

use crate::ouro::Network;

#[derive(Parser)]
#[command(name = "fetch-block")]
#[command(version = "0.1.0")]
#[command(about = "Fetch the bytes of a block from a Cardano network", long_about = None)]
pub struct Cli {
    /// The Cardano network from which to fetch the block from
    #[arg(long, value_enum)]
    network: Network,

    /// The slot of the block to fetch
    #[arg(long, value_name = "BLOCK_SLOT")]
    slot: u64,

    /// The block hash of the block to fetch
    #[arg(long, value_name = "BLOCK_HASH_HEX")]
    hash: String,

    /// (Optional) Return only the transaction at this index in the block
    #[arg(long, value_name = "TX_INDEX")]
    tx_at: Option<usize>,

    /// Print a diagnostic representation of the CBOR
    #[arg(long)]
    diag: bool,
}

impl Cli {
    pub fn network(&self) -> Network {
        self.network
    }

    pub fn slot(&self) -> u64 {
        self.slot
    }

    pub fn hash(&self) -> String {
        self.hash.clone()
    }

    pub fn tx_at(&self) -> Option<usize> {
        self.tx_at
    }

    pub fn diag(&self) -> bool {
        self.diag
    }
}
