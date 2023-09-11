import task from "@terra-money/terrariums";

task(async ({ deployer, signer, refs }) => {
  deployer.buildContract("cw-friend");
  deployer.optimizeContract("cw-friend");

  await deployer.storeCode("cw-friend");
  await new Promise((resolve) => setTimeout(resolve, 10000));

  const instantiateMsg = {
    admin_addr: signer.key.accAddress,
    key_register_admin_addr: signer.key.accAddress,
    protocol_fee_collector_addr: signer.key.accAddress,
    fee_denom: "uluna",
    protocol_fee_percentage: "5",
    key_issuer_fee_percentage: "5",
  }
  await deployer.instantiate("cw-friend", instantiateMsg, {
    admin: signer.key.accAddress,
    label: "cw-friend",
  });
  await new Promise((resolve) => setTimeout(resolve, 10000));

  refs.saveRefs();
});
