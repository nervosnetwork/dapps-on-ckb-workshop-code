# dapps-on-ckb-workshop-code

This repository contains the accompanying code for the `Dapps on CKB` workshop. The workshop consists of 3 Lectures:

* Lecture 1: Introduction [Slides](https://docs.google.com/presentation/d/10m3jPutAc731S5kHgS8axE565CPiUXNmCSU4eCZgv5Y/edit?usp=sharing) [Video](https://www.youtube.com/watch?v=iVjccs3z5q0)
* Lecture 2: On-chain Scripts with Capsule [Slides](https://docs.google.com/presentation/d/1pl5DtkaoHceC2zZ_OTosXAr98cr80D-8D_5iVEptecY/edit?usp=sharing) [Video](https://www.youtube.com/watch?v=NcN3NiBuJbo)
* Lecture 3: Dapps with Lumos [Slides](https://docs.google.com/presentation/d/1fQKyOrkN8I61a1ZGXCgRczi6T_zWH0aN-IA2SFpdCU4/edit?usp=sharing) [Video](https://www.youtube.com/watch?v=7ob-WL1eWrQ)
* Lecture 3 Part 2: Dapps with Lumos Part 2 [Video](https://www.youtube.com/watch?v=TJ2bnSFUpPQ&t=20s)

In this workshop, we are building an [NFT](https://talk.nervos.org/t/rfc-ckb-nft-draft-spec/4779) script together with a simple set of glue code for interaction. 2 individual components are built for the demo:

* [nft-glue](/nft-glue): [Lumos](https://github.com/nervosnetwork/lumos) powered TypeScript library for interacting with NFT tokens on CKB.
* [nft-validator](/nft-validator): Rust based on-chain validator script for supporting NFT tokens on CKB. [Capsule](https://github.com/nervosnetwork/capsule) is leveraged to simplify script development.


## Build and run that workshop code

## Config and run ckb test net
 1. config genesis.issued_cells
- `ckb init --chain devnet` create ckb config file
- `echo d00c06bfd800d27397002dca6fb0993d5ba6399b4238b2f29ee9deb97593d2bc > alice`
- `echo 63d86723e08f0f813a36ce6aa123bb2289d90680ae1e99d4de8cdb334553f24d > bob`
- `echo a800c82df5461756ae99b5c6677d019c98cc98c7786b80d7b2e77256e46ea1fe > charlie`
- add this for`ckb.toml`
```
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
  2. Add account for ckb-cli
 - `ckb-cli account import --privkey-path alice`
 - `ckb-cli account import --privkey-path bob`
 - `ckb-cli account import --privkey-path Charlie`
  3. Run ckb
 - `ckb run `

## Build and deploy contracts
 - `cd nft-validator`
    1. Test contracts correct
 - `capsule build`
 - `capsule test`
    2. deploy correct
 - `capsule build —release`
 - `capsule build —address ckt1qyqvsv5240xeh85wvnau2eky8pwrhh4jr8ts8vyj37` // save deploy tx_hash
 - `ckb-cli get_transaction —hash <get txhash from previous step ↑>` get and save cell info

## test contract with node 
    1. config
 - `cd ./nft-glue`
 - add NFT scripts to config.json
    2. Build project
 - `yarn`
 - `npx tsc`
    3. Use lib with node 
 - `node --experimental-repl-await`
 - `glue = require("./lib/index")`  // import lib
 - `await glue.listNftTokens(glue.ADDRESS.ALICE.LOCK)` // is []
    4. generate NFT token
 - `skeletion = await glue.generateNftToken(glue.ADDRESS.ALICE.ADDRESS, glue.ADDRESS.ALICE.LOCK, glue.ADDRESS.ALICE.ADDRESS)` 
 - `await glue.signAndSendTransactionSkeleton(skeletion, glue.ADDRESS.ALICE.PRIVATE_KEY)`
