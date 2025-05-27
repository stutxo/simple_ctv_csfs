use std::str::FromStr;

use ctv_csfs_scripts::{create_ctv_csfs_address, ctv_hash, spend_ctv_csfs};

use bitcoin::{
    absolute,
    consensus::encode::serialize_hex,
    key::{Keypair, Secp256k1},
    opcodes::all::OP_RETURN,
    script::{Builder, PushBytesBuf},
    secp256k1::{Message, SecretKey},
    transaction::{self},
    Address, Amount, OutPoint, Sequence, Transaction, TxIn, TxOut, Txid,
};

const NETWORK: bitcoin::Network = bitcoin::Network::Signet;

mod ctv_csfs_scripts;

const SPEND_AMOUNT: Amount = Amount::from_sat(6969);

fn main() {
    let secp = Secp256k1::new();

    let key = "7457e13133b7e90bcf6caa4165a14833153fd6164be95fc4a22829c26455a10a";

    let secret_key = SecretKey::from_str(&key).expect("Failed to create SecretKey");

    let key_pair = Keypair::from_secret_key(&secp, &secret_key);
    let pubkey = key_pair.x_only_public_key().0;

    let op_return_data = "ctv+csfs=magic";
    let mut op_return_bytes = PushBytesBuf::new();
    op_return_bytes
        .extend_from_slice(op_return_data.as_bytes())
        .unwrap();

    let op_return_script = Builder::new()
        .push_opcode(OP_RETURN)
        .push_slice(op_return_bytes)
        .into_script();

    let mutiny_faucet_address = "tb1qd28npep0s8frcm3y7dxqajkcy2m40eysplyr9v";
    let ctv_spend_to_address = Address::from_str(mutiny_faucet_address)
        .unwrap()
        .require_network(NETWORK)
        .unwrap();

    let ctv_tx_out = [
        TxOut {
            value: SPEND_AMOUNT,
            script_pubkey: ctv_spend_to_address.script_pubkey(),
        },
        TxOut {
            value: Amount::from_sat(0),
            script_pubkey: op_return_script,
        },
    ];

    //calculate ctv hash
    let ctv_hash = ctv_hash(&ctv_tx_out, None, None);

    // create ctv+csfs contract address
    let tr_spend_info = create_ctv_csfs_address(ctv_hash, pubkey).unwrap();
    let contract_address = Address::p2tr_tweaked(tr_spend_info.output_key(), NETWORK);
    println!("CTV+CSFS contract address: {}", contract_address);

    // create the signature before we even know the inputs!!!
    let msg = Message::from_digest_slice(&ctv_hash).unwrap();
    let signature: Vec<u8> = secp
        .sign_schnorr_no_aux_rand(&msg, &key_pair)
        .as_ref()
        .to_vec();

    // we dont need to know this info before signing the transaction! magic!
    let txid =
        Txid::from_str("a008c0640baedc8516902d1f67244269b5262f55c03c622485e8bb97c313725a").unwrap();

    let inputs = vec![TxIn {
        previous_output: OutPoint {
            txid: txid,
            vout: 1,
        },
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        ..Default::default()
    }];

    let tx = Transaction {
        version: transaction::Version(2),
        lock_time: absolute::LockTime::ZERO,
        input: inputs,
        output: ctv_tx_out.to_vec(),
    };

    let tx = spend_ctv_csfs(tx, tr_spend_info, ctv_hash, pubkey, signature);

    println!("\ntxid: {}", tx.compute_txid());

    let serialized_tx = serialize_hex(&tx);
    println!("\nraw transaction: {}", serialized_tx);
}
