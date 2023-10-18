import { MsgInstantiateContract } from "@terra-money/feather.js";
import {
  createSignBroadcastCatch,
  getLCD,
  getMnemonicKey,
  getWallet,
} from "./util";
import {
  CHAIN_PREFIX,
  MEMBER_CONTRACT_ADDR,
  THREAD_CONTRACT_CODE_ID,
  THREAD_CONTRACT_VERSION,
} from "./env";
import { THREAD_CONTRACT } from "./const";

const lcd = getLCD();

const mnemonicKey1 = getMnemonicKey();
const wallet = getWallet(lcd, mnemonicKey1);
const myAddress = wallet.key.accAddress(CHAIN_PREFIX);

const init = async () => {
  const threadContractInitMsg = {
    member_contract_addr: MEMBER_CONTRACT_ADDR!,
  };
  const initThreadContractCodeMsg = new MsgInstantiateContract(
    myAddress,
    myAddress,
    parseInt(THREAD_CONTRACT_CODE_ID!),
    threadContractInitMsg,
    undefined,
    `${THREAD_CONTRACT}-${THREAD_CONTRACT_VERSION}`
  );

  createSignBroadcastCatch(wallet, [initThreadContractCodeMsg], true);
};

init();
