import task from "@terra-money/terrariums";

task(async ({ deployer, signer, refs }) => {
  deployer.buildContract("cw-thread");
  deployer.optimizeContract("cw-thread");

  await deployer.storeCode("cw-thread");
  await new Promise((resolve) => setTimeout(resolve, 10000));

  const instantiateMsg = {
    admin_addr: signer.key.accAddress,
    registration_admin_addr: signer.key.accAddress,
    protocol_fee_collector_addr: signer.key.accAddress,
    fee_denom: "uluna",
    protocol_fee_percentage: "5",
    key_issuer_fee_percentage: "5",
  }
  await deployer.instantiate("cw-thread", instantiateMsg, {
    admin: signer.key.accAddress,
    label: "cw-thread",
  });
  await new Promise((resolve) => setTimeout(resolve, 10000));

  refs.saveRefs();
});
