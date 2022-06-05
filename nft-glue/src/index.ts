import { env } from "process";

import { ecdsaSign } from "secp256k1";
import { Reader, RPC, normalizers } from "ckb-js-toolkit";
import { commons, Indexer } from "@ckb-lumos/lumos";
import {
  HexString,
  Hash,
  Address,
  Cell,
  CellDep,
  Script,
  core,
  utils,
} from "@ckb-lumos/base";
const { CKBHasher } = utils;
import { getConfig, initializeConfig } from "@ckb-lumos/config-manager";
import {
  parseAddress,
  sealTransaction,
  TransactionSkeletonType,
  TransactionSkeleton,
  encodeToAddress,
} from "@ckb-lumos/helpers";

const CKB_RPC = "http://127.0.0.1:8114";

// For simplicity, we hardcode 0.1 CKB as transaction fee here.
const FEE = BigInt(10000000);

env.LUMOS_CONFIG_FILE = env.LUMOS_CONFIG_FILE || "./config.json";
initializeConfig();

export const CONFIG = getConfig();
export const INDEXER = new Indexer(CKB_RPC, "./lumos-indexed-data");
INDEXER.startForever();

export * as ADDRESS from "./addresses";

function buildNftTypeScript(governanceLock: Script): Script {
  const hasher = new CKBHasher();
  hasher.update(
    core.SerializeScript(normalizers.NormalizeScript(governanceLock))
  );
  const hash = hasher.digestHex();
  const NFT = CONFIG.SCRIPTS.NFT;
  if (!NFT) {
    throw new Error("NFT script is not configured!");
  }
  return {
    code_hash: NFT.CODE_HASH,
    hash_type: NFT.HASH_TYPE,
    args: hash,
  };
}

function buildNftCellDep(): CellDep {
  const NFT = CONFIG.SCRIPTS.NFT;
  if (!NFT) {
    throw new Error("NFT script is not configured!");
  }
  return {
    out_point: {
      tx_hash: NFT.TX_HASH,
      index: NFT.INDEX,
    },
    dep_type: NFT.DEP_TYPE,
  };
}

export async function generateNftToken(
  // Since the main purpose of this library is to explain integrations for NFT, we
  // only support gathering capacities from one single wallet. But lumos is designed
  // to be able to treat multiple different wallets as a single unit. If you are
  // interested in this, look for FromInfo in lumos documentation as well as source
  // code.
  fromAddress: Address,
  governanceLock: Script,
  owner: Address
): Promise<TransactionSkeletonType> {
  // Lumos is based heavily on immutable-js library. TransactionSkeleton here is
  // essentially a Record from immutable-js.
  let skeleton = TransactionSkeleton({ cellProvider: INDEXER });
  // First, let's insert a dummy NFT output cell. The dummy cell is exactly the same
  // as a normal cell, except that it uses all zeros as NFT ID. This way we can
  // leverage lumos' utility for providing input cells that accommodate the capacities
  // required by the output cell. When input cells are created, we can then generate
  // correct NFT ID based on the first input cell.
  skeleton = skeleton.update("outputs", (outputs) => {
    return outputs.push({
      cell_output: {
        capacity: "0x" + (BigInt(200) * BigInt(100000000)).toString(16),
        lock: parseAddress(owner),
        type: buildNftTypeScript(governanceLock),
      },
      data:
        "0x0000000000000000000000000000000000000000000000000000000000000000",
      out_point: undefined,
      block_hash: undefined,
    });
  });
  // Here we need some bookkeeping efforts: by default, lumos is designed to generate
  // smaller transactions. Meaning the following cases might happen:
  //
  // 1. Multiple output cells with the same owner might be merged together;
  // 2. Transferring to an address which has input cells in the transaction, might
  // result in the input cell being removed to cancel the transfer operation.
  //
  // While for a normal workflow, those optimizations help us achieve smaller
  // transactions, they might get in the way, since NFT requires special output cell,
  // as well as stable input cell(since the first input cell is used to calculate
  // NFT ID). We do have thought about this case, `fixedEntries` in TransactionSkeleton
  // can be used to mark certain fields as fixed, meaning no further optimizations should
  // alter those components.
  skeleton = skeleton.update("fixedEntries", (fixedEntries) => {
    return fixedEntries.push({
      field: "outputs",
      index: 0,
    });
  });
  // Now let's inject input cells to the transaction so as to provide capacities
  // needed by the newly created input cells.
  skeleton = await commons.secp256k1Blake160.injectCapacity(
    skeleton,
    0,
    fromAddress
  );
  // Now we can generate and fill in the correct NFT token ID.
  const hasher = new CKBHasher();
  let inputCell = skeleton.get("inputs")!.get(0)!;
  hasher.update(
    core.SerializeCellInput(
      normalizers.NormalizeCellInput({
        previous_output: inputCell.out_point,
        since: "0x0",
      })
    )
  );
  hasher.update("0x0000000000000000");
  const nftId = hasher.digestHex();
  skeleton = skeleton.update("outputs", (outputs) => {
    return outputs.update(0, (output) => {
      output.data = nftId;
      return output;
    });
  });
  // The first input must be fixed as well, since it is used to generate NFT ID
  skeleton = skeleton.update("fixedEntries", (fixedEntries) => {
    return fixedEntries.push({
      field: "inputs",
      index: 0,
    });
  });
  // Since we are using the NFT script, we need to include NFT cell dep.
  skeleton = skeleton.update("cellDeps", (cellDeps) => {
    return cellDeps.push(buildNftCellDep());
  });
  // Similar to injectCapacity, lumos also provides helper methods to inject fee:
  skeleton = await commons.secp256k1Blake160.payFee(skeleton, fromAddress, FEE);
  // Finally, let's generate messages that are required in transaction signing phase:
  skeleton = commons.secp256k1Blake160.prepareSigningEntries(skeleton, {
    config: CONFIG,
  });
  return skeleton;
}

