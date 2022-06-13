# dapps-on-ckb-workshop-code

This repository contains the accompanying code for the `Dapps on CKB` workshop. The workshop consists of 3 Lectures:

* Lecture 1: Introduction [Slides](https://docs.google.com/presentation/d/10m3jPutAc731S5kHgS8axE565CPiUXNmCSU4eCZgv5Y/edit?usp=sharing) [Video](https://www.youtube.com/watch?v=iVjccs3z5q0)
* Lecture 2: On-chain Scripts with Capsule [Slides](https://docs.google.com/presentation/d/1pl5DtkaoHceC2zZ_OTosXAr98cr80D-8D_5iVEptecY/edit?usp=sharing) [Video](https://www.youtube.com/watch?v=NcN3NiBuJbo)
* Lecture 3: Dapps with Lumos [Slides](https://docs.google.com/presentation/d/1fQKyOrkN8I61a1ZGXCgRczi6T_zWH0aN-IA2SFpdCU4/edit?usp=sharing) [Video](https://www.youtube.com/watch?v=7ob-WL1eWrQ)
* Lecture 3 Part 2: Dapps with Lumos Part 2 [Video](https://www.youtube.com/watch?v=TJ2bnSFUpPQ&t=20s)

In this workshop, we are building an [NFT](https://talk.nervos.org/t/rfc-ckb-nft-draft-spec/4779) script together with a simple set of glue code for interaction. 2 individual components are built for the demo:

* [nft-glue](/nft-glue): [Lumos](https://github.com/nervosnetwork/lumos) powered TypeScript library for interacting with NFT tokens on CKB.
* [nft-validator](/nft-validator): Rust based on-chain validator script for supporting NFT tokens on CKB. [Capsule](https://github.com/nervosnetwork/capsule) is leveraged to simplify script development.


# Build & Run Workshop

## Start CKB Devnet

```
# create ckb config file
$ ckb init --chain devnet

# create account file 
$ echo 0xfd686a48908e8caf97723578bf85f746e1e1d8956cb132f6a2e92e7234a2a245 > alice
$ echo 0x5368b818f59570b5bc078a6a564f098a191dcb8938d95c413be5065fd6c42d32 > bob
$ echo 0xd6013cd867d286ef84cc300ac6546013837df2b06c9f53c83b4c33c2417f6a07 > charlie
```

### Edit `dev.toml`

<details>
   <summary>dev.toml</summary>
``` yaml
# alice
# issue for random generated private key:0xfd686a48908e8caf97723578bf85f746e1e1d8956cb132f6a2e92e7234a2a245
# address: ckt1qyqw8yx5hx6vwcm7eqren0d0v39wvfwdhy3q2807pp
[[genesis.issued_cells]]
capacity = 20_000_000_000_00000000
lock.code_hash = "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8"
lock.args = "0xe390d4b9b4c7637ec80799bdaf644ae625cdb922"
lock.hash_type = "type"

# bob
# issue for random generated private key:0x5368b818f59570b5bc078a6a564f098a191dcb8938d95c413be5065fd6c42d32
# address: ckt1qyqtdhd6s7a44a0s2wc6uk7tcl6duq68nalqvzxw09
[[genesis.issued_cells]]
capacity = 20_000_000_000_00000000
lock.code_hash = "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8"
lock.args = "0xb6ddba87bb5af5f053b1ae5bcbc7f4de03479f7e"
lock.hash_type = "type"

# charlie
# issue for random generated private key:0xd6013cd867d286ef84cc300ac6546013837df2b06c9f53c83b4c33c2417f6a07
# address: ckt1qyqxek9w28u3htxhjyqjd7yqzw9nptzaxq2shqlft0
[[genesis.issued_cells]]
capacity = 20_000_000_000_00000000
lock.code_hash = "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8"
lock.args = "0x6cd8ae51f91bacd7910126f880138b30ac5d3015"
lock.hash_type = "type"
```
</details>

### Importing Account to ckb-cli
``` 
$ ckb-cli account import --privkey-path alice
$ ckb-cli account import --privkey-path bob
$ ckb-cli account import --privkey-path charlie
```

### Start CKB & CKB-Indexer
```
$ ckb run
```

### Building & Testing Contracts
``` 
$ cd nft-validator
$ capsule build
$ capsule test
```

### Deploy Contracts to Devnet
```
$ capsule build --release

$ capsule deploy --address ckt1qyqw8yx5hx6vwcm7eqren0d0v39wvfwdhy3q2807pp --fee 0.01

# get and save cell info
$ ckb-cli rpc get_transaction --hash <get tx_hash from previous step ↑>` 
```

## Interacting with Contract via Lumos 


```
# Modify the TX_HASH field of SCRIPTS.SECP256K1_BLAKE160 in config.json file to the current configuration on the dev chain.

# Get the current configuration on the chain
$ ckb-cli rpc get_block_by_number --number 0
# Find the transactions[1].hash field from the returned value
```


### Creating the Config
``` json
{
   "SCRIPTS": {
      "SECP256K1_BLAKE160": {
         "TX_HASH": "<tx_hash from ckb-cli rpc get_block_by_number —number 0>",
         "<other fields>": "<...>"
      },
      "SECP256K1_BLAKE160_MULTISIG": {
         "TX_HASH": "<tx_hash from ckb-cli rpc get_block_by_number —number 0>",
         "<other fields>": "<...>"
      },
      "NFT": {
         "CODE_HASH": "<data_hash from ./nft-validator/migrations/dev/{timestamp}.json>",
         "HASH_TYPE": "data",
         "TX_HASH": "<tx_hash from ./nft-validator/migrations/dev/{timestamp}.json>",
         "INDEX": "0x0",
         "DEP_TYPE": "code"
      }
   }
}
```


build node project
```
$ cd ./nft-glue
$ yarn
$ npx tsc
```

use lib
```
$ node --experimental-repl-await

# import lib
node> glue = require("./lib/index")

# is []
node> await glue.listNftTokens(glue.ADDRESS.ALICE.LOCK)

# generate NFT token 
node> skeletion = await glue.generateNftToken(glue.ADDRESS.ALICE.ADDRESS, glue.ADDRESS.ALICE.LOCK, glue.ADDRESS.ALICE.ADDRESS)

# mint a NFT for Alice
node> await glue.signAndSendTransactionSkeleton(skeletion, glue.ADDRESS.ALICE.PRIVATE_KEY)

# wait a moment, and we can find minted nft
node> await glue.listNftTokens(glue.ADDRESS.ALICE.LOCK)
```
