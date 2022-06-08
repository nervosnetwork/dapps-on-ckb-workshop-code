# dapps-on-ckb-workshop-code

This repository contains the accompanying code for the `Dapps on CKB` workshop. The workshop consists of 3 Lectures:

* Lecture 1: Introduction [Slides](https://docs.google.com/presentation/d/10m3jPutAc731S5kHgS8axE565CPiUXNmCSU4eCZgv5Y/edit?usp=sharing) [Video](https://www.youtube.com/watch?v=iVjccs3z5q0)
* Lecture 2: On-chain Scripts with Capsule [Slides](https://docs.google.com/presentation/d/1pl5DtkaoHceC2zZ_OTosXAr98cr80D-8D_5iVEptecY/edit?usp=sharing) [Video](https://www.youtube.com/watch?v=NcN3NiBuJbo)
* Lecture 3: Dapps with Lumos [Slides](https://docs.google.com/presentation/d/1fQKyOrkN8I61a1ZGXCgRczi6T_zWH0aN-IA2SFpdCU4/edit?usp=sharing) [Video](https://www.youtube.com/watch?v=7ob-WL1eWrQ)
* Lecture 3 Part 2: Dapps with Lumos Part 2 [Video](https://www.youtube.com/watch?v=TJ2bnSFUpPQ&t=20s)

In this workshop, we are building an [NFT](https://talk.nervos.org/t/rfc-ckb-nft-draft-spec/4779) script together with a simple set of glue code for interaction. 2 individual components are built for the demo:

* [nft-glue](/nft-glue): [Lumos](https://github.com/nervosnetwork/lumos) powered TypeScript library for interacting with NFT tokens on CKB.
* [nft-validator](/nft-validator): Rust based on-chain validator script for supporting NFT tokens on CKB. [Capsule](https://github.com/nervosnetwork/capsule) is leveraged to simplify script development.


# Build and run workshop code

## run ckb dev net

```
# create ckb config file
$ ckb init --chain devnet

# create account file 
$ echo d00c06bfd800d27397002dca6fb0993d5ba6399b4238b2f29ee9deb97593d2bc > alice
$ echo 63d86723e08f0f813a36ce6aa123bb2289d90680ae1e99d4de8cdb334553f24d > bob
$ echo a800c82df5461756ae99b5c6677d019c98cc98c7786b80d7b2e77256e46ea1fe > charlie
```

edit dev.toml

``` yaml
# alice
# issue for random generated private key: d00c06bfd800d27397002dca6fb0993d5ba6399b4238b2f29ee9deb97593d2bc
# address: ckt1qyqvsv5240xeh85wvnau2eky8pwrhh4jr8ts8vyj37
[[genesis.issued_cells]]
capacity = 20_000_000_000_00000000
lock.code_hash = "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8"
lock.args = "0xc8328aabcd9b9e8e64fbc566c4385c3bdeb219d7"
lock.hash_type = "type"

# bob
# issue for random generated private key: 63d86723e08f0f813a36ce6aa123bb2289d90680ae1e99d4de8cdb334553f24d
# address: ckt1qyqywrwdchjyqeysjegpzw38fvandtktdhrs0zaxl4
[[genesis.issued_cells]]
capacity = 20_000_000_000_00000000
lock.code_hash = "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8"
lock.args = "0x470dcdc5e44064909650113a274b3b36aecb6dc7"
lock.hash_type = "type"

# charlie
# issue for random generated private key: a800c82df5461756ae99b5c6677d019c98cc98c7786b80d7b2e77256e46ea1fe
# address: ckt1qyqyph8v9mclls35p6snlaxajeca97tc062sa5gahk
[[genesis.issued_cells]]
capacity = 20_000_000_000_00000000
lock.code_hash = "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8"
lock.args = "0x40dcec2ef1ffc2340ea13ff4dd9671d2f9787e95"
lock.hash_type = "type"
```

Add account for ckb-cli
``` 
$ ckb-cli account import --privkey-path alice
$ ckb-cli account import --privkey-path bob
$ ckb-cli account import --privkey-path charlie
```

Run ckb
```
$ ckb run
```

test contracts
``` 
$ cd nft-validator
$ capsule build
$ capsule test
```

Build and deploy contracts
```
$ capsule build —release

$ capsule deploy —address ckt1qyqvsv5240xeh85wvnau2eky8pwrhh4jr8ts8vyj37 --fee 0.01

# get and save cell info
$ ckb-cli rpc get_transaction —hash <get tx_hash from previous step ↑>` 
```

## test contract with node 

## test contract with node 


```
# Modify the TX_HASH field of SCRIPTS.SECP256K1_BLAKE160 in config.json file to the current configuration on the dev chain.

# Get the current configuration on the chain
$ ckb-cli rpc get_block_by_number —number 0
# Find the cell_deps.hash field from the returned value
```


edit nft-glue/config.json
``` json
{
  …
  "SCRIPTS": {
    "SECP256K1_BLAKE160": {
      …
      "TX_HASH": "<tx_hash from ckb-cli rpc get_block_by_number —number 0>",
    },
    "SECP256K1_BLAKE160_MULTISIG": {
      …
      "TX_HASH": ""<tx_hash from ckb-cli rpc get_block_by_number —number 0>",
    },
    "NFT": {
      "CODE_HASH": "<data_hash from ./nft-validator/migrations/dev/{timestamp}.json>",
      "HASH_TYPE": "data",
      "TX_HASH": "<tx_hash from ./nft-validator/migrations/dev/{timestamp}.json>",
      "INDEX": "0x0",
      "DEP_TYPE": "code"
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

node> await glue.signAndSendTransactionSkeleton(skeletion, glue.ADDRESS.ALICE.PRIVATE_KEY)
```
