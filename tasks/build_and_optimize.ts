import { Env, task } from "@terra-money/terrain";

task(async (env: Env) => {
  const memberContract = "member";
  const memberContractVersion = "v0.1.0";
  const distributionContract = "distribution";
  const distributionContractVersion = "v0.1.0";
  const threadContract = "thread";
  const threadContractVersion = "v0.1.0";

  const { deploy } = env;

  // optimize will build it if it doesn't exist
  await deploy.optimize(memberContract);
  await deploy.optimize(distributionContract);
  await deploy.optimize(threadContract);
});
