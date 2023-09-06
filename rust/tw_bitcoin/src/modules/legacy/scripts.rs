use crate::aliases::*;
use bitcoin::PublicKey;
use std::ffi::{c_char, CStr};
use tw_memory::ffi::c_byte_array::CByteArray;
use tw_memory::ffi::c_byte_array_ref::CByteArrayRef;
use tw_misc::try_or_else;
use tw_proto::Bitcoin::Proto as LegacyProto;
use tw_proto::BitcoinV2::Proto;

#[no_mangle]
#[deprecated]
// Builds the P2PKH scriptPubkey.
pub unsafe extern "C" fn tw_build_p2pkh_script(
    _satoshis: i64,
    pubkey: *const u8,
    pubkey_len: usize,
) -> CByteArray {
    // Convert Recipient
    let slice = try_or_else!(
        CByteArrayRef::new(pubkey, pubkey_len).as_slice(),
        CByteArray::null
    );
    let recipient = try_or_else!(PublicKey::from_slice(slice), CByteArray::null);

    let output = Proto::Output {
        value: _satoshis as u64,
        to_recipient: ProtoOutputRecipient::builder(Proto::mod_Output::OutputBuilder {
            variant: ProtoOutputBuilder::p2pkh(Proto::ToPublicKeyOrHash {
                to_address: ProtoPubkeyOrHash::pubkey(recipient.to_bytes().into()),
            }),
        }),
    };

    let res = try_or_else!(
        crate::modules::transactions::OutputBuilder::utxo_from_proto(&output),
        CByteArray::null
    );

    // Prepare and serialize protobuf structure.
    let proto = LegacyProto::TransactionOutput {
        value: res.value as i64,
        script: res.script_pubkey,
        spendingScript: Default::default(),
    };

    let serialized = tw_proto::serialize(&proto).expect("failed to serialized transaction output");
    CByteArray::from(serialized)
}

#[no_mangle]
#[deprecated]
// Builds the P2WPKH scriptPubkey.
pub unsafe extern "C" fn tw_build_p2wpkh_script(
    _satoshis: i64,
    pubkey: *const u8,
    pubkey_len: usize,
) -> CByteArray {
    // Convert Recipient
    let slice = try_or_else!(
        CByteArrayRef::new(pubkey, pubkey_len).as_slice(),
        CByteArray::null
    );

    let recipient = try_or_else!(PublicKey::from_slice(slice), CByteArray::null);

    let output = Proto::Output {
        value: _satoshis as u64,
        to_recipient: ProtoOutputRecipient::builder(Proto::mod_Output::OutputBuilder {
            variant: ProtoOutputBuilder::p2wpkh(Proto::ToPublicKeyOrHash {
                to_address: ProtoPubkeyOrHash::pubkey(recipient.to_bytes().into()),
            }),
        }),
    };

    let res = try_or_else!(
        crate::modules::transactions::OutputBuilder::utxo_from_proto(&output),
        CByteArray::null
    );

    // Prepare and serialize protobuf structure.
    let proto = LegacyProto::TransactionOutput {
        value: res.value as i64,
        script: res.script_pubkey,
        spendingScript: Default::default(),
    };

    let serialized = tw_proto::serialize(&proto).expect("failed to serialized transaction output");
    CByteArray::from(serialized)
}

#[no_mangle]
#[deprecated]
// Builds the P2TR key-path scriptPubkey.
pub unsafe extern "C" fn tw_build_p2tr_key_path_script(
    _satoshis: i64,
    pubkey: *const u8,
    pubkey_len: usize,
) -> CByteArray {
    // Convert Recipient
    let slice = try_or_else!(
        CByteArrayRef::new(pubkey, pubkey_len).as_slice(),
        CByteArray::null
    );
    let recipient = try_or_else!(PublicKey::from_slice(slice), CByteArray::null);

    let output = Proto::Output {
        value: _satoshis as u64,
        to_recipient: ProtoOutputRecipient::builder(Proto::mod_Output::OutputBuilder {
            variant: ProtoOutputBuilder::p2tr_key_path(recipient.to_bytes().into()),
        }),
    };

    let res = try_or_else!(
        crate::modules::transactions::OutputBuilder::utxo_from_proto(&output),
        CByteArray::null
    );

    // Prepare and serialize protobuf structure.
    let proto = LegacyProto::TransactionOutput {
        value: res.value as i64,
        script: res.script_pubkey,
        spendingScript: Default::default(),
    };

    let serialized = tw_proto::serialize(&proto).expect("failed to serialized transaction output");
    CByteArray::from(serialized)
}

