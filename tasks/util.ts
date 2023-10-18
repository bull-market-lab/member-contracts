import axios from "axios";
import { Msg, Wallet } from "@terra-money/feather.js";

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

// used for encoding wasm contract
export const toBase64FromBuffer = (b: Buffer) => {
  return b.toString("base64");
};

export const createSignBroadcastCatch = async (
  wallet: Wallet,
  msgs: Msg[],
  chainID = "pisco-1",
  autoEstimateFee = true
) => {
  const txOptions = {
    msgs: msgs,
    chainID,
  };
  if (!autoEstimateFee) {
    txOptions["gasPrices"] = "0.15uluna";
    txOptions["gasAdjustment"] = 1.4;
    txOptions["gas"] = (1_500_000).toString();
  }
  wallet
    .createAndSignTx(txOptions)
    .then((tx) => wallet.lcd.tx.broadcast(tx, chainID))
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
