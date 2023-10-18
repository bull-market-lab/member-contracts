import { MsgExecuteContract } from "@terra-money/feather.js";
import {
  createSignBroadcastCatch,
  getLCD,
  getMnemonicKey,
  getWallet,
} from "./util";
import {
  CHAIN_PREFIX,
  DISTRIBUTION_CONTRACT_ADDR,
  MEMBER_CONTRACT_ADDR,
} from "./env";

const lcd = getLCD();

const mnemonicKey1 = getMnemonicKey();
const wallet = getWallet(lcd, mnemonicKey1);
const myAddress = wallet.key.accAddress(CHAIN_PREFIX);

const updateMemberConfig = async () => {
  const memberContractUpdateConfigMsg = {
    update_config: {
      distribution_contract_addr: DISTRIBUTION_CONTRACT_ADDR!,
    },
  };
  const updateMemberContractCodeMsg = new MsgExecuteContract(
    myAddress,
    MEMBER_CONTRACT_ADDR!,
    memberContractUpdateConfigMsg
  );

  createSignBroadcastCatch(wallet, [updateMemberContractCodeMsg], true);
};

updateMemberConfig();
