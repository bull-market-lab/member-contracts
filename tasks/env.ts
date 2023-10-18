import dotenv from "dotenv";
import { env } from "process";

dotenv.config();

export const LCD_ENDPOINT: string = env.LCD_ENDPOINT!;

export const CHAIN_ID: string = env.CHAIN_ID!;
export const CHAIN_PREFIX: string = env.CHAIN_PREFIX!;
export const CHAIN_DENOM: string = env.CHAIN_DENOM!;

export const TESTER1_MNEMONIC_KEY: string = env.TESTER1_MNEMONIC_KEY!;
export const TESTER2_MNEMONIC_KEY: string = env.TESTER2_MNEMONIC_KEY!;
export const TESTER3_MNEMONIC_KEY: string = env.TESTER3_MNEMONIC_KEY!;

export const DEFAULT_TESTER_ID: number = parseInt(env.DEFAULT_TESTER_ID!);

export const IS_COIN_TYPE_118: boolean = env.IS_COIN_TYPE_118! === "true";

export const MEMBER_CONTRACT_CODE_ID = env.MEMBER_CONTRACT_CODE_ID;
export const DISTRIBUTION_CONTRACT_CODE_ID = env.DISTRIBUTION_CONTRACT_CODE_ID;
export const THREAD_CONTRACT_CODE_ID = env.THREAD_CONTRACT_CODE_ID;

export const MEMBER_CONTRACT_VERSION: string = env.MEMBER_CONTRACT_VERSION!;
export const DISTRIBUTION_CONTRACT_VERSION: string =
  env.DISTRIBUTION_CONTRACT_VERSION!;
export const THREAD_CONTRACT_VERSION: string = env.THREAD_CONTRACT_VERSION!;

export const MEMBER_CONTRACT_ADDR = env.MEMBER_CONTRACT_ADDR;
export const DISTRIBUTION_CONTRACT_ADDR = env.DISTRIBUTION_CONTRACT_ADDR;
export const THREAD_CONTRACT_ADDR = env.THREAD_CONTRACT_ADDR;
