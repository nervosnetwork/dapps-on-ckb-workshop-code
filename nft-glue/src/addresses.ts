import {
  HexString,
  Script,
} from "@ckb-lumos/base";
import { getConfig } from "@ckb-lumos/config-manager";

function lockScript(pubkeyHash: HexString): Script {
  return {
    code_hash: getConfig().SCRIPTS!.SECP256K1_BLAKE160!.CODE_HASH,
    hash_type: getConfig().SCRIPTS!.SECP256K1_BLAKE160!.HASH_TYPE,
    args: pubkeyHash,
  };
}

// Note the private keys here are only for demo purposes, please do not use them
// elsewhere!
export const ALICE = {
  PRIVATE_KEY: "0xfd686a48908e8caf97723578bf85f746e1e1d8956cb132f6a2e92e7234a2a245",
  ADDRESS: "ckt1qyqw8yx5hx6vwcm7eqren0d0v39wvfwdhy3q2807pp",
  ARGS: "0xe390d4b9b4c7637ec80799bdaf644ae625cdb922",
  LOCK: lockScript("0xe390d4b9b4c7637ec80799bdaf644ae625cdb922")
};

export const BOB = {
  PRIVATE_KEY: "0x5368b818f59570b5bc078a6a564f098a191dcb8938d95c413be5065fd6c42d32",
  ADDRESS: "ckt1qyqtdhd6s7a44a0s2wc6uk7tcl6duq68nalqvzxw09",
  ARGS: "0xb6ddba87bb5af5f053b1ae5bcbc7f4de03479f7e",
  LOCK: lockScript("0xb6ddba87bb5af5f053b1ae5bcbc7f4de03479f7e"),
};

export const CHARLIE = {
  PRIVATE_KEY: "0xd6013cd867d286ef84cc300ac6546013837df2b06c9f53c83b4c33c2417f6a07",
  ADDRESS: "ckt1qyqxek9w28u3htxhjyqjd7yqzw9nptzaxq2shqlft0",
  ARGS: "0x6cd8ae51f91bacd7910126f880138b30ac5d3015",
  LOCK: lockScript("0x6cd8ae51f91bacd7910126f880138b30ac5d3015"),
};
