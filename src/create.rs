use std::collections::HashMap;

use bdk_esplora::{EsploraExt, esplora_client};
use bdk_wallet::bitcoin::script::{PushBytesBuf, ScriptBuf};
use bdk_wallet::bitcoin::{Amount, OutPoint};
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResponse};
use bdk_wallet::{KeychainKind, TxOrdering, Wallet};

use crate::error::ClerkMistake;
use crate::{ESPLORA_URL, NETWORK, PARALLEL_REQUESTS, STOP_GAP};

const DEFAULT_FEE: usize = 420;

fn sync_wallet(wallet: &mut Wallet, hide_desc: bool) -> Result<(), ClerkMistake> {
    let esplora = esplora_client::Builder::new(ESPLORA_URL).build_blocking();

    let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();

    match hide_desc {
        false => {
            let descriptor = wallet.public_descriptor(KeychainKind::External);
            println!(
                "\nSynchronizing wallet with descriptor\n {descriptor}\nfrom\n {ESPLORA_URL}...\n"
            );
        }
        true => {
            println!(
                "\nSynchronizing wallet with descriptor\n <redacted>\nfrom\n {ESPLORA_URL}...\n"
            )
        }
    }

    let update: FullScanResponse<KeychainKind> = esplora
        .full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)
        .unwrap();
    wallet.apply_update(update)?;

    Ok(())
}

pub(crate) fn create_tx(
    descriptor: String,
    utxo_idx: usize,
    custom_fee: Option<usize>,
    op_returns: Vec<String>,
    hide_desc: bool,
) -> Result<(), ClerkMistake> {
    let mut wallet = Wallet::create_single(descriptor.clone())
        .network(NETWORK)
        .create_wallet_no_persist()?;

    sync_wallet(&mut wallet, hide_desc)?;

    let mut utxo_map: HashMap<usize, (OutPoint, Amount)> = HashMap::new();
    for (idx, utxo) in wallet.list_unspent().enumerate() {
        utxo_map.insert(idx, (utxo.outpoint, utxo.txout.value));
    }

    let return_addr = wallet.next_unused_address(KeychainKind::External);

    let selected_utxo = utxo_map
        .get(&utxo_idx)
        .ok_or(ClerkMistake::OutPointNotFound)?;

    let mut tx_builder = wallet.build_tx();

    let fee = if let Some(fee) = custom_fee {
        Amount::from_sat(fee as u64)
    } else {
        Amount::from_sat(DEFAULT_FEE as u64)
    };
    tx_builder.fee_absolute(fee);

    tx_builder.ordering(TxOrdering::Untouched);

    tx_builder.add_recipient(return_addr.script_pubkey(), selected_utxo.1 - fee);

    for op_return in op_returns {
        let bytes = op_return.into_bytes();
        let push_bytes = PushBytesBuf::try_from(bytes).expect("OP_RETURN data under 4GB limit");
        let script = ScriptBuf::new_op_return(&push_bytes);
        tx_builder.add_recipient(script, Amount::ZERO);
    }

    let unsigned_psbt = tx_builder.finish()?;
    let unsigned_psbt_str = unsigned_psbt.serialize_hex();
    println!("SERIALIZED UNSIGNED PSBT (SIGN AND BROADCAST IT YOURSELF)");
    println!("{unsigned_psbt_str}");

    Ok(())
}
