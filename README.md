# Simple CTV+CSFS 🥪+🥞=🪄

A very simple example of how you can use OP_CHECKTEMPLATEVERIFY [(BIP-119)][bip119] with OP_CHECKSIGFROMSTACK [(BIP-348)][bip348] to create a transaction that can be pre-signed without committing to a specific input

> When combined with BIP 119 (OP_CHECKTEMPLATEVERIFY/CTV), OP_CHECKSIGFROMSTACK (CSFS) can be used to implement Lightning Symmetry channels. The construction OP_CHECKTEMPLATEVERIFY <pubkey> OP_CHECKSIGFROMSTACK with a spend stack containing the CTV hash and a signature for it is logically equivalent to <bip118_pubkey> OP_CHECKSIG and a signature over SIGHASH_ALL\|SIGHASH_ANYPREVOUTANYSCRIPT. The OP_CHECKSIGFROMSTACK construction is 8 vBytes larger.  
>
> — [bip-0348](https://github.com/bitcoin/bips/blob/master/bip-0348.md)


[bip119]: https://github.com/bitcoin/bips/blob/master/bip-0119.mediawiki
[bip348]: https://github.com/bitcoin/bips/blob/master/bip-0348.md

## more info 

https://rubin.io/blog/2021/07/02/covenants/

https://bitcoin.stackexchange.com/questions/111497/how-do-eltoo-channel-constructions-using-anyprevout-compare-to-those-using-ctv-a

## mutinynet tx

https://mutinynet.com/tx/86990eb12f4d86bae21ec2e0e6eac6298d696d84c710c3e0a813c76bc3f79690

## raw transaction

020000000001015a7213c397bbe88524623cc0552f26b5694224671f2d901685dcae0b64c008a00100000000fdffffff02391b0000000000001600146a8f30e42f81d23c6e24f34c0ecad822b757e4900000000000000000106a0e6374762b637366733d6d61676963034076411c14bbe3f71d59f80e357b4825251852ecf9b0f621a775e11a537f3cf92fa54a8a40f72632105f11e448eed8dfebf712ecc24d367c052fad9a4b071077b744200f6da9d969996031de44e950469ff68bec08878faef1e9317289505661aa5f85b32099eedb11f699d4408127d928af8fa5b552c309a708559b956775fd7db4e4c3e6cc21c148755882774d45eaaddb6a628b291796cc3a21680a2a983cb7fbc6dbc416957a00000000

## decoded transaction 

```bash
{
  "txid": "86990eb12f4d86bae21ec2e0e6eac6298d696d84c710c3e0a813c76bc3f79690",
  "hash": "87b887228ab1643281435e1461b6cd2523b67d904b56e9098f7067a21da38a38",
  "version": 2,
  "size": 278,
  "vsize": 150,
  "weight": 599,
  "locktime": 0,
  "vin": [
    {
      "txid": "a008c0640baedc8516902d1f67244269b5262f55c03c622485e8bb97c313725a",
      "vout": 1,
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "txinwitness": [
        "76411c14bbe3f71d59f80e357b4825251852ecf9b0f621a775e11a537f3cf92fa54a8a40f72632105f11e448eed8dfebf712ecc24d367c052fad9a4b071077b7",
        "200f6da9d969996031de44e950469ff68bec08878faef1e9317289505661aa5f85b32099eedb11f699d4408127d928af8fa5b552c309a708559b956775fd7db4e4c3e6cc",
        "c148755882774d45eaaddb6a628b291796cc3a21680a2a983cb7fbc6dbc416957a"
      ],
      "sequence": 4294967293
    }
  ],
  "vout": [
    {
      "value": 0.00006969,
      "n": 0,
      "scriptPubKey": {
        "asm": "0 6a8f30e42f81d23c6e24f34c0ecad822b757e490",
        "desc": "addr(bcrt1qd28npep0s8frcm3y7dxqajkcy2m40eysrkawj9)#mcfu834t",
        "hex": "00146a8f30e42f81d23c6e24f34c0ecad822b757e490",
        "address": "bcrt1qd28npep0s8frcm3y7dxqajkcy2m40eysrkawj9",
        "type": "witness_v0_keyhash"
      }
    },
    {
      "value": 0.00000000,
      "n": 1,
      "scriptPubKey": {
        "asm": "OP_RETURN 6374762b637366733d6d61676963",
        "desc": "raw(6a0e6374762b637366733d6d61676963)#u8kauxmf",
        "hex": "6a0e6374762b637366733d6d61676963",
        "type": "nulldata"
      }
    }
  ]
}
```

![alt text](image.png)

## How to Run

This application requires a two-step process to generate and spend from the contract.

### 1. Get the Contract Address

First, generate the contract address. You will need a destination address on the desired network (`regtest` or `signet`).

```bash
# For regtest
cargo run -- --network regtest --to-address <your_regtest_address>

# For signet (default)
cargo run -- --to-address <your_signet_address>
```
This will output the `CTV+CSFS contract address`.

### 2. Create the Transaction

Next, send funds to the contract address you just generated. Once the funding transaction is confirmed, you can create the final spending transaction.

```bash
# Provide the same address and the txid of your funding transaction
cargo run -- --network <network> --to-address <your_address> --txid <funding_txid>
```

This will output the raw transaction hex, which you can then broadcast using your node.
