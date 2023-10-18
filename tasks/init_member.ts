import { MsgInstantiateContract } from "@terra-money/feather.js";
import {
  createSignBroadcastCatch,
  getLCD,
  getMnemonicKey,
  getWallet,
} from "./util";
import {
  CHAIN_PREFIX,
  MEMBER_CONTRACT_CODE_ID,
  MEMBER_CONTRACT_VERSION,
} from "./env";
import { MEMBER_CONTRACT } from "./const";

const lcd = getLCD();

const mnemonicKey1 = getMnemonicKey();
const wallet = getWallet(lcd, mnemonicKey1);
const myAddress = wallet.key.accAddress(CHAIN_PREFIX);

const init = async () => {
  const memberContractInitMsg = {};
  const initMemberContractCodeMsg = new MsgInstantiateContract(
    myAddress,
    myAddress,
    parseInt(MEMBER_CONTRACT_CODE_ID!),
    memberContractInitMsg,
    undefined,
    `${MEMBER_CONTRACT}-${MEMBER_CONTRACT_VERSION}`
  );

  createSignBroadcastCatch(wallet, [initMemberContractCodeMsg], true);
};

init();
