import axios from "axios";
import { LCDClient, MnemonicKey, Msg, Wallet } from "@terra-money/feather.js";

import {
  CHAIN_DENOM,
  CHAIN_ID,
  CHAIN_PREFIX,
  IS_COIN_TYPE_118,
  LCD_ENDPOINT,
  TESTER1_MNEMONIC_KEY,
  TESTER2_MNEMONIC_KEY,
  TESTER3_MNEMONIC_KEY,
  DEFAULT_TESTER_ID,
} from "./env";

export const getCurrentBlockHeight = async (
  lcd: LCDClient
): Promise<string> => {
  return (await lcd.tendermint.blockInfo(CHAIN_ID)).block.header.height;
};

export const getLCD = (): LCDClient => {
  return new LCDClient({
    [CHAIN_ID]: {
      lcd: LCD_ENDPOINT,
      chainID: CHAIN_ID,
      gasAdjustment: 3.5,
      gasPrices: { [CHAIN_DENOM]: 0.015 },
      prefix: CHAIN_PREFIX, // bech32 prefix, used by the LCD to understand which is the right chain to query
    },
  });
};

export const getMnemonicKey = (testerId = DEFAULT_TESTER_ID): MnemonicKey => {
  switch (testerId) {
    case 1:
      return new MnemonicKey({
        mnemonic: TESTER1_MNEMONIC_KEY,
        coinType: IS_COIN_TYPE_118 ? 118 : 330,
      });
    case 2:
      return new MnemonicKey({
        mnemonic: TESTER2_MNEMONIC_KEY,
        coinType: IS_COIN_TYPE_118 ? 118 : 330,
      });
    case 3:
      return new MnemonicKey({
        mnemonic: TESTER3_MNEMONIC_KEY,
        coinType: IS_COIN_TYPE_118 ? 118 : 330,
      });
    default:
      return new MnemonicKey({
        mnemonic: TESTER1_MNEMONIC_KEY,
        coinType: IS_COIN_TYPE_118 ? 118 : 330,
      });
  }
};

export const getWallet = (lcd: LCDClient, mnemonicKey: MnemonicKey): Wallet => {
  return new Wallet(lcd, mnemonicKey);
};

export const toBase64 = (obj: Object) => {
  return Buffer.from(JSON.stringify(obj)).toString("base64");
};

// !!! stargate msg value is binary encoded unlike others that are json encoded
export const toBase64FromBinary = (b: Uint8Array) => {
  return Buffer.from(b).toString("base64");
};

// used for encoding wasm contract
export const toBase64FromBuffer = (b: Buffer) => {
  return b.toString("base64");
};

// if is axios error then print the extracted part otherwise print whole error
// most of time it should be cause axios error is the one returned when we call lcd
export const printAxiosError = (e: any) => {
  if (axios.isAxiosError(e)) {
    if (e.response) {
      console.log(e.response.status);
      console.log(e.response.headers);
      if (
        typeof e.response.data === "object" &&
        e.response.data !== null &&
        "code" in e.response.data &&
        "message" in e.response.data
      ) {
        console.log(
          `Code=${e.response?.data["code"]} Message=${e.response?.data["message"]} \n`
        );
      } else {
        console.log(e.response.data);
      }
    }
  } else {
    console.log(e);
  }
};

export const queryWasmContractWithCatch = async (
  lcd: LCDClient,
  contractAddress: string,
  query: object
) => {
  return lcd.wasm.contractQuery(contractAddress, query).catch((e) => {
    console.log(`error in querying contract ${contractAddress}`);
    printAxiosError(e);
    throw e;
  });
};

export const createSignBroadcastCatch = async (
  wallet: Wallet,
  msgs: Msg[],
  skipLog = false,
  autoEstimateFee = true
) => {
  if (!skipLog) {
    console.log(
      `wallet address: ${wallet.key.accAddress(
        CHAIN_PREFIX
      )} sending tx msgs: ${JSON.stringify(msgs, null, 2)}`
    );
  }

  const txOptions = {
    msgs: msgs,
    chainID: CHAIN_ID,
  };
  if (!autoEstimateFee) {
    txOptions["gasPrices"] = "0.15uluna";
    txOptions["gasAdjustment"] = 1.4;
    txOptions["gas"] = (1_500_000).toString();
  }
  wallet
    .createAndSignTx(txOptions)
    .then((tx) => wallet.lcd.tx.broadcast(tx, CHAIN_ID))
    .catch((e) => {
      console.log("error in create and sign tx");
      printAxiosError(e);
      throw e;
    })
    .then((txInfo) => {
      console.log(txInfo);
    })
    .catch((e) => {
      console.log("error in broadcast tx");
      printAxiosError(e);
      throw e;
    });
};
