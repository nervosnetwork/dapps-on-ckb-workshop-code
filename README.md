# dapps-on-ckb-workshop-code

This repository contains the accompanying code for the `Dapps on CKB` workshop. The workshop consists of 3 parts:

* Part 1: Introduction [Slides](https://docs.google.com/presentation/d/10m3jPutAc731S5kHgS8axE565CPiUXNmCSU4eCZgv5Y/edit?usp=sharing) [Video](https://www.youtube.com/watch?v=iVjccs3z5q0)
* Part 2: On-chain Scripts with Capsule [Slides](https://docs.google.com/presentation/d/1pl5DtkaoHceC2zZ_OTosXAr98cr80D-8D_5iVEptecY/edit?usp=sharing) [Video](https://www.youtube.com/watch?v=NcN3NiBuJbo)
* Part 3: Dapps with Lumos [Slides](https://docs.google.com/presentation/d/1fQKyOrkN8I61a1ZGXCgRczi6T_zWH0aN-IA2SFpdCU4/edit?usp=sharing) [Video](https://www.youtube.com/watch?v=7ob-WL1eWrQ)

In this workshop, we are building an [NFT](https://talk.nervos.org/t/rfc-ckb-nft-draft-spec/4779) script together with a simple set of glue code for interaction. 2 individual components are built for the demo:

* [nft-glue](/nft-glue): [Lumos](https://github.com/nervosnetwork/lumos) powered TypeScript library for interacting with NFT tokens on CKB.
* [nft-validator](/nft-validator): Rust based on-chain validator script for supporting NFT tokens on CKB. [Capsule](https://github.com/nervosnetwork/capsule) is leveraged to simplify script development.
