import * as fs from "fs";
import { MsgStoreCode } from "@terra-money/feather.js";
import {
  createSignBroadcastCatch,
  getLCD,
  getMnemonicKey,
  getWallet,
  toBase64FromBuffer,
} from "./util";
import { CHAIN_PREFIX } from "./env";
import {
  DISTRIBUTION_CONTRACT,
  MEMBER_CONTRACT,
  THREAD_CONTRACT,
} from "./const";

const lcd = getLCD();

const mnemonicKey1 = getMnemonicKey();
const wallet = getWallet(lcd, mnemonicKey1);
const myAddress = wallet.key.accAddress(CHAIN_PREFIX);

const WASM_CONTRACT_DIRECTORY = "./artifacts";

const storeCode = async () => {
  const storeMemberContractCodeMsg = new MsgStoreCode(
    myAddress,
    toBase64FromBuffer(
      fs.readFileSync(`${WASM_CONTRACT_DIRECTORY}/${MEMBER_CONTRACT}.wasm`)
    )
  );
  const storeDistributionContractCodeMsg = new MsgStoreCode(
    myAddress,
    toBase64FromBuffer(
      fs.readFileSync(
        `${WASM_CONTRACT_DIRECTORY}/${DISTRIBUTION_CONTRACT}.wasm`
      )
    )
  );
  const storeThreadContractCodeMsg = new MsgStoreCode(
    myAddress,
    toBase64FromBuffer(
      fs.readFileSync(`${WASM_CONTRACT_DIRECTORY}/${THREAD_CONTRACT}.wasm`)
    )
  );
  createSignBroadcastCatch(
    wallet,
    [
      storeMemberContractCodeMsg,
      storeDistributionContractCodeMsg,
      storeThreadContractCodeMsg,
    ],
    true
  );
};

storeCode();
