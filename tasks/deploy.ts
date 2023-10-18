import { Env, task } from "@terra-money/terrain";
import { MnemonicKey } from "@terra-money/feather.js";

task(async (env: Env) => {
  //   console.log("creating new key");
  //   const key = new MnemonicKey();
  //   console.log("private key", key.privateKey.toString("base64"));
  //   console.log("mnemonic", key.mnemonic);

  const memberContract = "member";
  const memberContractVersion = "v0.1.0";
  const distributionContract = "distribution";
  const distributionContractVersion = "v0.1.0";
  const threadContract = "thread";
  const threadContractVersion = "v0.1.0";

  const { deploy, defaultWallet } = env;

  console.log("yoyoyo");
  console.log(defaultWallet.key.accAddress("terra"));

  deploy.optimize("member");
});

// const memberContract = "cw-member";
// const memberContractVersion = "v0.1.0";
// const distributionContract = "cw-distribution";
// const distributionContractVersion = "v0.1.0";
// const threadContract = "cw-thread";
// const threadContractVersion = "v0.1.0";

// const deployerAddr = signer.key.accAddress;

// // ================= Deploy and instantiate member contract =================

// deployer.buildContract(memberContract);
// deployer.optimizeContract(memberContract);

// await deployer.storeCode(memberContract);
// await new Promise((resolve) => setTimeout(resolve, 10000));

// // Use default value for all params
// // const memberInstantiateMsg = {};
// // const { address: memberContractAddr } = await deployer.instantiate(
// //   memberContract,
// //   memberInstantiateMsg,
// //   {
// //     admin: deployerAddr,
// //     label: `${memberContract}-${memberContractVersion}`,
// //   }
// // );
// // await new Promise((resolve) => setTimeout(resolve, 10000));

// // // ================= Deploy and instantiate distribution contract =================

// deployer.buildContract(distributionContract);
// deployer.optimizeContract(distributionContract);

// await deployer.storeCode(distributionContract);
// await new Promise((resolve) => setTimeout(resolve, 10000));

// // const instantiateMsg = {
// //   member_contract_addr: memberContractAddr,
// // };
// // await deployer.instantiate(distributionContract, instantiateMsg, {
// //   admin: deployerAddr,
// //   label: `${distributionContract}-${distributionContractVersion}`,
// // });
// // await new Promise((resolve) => setTimeout(resolve, 10000));

// // // ================= Deploy and instantiate thread contract =================

// deployer.buildContract(threadContract);
// deployer.optimizeContract(threadContract);

// await deployer.storeCode(threadContract);
// await new Promise((resolve) => setTimeout(resolve, 10000));

// // const threadInstantiateMsg = {
// //   member_contract_addr: memberContractAddr,
// // };
// // await deployer.instantiate(threadContract, threadInstantiateMsg, {
// //   admin: deployerAddr,
// //   label: `${threadContract}-${threadContractVersion}`,
// // });

// // ================= Save refs =================

// refs.saveRefs();