// Sign the prepared transaction skeleton, then send it to CKB. Note this is separate
// from lumos related functions, since we believe lumos should not manage private keys
// unless absolutely requested. Developers might have their existing services that handle
// private keys.
export async function signAndSendTransactionSkeleton(
  skeleton: TransactionSkeletonType,
  privateKey: HexString
): Promise<Hash> {
  const signatures = skeleton
    .get("signingEntries")
    .map(({ message }) => {
      const o = ecdsaSign(
        new Uint8Array(new Reader(message).toArrayBuffer()),
        new Uint8Array(new Reader(privateKey).toArrayBuffer())
      );
      const signature = new Uint8Array(65);
      signature.set(o.signature, 0);
      signature.set([o.recid], 64);
      return new Reader(signature.buffer).serializeJson();
    })
    .toArray();
  const tx = sealTransaction(skeleton, signatures);
  const rpc = new RPC(CKB_RPC);
  const hash = await rpc.send_transaction(tx);
  return hash;
}

// List all current live NFT cells
export async function listNftTokens(
  governanceLock: Script
): Promise<Array<Cell>> {
  // NFT cells can be seen as live cells with the requested NFT type script.
  const collector = INDEXER.collector({
    type: buildNftTypeScript(governanceLock),
    data: "any",
  });
  const results = [];
  // For simplicity, we are gathering all cells in a single array. Note this might
  // be slow in case the number of cells grows quite big. You might want to use
  // a stream based solution to fetch only cells you need from the async iterator.
  for await (const cell of collector.collect()) {
    results.push(cell);
  }
  return results;
}

// Transfer NFT token from one user to another. Note that for simplicity, the original
// token sender will pay for the transaction fee. This means the token sender must
// have spare CKB capacities in addition to the NFT token.
export async function transferNftToken(
  nftCell: Cell,
  toAddress: Address
): Promise<TransactionSkeletonType> {
  let skeleton = TransactionSkeleton({ cellProvider: INDEXER });
  // Insert input and output cell for the specified NFT.
  skeleton = skeleton
    .update("inputs", (inputs) => {
      return inputs.push(nftCell);
    })
    .update("outputs", (outputs) => {
      return outputs.push({
        cell_output: {
          capacity: nftCell.cell_output.capacity,
          lock: parseAddress(toAddress),
          type: nftCell.cell_output.type,
        },
        data: nftCell.data,
        out_point: undefined,
        block_hash: undefined,
      });
    });
  // For extra safety, let's add fixedEntries for the input and output NFT cells.
  skeleton = skeleton.update("fixedEntries", (fixedEntries) => {
    return fixedEntries.push(
      {
        field: "inputs",
        index: 0,
      },
      {
        field: "outputs",
        index: 0,
      }
    );
  });
  // Since we are using the NFT script, we need to include NFT cell dep.
  skeleton = skeleton.update("cellDeps", (cellDeps) => {
    return cellDeps.push(buildNftCellDep());
  });
  // For simplicity, token sender would pay for transaction fee.
  skeleton = await commons.secp256k1Blake160.payFee(
    skeleton,
    encodeToAddress(nftCell.cell_output.lock),
    FEE
  );
  // Finally, let's generate messages that are required in transaction signing phase:
  skeleton = commons.secp256k1Blake160.prepareSigningEntries(skeleton, {
    config: CONFIG,
  });
  return skeleton;
}
