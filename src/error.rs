use bdk::Error;

pub fn get_name(error: &bdk::Error) -> String {
    match error {
        Error::InvalidU32Bytes(_) => "InvalidU32Bytes",
        Error::Generic(_) => "Generic",
        Error::ScriptDoesntHaveAddressForm => "ScriptDoesntHaveAddressForm",
        Error::SingleRecipientMultipleOutputs => "SingleRecipientMultipleOutputs",
        Error::SingleRecipientNoInputs => "SingleRecipientNoInputs",
        Error::NoRecipients => "NoRecipients",
        Error::NoUtxosSelected => "NoUtxosSelected",
        Error::OutputBelowDustLimit(_) => "OutputBelowDustLimit",
        Error::InsufficientFunds { .. } => "InsufficientFunds",
        Error::BnBTotalTriesExceeded => "BnBTotalTriesExceeded",
        Error::BnBNoExactMatch => "BnBNoExactMatch",
        Error::UnknownUtxo => "UnknownUtxo",
        Error::TransactionNotFound => "TransactionNotFound",
        Error::TransactionConfirmed => "TransactionConfirmed",
        Error::IrreplaceableTransaction => "IrreplaceableTransaction",
        Error::FeeRateTooLow { .. } => "FeeRateTooLow",
        Error::FeeTooLow { .. } => "FeeTooLow",
        Error::MissingKeyOrigin(_) => "MissingKeyOrigin",
        Error::Key(_) => "Key",
        Error::ChecksumMismatch => "ChecksumMismatch",
        Error::SpendingPolicyRequired(_) => "SpendingPolicyRequired",
        Error::InvalidPolicyPathError(_) => "InvalidPolicyPathError",
        Error::Signer(_) => "Signer",
        Error::InvalidProgressValue(_) => "InvalidProgressValue",
        Error::ProgressUpdateError => "ProgressUpdateError",
        Error::InvalidOutpoint(_) => "InvalidOutpoint",
        Error::Descriptor(_) => "Descriptor",
        Error::AddressValidator(_) => "AddressValidator",
        Error::Encode(_) => "Encode",
        Error::Miniscript(_) => "Miniscript",
        Error::Bip32(_) => "Bip32",
        Error::Secp256k1(_) => "Secp256k1",
        Error::Json(_) => "Json",
        Error::Hex(_) => "Hex",
        Error::Psbt(_) => "Psbt",
        Error::Electrum(_) => "Electrum",
        //        Error::Esplora(_) => {}
        //        Error::CompactFilters(_) => {}
        Error::Sled(_) => "Sled",
    }
    .to_string()
}