#[no_mangle]
#[deprecated]
// Builds the Ordinals inscripton for BRC20 transfer.
pub unsafe extern "C" fn tw_build_brc20_transfer_inscription(
    // The 4-byte ticker.
    ticker: *const c_char,
    value: u64,
    _satoshis: i64,
    pubkey: *const u8,
    pubkey_len: usize,
) -> CByteArray {
    // Convert Recipient
    let slice = try_or_else!(
        CByteArrayRef::new(pubkey, pubkey_len).as_slice(),
        CByteArray::null
    );

    let recipient = try_or_else!(PublicKey::from_slice(slice), CByteArray::null);

    // Convert ticket.
    let ticker = match CStr::from_ptr(ticker).to_str() {
        Ok(input) => input,
        Err(_) => return CByteArray::null(),
    };

    let output = Proto::Output {
        value: _satoshis as u64,
        to_recipient: ProtoOutputRecipient::builder(Proto::mod_Output::OutputBuilder {
            variant: ProtoOutputBuilder::brc20_inscribe(
                Proto::mod_Output::OutputBrc20Inscription {
                    inscribe_to: recipient.to_bytes().into(),
                    ticker: ticker.into(),
                    transfer_amount: value,
                },
            ),
        }),
    };

    let res = try_or_else!(
        crate::modules::transactions::OutputBuilder::utxo_from_proto(&output),
        CByteArray::null
    );

    // Prepare and serialize protobuf structure.
    let proto = LegacyProto::TransactionOutput {
        value: res.value as i64,
        script: res.script_pubkey,
        spendingScript: res.taproot_payload,
    };

    let serialized = tw_proto::serialize(&proto).expect("failed to serialized transaction output");
    CByteArray::from(serialized)
}

#[no_mangle]
#[deprecated]
// Builds the Ordinals inscripton for BRC20 transfer.
pub unsafe extern "C" fn tw_bitcoin_build_nft_inscription(
    mime_type: *const c_char,
    payload: *const u8,
    payload_len: usize,
    _satoshis: i64,
    pubkey: *const u8,
    pubkey_len: usize,
) -> CByteArray {
    // Convert mimeType.
    let mime_type = match CStr::from_ptr(mime_type).to_str() {
        Ok(input) => input,
        Err(_) => return CByteArray::null(),
    };

    // Convert data to inscribe.
    let payload = try_or_else!(
        CByteArrayRef::new(payload, payload_len).as_slice(),
        CByteArray::null
    );

    // Convert Recipient.
    let slice = try_or_else!(
        CByteArrayRef::new(pubkey, pubkey_len).as_slice(),
        CByteArray::null
    );

    let recipient = try_or_else!(PublicKey::from_slice(slice), CByteArray::null);

    // Inscribe NFT data.
    let output = Proto::Output {
        value: _satoshis as u64,
        to_recipient: ProtoOutputRecipient::builder(Proto::mod_Output::OutputBuilder {
            variant: ProtoOutputBuilder::ordinal_inscribe(
                Proto::mod_Output::OutputOrdinalInscription {
                    inscribe_to: recipient.to_bytes().into(),
                    mime_type: mime_type.into(),
                    payload: payload.into(),
                },
            ),
        }),
    };

    let res = try_or_else!(
        crate::modules::transactions::OutputBuilder::utxo_from_proto(&output),
        CByteArray::null
    );

    // Prepare and serialize protobuf structure.
    let proto = LegacyProto::TransactionOutput {
        value: res.value as i64,
        script: res.script_pubkey,
        spendingScript: res.taproot_payload,
    };

    let serialized = tw_proto::serialize(&proto).expect("failed to serialized transaction output");
    CByteArray::from(serialized)
}
