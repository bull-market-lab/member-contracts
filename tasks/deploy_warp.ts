import task from "@terra-money/terrariums";

task(async ({ deployer, signer, refs }) => {
  const memberContract = "cw-member";
  const memberContractVersion = "v0.1.0";
  const distributionContract = "cw-distribution";
  const distributionContractVersion = "v0.1.0";
  const threadContract = "cw-thread";
  const threadContractVersion = "v0.1.0";

  const deployerAddr = signer.key.accAddress;

  // ================= Deploy and instantiate member contract =================

  deployer.buildContract(memberContract);
  deployer.optimizeContract(memberContract);

  await deployer.storeCode(memberContract);
  await new Promise((resolve) => setTimeout(resolve, 10000));

  // Use default value for all params
  // const memberInstantiateMsg = {};
  // const { address: memberContractAddr } = await deployer.instantiate(
  //   memberContract,
  //   memberInstantiateMsg,
  //   {
  //     admin: deployerAddr,
  //     label: `${memberContract}-${memberContractVersion}`,
  //   }
  // );
  // await new Promise((resolve) => setTimeout(resolve, 10000));

  // // ================= Deploy and instantiate distribution contract =================

  // deployer.buildContract(distributionContract);
  // deployer.optimizeContract(distributionContract);

  // await deployer.storeCode(distributionContract);
  // await new Promise((resolve) => setTimeout(resolve, 10000));

  // const instantiateMsg = {
  //   member_contract_addr: memberContractAddr,
  // };
  // await deployer.instantiate(distributionContract, instantiateMsg, {
  //   admin: deployerAddr,
  //   label: `${distributionContract}-${distributionContractVersion}`,
  // });
  // await new Promise((resolve) => setTimeout(resolve, 10000));

  // // ================= Deploy and instantiate thread contract =================

  // deployer.buildContract(threadContract);
  // deployer.optimizeContract(threadContract);

  // await deployer.storeCode(threadContract);
  // await new Promise((resolve) => setTimeout(resolve, 10000));

  // const threadInstantiateMsg = {
  //   member_contract_addr: memberContractAddr,
  // };
  // await deployer.instantiate(threadContract, threadInstantiateMsg, {
  //   admin: deployerAddr,
  //   label: `${threadContract}-${threadContractVersion}`,
  // });

  // ================= Save refs =================

  refs.saveRefs();
});
