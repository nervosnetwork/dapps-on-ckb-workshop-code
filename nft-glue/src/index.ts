import { env } from "process";

import { getConfig, initializeConfig } from "@ckb-lumos/config-manager";
import { Indexer } from "@ckb-lumos/indexer";

const CKB_RPC = "http://127.0.0.1:8114";

// For simplicity, we hardcode 0.1 CKB as transaction fee here.
const FEE = BigInt(10000000);

env.LUMOS_CONFIG_FILE = env.LUMOS_CONFIG_FILE || "./config.json";
initializeConfig();

export const CONFIG = getConfig();
export const INDEXER = new Indexer(CKB_RPC, "./lumos-indexed-data");
INDEXER.startForever();
