use anyhow::{bail, Result};
use cbor_diag;
use clap::Parser;
use cli::Cli;
use ouro::Ouroboros;
use pallas::{
    ledger::traverse::MultiEraBlock,
    network::miniprotocols::{blockfetch, Point},
};

pub mod cli;
pub mod ouro;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let mut chain = Ouroboros::new(cli.network())?;

    let hash = match hex::decode(cli.hash()) {
        Ok(h) => h,
        Err(_) => bail!("Block hash invalid hex"),
    };

    let block_bytes = match chain.get_block_bytes(Point::Specific(cli.slot(), hash)) {
        Ok(b) => b,
        Err(blockfetch::Error::NoBlocks) => {
            bail!("Block not found for supplied point")
        }
        Err(e) => bail!("Unexpected error while fetching block: {e}"),
    };

    let res = match cli.tx_at() {
        None => block_bytes,
        Some(idx) => {
            let block = match MultiEraBlock::decode(&block_bytes) {
                Ok(b) => b,
                Err(e) => bail!("Unable to decode block: {e}"),
            };

            match block.txs().get(idx) {
                Some(tx) => tx.encode(),
                None => bail!(
                    "Invalid index for block containing {} transactions",
                    block.tx_count()
                ),
            }
        }
    };

    match cli.diag() {
        false => println!("{}", hex::encode(res)),
        true => {
            let diag = match cbor_diag::parse_bytes(res) {
                Ok(d) => d,
                Err(e) => bail!("Unable to parse CBOR: {e}"),
            };

            println!("{}", diag.to_hex())
        }
    }

    Ok(())
}
