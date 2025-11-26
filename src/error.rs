use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ClerkMistake {
    /// Esplora related error.
    #[error(transparent)]
    Esplora(#[from] bdk_esplora::esplora_client::Error),

    /// [`Wallet`] creation error.
    #[error(transparent)]
    WalletCreation(#[from] bdk_wallet::descriptor::DescriptorError),

    /// Cannot apply update to [`Wallet`] error.
    #[error(transparent)]
    WalletUpdate(#[from] bdk_wallet::chain::local_chain::CannotConnectError),

    /// The selected [`OutPoint`] was not found.
    #[error("404 OutPoint Not Found")]
    OutPointNotFound,

    /// Error whilst creating a PSBT.
    #[error(transparent)]
    PsbtCreation(#[from] bdk_wallet::error::CreateTxError),

    /// Error whilst extracting a transaction from a PSBT.
    #[error(transparent)]
    TransactionExtraction(#[from] bdk_wallet::bitcoin::psbt::ExtractTxError),
}
