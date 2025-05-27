use std::str::FromStr;

use bitcoin::{
    consensus::Encodable,
    hashes::{sha256, Hash},
    io::Error,
    key::Secp256k1,
    opcodes::all::{OP_NOP4, OP_RETURN_204},
    script::Builder,
    taproot::{LeafVersion, TaprootBuilder, TaprootSpendInfo},
    Opcode, ScriptBuf, Sequence, Transaction, TxIn, TxOut, XOnlyPublicKey,
};

const OP_CHECKTEMPLATEVERIFY: Opcode = OP_NOP4;
const OP_CHECKSIGFROMSTACK: Opcode = OP_RETURN_204;

pub fn ctv_csfs_script(ctv_hash: [u8; 32], pubkey: XOnlyPublicKey) -> ScriptBuf {
    Builder::new()
        .push_slice(ctv_hash)
        .push_opcode(OP_CHECKTEMPLATEVERIFY)
        .push_x_only_key(&pubkey)
        .push_opcode(OP_CHECKSIGFROMSTACK)
        .into_script()
}

// CTV hash used to spend 1 input
pub fn ctv_hash(outputs: &[TxOut], timeout: Option<u32>, maybe_txin: Option<&TxIn>) -> [u8; 32] {
    let mut buffer = Vec::new();
    buffer.extend(2_i32.to_le_bytes()); // version
    buffer.extend(0_i32.to_le_bytes()); // locktime

    if let Some(txin) = maybe_txin {
        let script_sigs_hash = sha256::Hash::hash(&txin.script_sig.to_bytes());
        buffer.extend(script_sigs_hash.to_byte_array()); //scriptSigs hash (if any non-null scriptSigs)
    }

    buffer.extend(1_u32.to_le_bytes()); // number of inputs

    let seq = if let Some(timeout_value) = timeout {
        sha256::Hash::hash(&Sequence(timeout_value).0.to_le_bytes())
    } else {
        sha256::Hash::hash(&Sequence::ENABLE_RBF_NO_LOCKTIME.0.to_le_bytes())
    };

    buffer.extend(seq.to_byte_array()); // sequences hash

    let outputs_len = outputs.len() as u32;
    buffer.extend(outputs_len.to_le_bytes()); // number of outputs

    let mut output_bytes: Vec<u8> = Vec::new();
    for o in outputs {
        o.consensus_encode(&mut output_bytes).unwrap();
    }
    buffer.extend(sha256::Hash::hash(&output_bytes).to_byte_array()); // outputs hash

    buffer.extend(0_u32.to_le_bytes()); // inputs index

    let hash = sha256::Hash::hash(&buffer);
    hash.to_byte_array()
}

pub fn create_ctv_csfs_address(
    ctv_hash: [u8; 32],
    pubkey: XOnlyPublicKey,
) -> Result<TaprootSpendInfo, Error> {
    let secp = Secp256k1::new();

    //"Nothing Up My Sleeve" public key
    let nums_pubkey = XOnlyPublicKey::from_str(
        "48755882774d45eaaddb6a628b291796cc3a21680a2a983cb7fbc6dbc416957a",
    )
    .unwrap();

    let script = ctv_csfs_script(ctv_hash, pubkey);

    let taproot_spend_info = TaprootBuilder::new()
        .add_leaf(0, script)
        .unwrap()
        .finalize(&secp, nums_pubkey)
        .unwrap();

    Ok(taproot_spend_info)
}

pub fn spend_ctv_csfs(
    mut tx: Transaction,
    tap_info: TaprootSpendInfo,
    ctv_hash: [u8; 32],
    pubkey: XOnlyPublicKey,
    signature: Vec<u8>,
) -> Transaction {
    let ctv_script = ctv_csfs_script(ctv_hash, pubkey);
    let script_ver = (ctv_script.clone(), LeafVersion::TapScript);
    let ctrl = tap_info.control_block(&script_ver).unwrap().serialize();

    tx.input[0].witness.push(signature);
    tx.input[0].witness.push(ctv_script.into_bytes());
    tx.input[0].witness.push(ctrl);

    tx
}
