use anyhow::Result;
use bdk_wallet::bitcoin::Network;
use clap::Parser;

pub(crate) mod create;
pub(crate) mod error;
pub(crate) mod sync;

use create::create_tx;
use sync::sync_wallet;

pub(crate) const ESPLORA_URL: &str = "https://mempool.emzy.de/api";
pub(crate) const NETWORK: Network = Network::Bitcoin;
pub(crate) const STOP_GAP: usize = 10;
pub(crate) const PARALLEL_REQUESTS: usize = 5;

#[derive(Parser)]
#[command(about = "bitclerk: A CLI tool to build transactions with OP_RETURNs from a descriptor")]
pub(crate) struct Arguments {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Parser)]
pub(crate) enum Command {
    /// Synchronize the wallet and list available UTXOs
    Sync(SyncArgs),
    /// Create a transaction from available UTXOs
    Create(CreateArgs),
}

#[derive(Parser)]
pub(crate) struct SyncArgs {
    /// The public descriptor to synchronize from.
    #[arg(long)]
    pub(crate) desc: String,

    /// Whether to omit the descriptor from stdout.
    #[arg(long, short = 'h', default_value_t = false)]
    pub(crate) hide_desc: bool,
}

#[derive(Parser)]
pub(crate) struct CreateArgs {
    /// The public descriptor which to create and sign PSBTs from.
    #[arg(long)]
    pub(crate) desc: String,

    /// Which UTXO to use as input for the PSBT.
    #[arg(long)]
    pub(crate) utxo: usize,

    /// The fee amount in sats. Defaults to 420 sats.
    #[arg(long)]
    pub(crate) fee: Option<usize>,

    /// Messages for the OP_RETURNs (each string will create a new OP_RETURN).
    #[arg(long, required = true, num_args = 1..)]
    pub(crate) msg: Vec<String>,

    /// Whether to omit the descriptor from stdout.
    #[arg(long, short = 'h', default_value_t = false)]
    pub(crate) hide_desc: bool,
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    match args.command {
        Command::Sync(sync_args) => {
            let descriptor = sync_args.desc;
            let hide_descriptor = sync_args.hide_desc;

            sync_wallet(descriptor, hide_descriptor)?;
        }
        Command::Create(create_args) => {
            let descriptor = create_args.desc;
            let utxo_idx = create_args.utxo;
            let fee = create_args.fee;
            let op_returns = create_args.msg;
            let hide_descriptor = create_args.hide_desc;

            create_tx(descriptor, utxo_idx, fee, op_returns, hide_descriptor)?;
        }
    }
    Ok(())
}
