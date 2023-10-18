import { Env, task } from "@terra-money/terrain";
import { Wallet, MsgStoreCode } from "@terra-money/feather.js";
import fs from "fs";

import { createSignBroadcastCatch, toBase64FromBuffer } from "./util";

task(async (env: Env) => {
  const memberContract = "member";
  const memberContractVersion = "v0.1.0";
  const memberContractWasm = "./artifacts/member.wasm";

  const distributionContract = "distribution";
  const distributionContractVersion = "v0.1.0";
  const distributionContractWasm = "./artifacts/distribution.wasm";

  const threadContract = "thread";
  const threadContractVersion = "v0.1.0";
  const threadContractWasm = "./artifacts/thread.wasm";

  const chainID = "pisco-1";

  const { deploy, defaultWallet } = env;

  const defaultWalletAddr = defaultWallet.key.accAddress("terra");
  const admin = defaultWalletAddr;

//   console.log(defaultWalletAddr);

//   //   console.log(console.log(process.cwd()));

//   const lcdClient = defaultWallet.lcd;
//   console.log(JSON.stringify(lcdClient.config, null, 2));

//   //   lcdClient.config

//   const wallet = new Wallet(lcdClient, defaultWallet.key);
//   console.log(wallet == undefined);

//   //   console.log(JSON.stringify(wallet, null, 2));
//   const storeCodeMsg = new MsgStoreCode(
//     admin,
//     toBase64FromBuffer(fs.readFileSync(memberContractWasm))
//   );
//   createSignBroadcastCatch(wallet, [storeCodeMsg], chainID);

  // For some reason, storeCode doesn't work, it cannot detect we are using cargo workspace
  // Although I checked terrain's code and it looks right
  // Either upload via celatone, or manually copy it to contract's artifacts folder

  //   // ================= Deploy and instantiate member contract =================

    const memberContractCodeID = await deploy.storeCode(
      memberContract,
      defaultWallet,
      {
        noRebuild: true,
      }
    );
    await new Promise((resolve) => setTimeout(resolve, 10000));

  //   const memberContractAddr = await deploy.instantiate(
  //     memberContract,
  //     defaultWallet,
  //     {
  //       admin,
  //       // custom label not supported by terrain
  //       // label: `${memberContract}-${memberContractVersion}`,
  //       codeId: memberContractCodeID,
  //       init: {},
  //     }
  //   );
  //   console.log(`memberContractAddr: ${memberContractAddr}`);
  //   await new Promise((resolve) => setTimeout(resolve, 10000));

  //   // ================= Deploy and instantiate distribution contract =================

  //   const distributionContractCodeID = await deploy.storeCode(
  //     distributionContract,
  //     defaultWallet,
  //     {
  //       noRebuild: true,
  //     }
  //   );
  //   await new Promise((resolve) => setTimeout(resolve, 10000));

  //   const distributionContractAddr = await deploy.instantiate(
  //     distributionContract,
  //     defaultWallet,
  //     {
  //       admin,
  //       // custom label not supported by terrain
  //       // label: `${distributionContract}-${distributionContractVersion}`,
  //       codeId: distributionContractCodeID,
  //       init: {
  //         member_contract_addr: memberContractAddr,
  //       },
  //     }
  //   );
  //   console.log(`distributionContractAddr: ${distributionContractAddr}`);
  //   await new Promise((resolve) => setTimeout(resolve, 10000));

  //   // ================= Deploy and instantiate thread contract =================

  //   const threadContractCodeID = await deploy.storeCode(
  //     threadContract,
  //     defaultWallet,
  //     {
  //       noRebuild: true,
  //     }
  //   );
  //   await new Promise((resolve) => setTimeout(resolve, 10000));

  //   const threadContractAddr = await deploy.instantiate(
  //     threadContract,
  //     defaultWallet,
  //     {
  //       admin,
  //       // custom label not supported by terrain
  //       // label: `${threadContract}-${threadContractVersion}`,
  //       codeId: threadContractCodeID,
  //       init: {
  //         member_contract_addr: memberContractAddr,
  //       },
  //     }
  //   );
  //   console.log(`threadContractAddr: ${threadContractAddr}`);
});
