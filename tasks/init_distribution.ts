import { MsgInstantiateContract } from "@terra-money/feather.js";
import {
  createSignBroadcastCatch,
  getLCD,
  getMnemonicKey,
  getWallet,
} from "./util";
import {
  CHAIN_PREFIX,
  DISTRIBUTION_CONTRACT_CODE_ID,
  DISTRIBUTION_CONTRACT_VERSION,
  MEMBER_CONTRACT_ADDR,
} from "./env";
import { DISTRIBUTION_CONTRACT } from "./const";

const lcd = getLCD();

const mnemonicKey1 = getMnemonicKey();
const wallet = getWallet(lcd, mnemonicKey1);
const myAddress = wallet.key.accAddress(CHAIN_PREFIX);

const init = async () => {
  const distributionContractInitMsg = {
    member_contract_addr: MEMBER_CONTRACT_ADDR!,
  };
  const initDistributionContractCodeMsg = new MsgInstantiateContract(
    myAddress,
    myAddress,
    parseInt(DISTRIBUTION_CONTRACT_CODE_ID!),
    distributionContractInitMsg,
    undefined,
    `${DISTRIBUTION_CONTRACT}-${DISTRIBUTION_CONTRACT_VERSION}`
  );

  createSignBroadcastCatch(wallet, [initDistributionContractCodeMsg], true);
};

init();
