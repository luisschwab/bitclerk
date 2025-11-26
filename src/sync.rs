use bdk_esplora::{EsploraExt, esplora_client};
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResponse};
use bdk_wallet::{KeychainKind, Wallet};

use crate::error::ClerkMistake;
use crate::{ESPLORA_URL, NETWORK, PARALLEL_REQUESTS, STOP_GAP};

pub(crate) fn sync_wallet(descriptor: String, hide_desc: bool) -> Result<(), ClerkMistake> {
    let mut wallet = Wallet::create_single(descriptor.clone())
        .network(NETWORK)
        .create_wallet_no_persist()?;

    let esplora = esplora_client::Builder::new(ESPLORA_URL).build_blocking();

    let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();

    match hide_desc {
        false => println!(
            "\nSynchronizing wallet with descriptor\n {descriptor}\nfrom\n {ESPLORA_URL}...\n"
        ),
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

    let available_utxos = wallet.list_unspent();

    println!(">AVAILABLE UTXOs");
    print_header();
    for (idx, utxo) in available_utxos.enumerate() {
        let amount = utxo.txout.value.to_sat();
        let outpoint = utxo.outpoint.to_string();

        print_row(idx, amount, outpoint);
    }
    print_footer();

    Ok(())
}

fn print_header() {
    println!(
        "+-------+---------------------------+--------------------------------------------------------------------+"
    );
    println!("| {:<5} | {:<25} | {:<66} |", "INDEX", "AMOUNT", "OUTPOINT");
    println!(
        "+-------+---------------------------+--------------------------------------------------------------------+"
    );
}

fn print_row(index: usize, amount: u64, outpoint: String) {
    let amount_str = format_amount(amount);
    println!("| {:<5} | {:<25} | {:<66} |", index, amount_str, outpoint);
}

fn print_footer() {
    println!(
        "+-------+---------------------------+--------------------------------------------------------------------+"
    );
    println!();
}

fn format_amount(amount: u64) -> String {
    let amount_str = amount.to_string();
    let mut result = String::new();
    let chars: Vec<char> = amount_str.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }

    format!("{} sats", result)
}
