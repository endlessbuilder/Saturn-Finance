import { BN } from "@coral-xyz/anchor"
export const PROGRAM_ID = ""


export const TREASURY_SEED: string = "global-treasury-2";
export const TREASURY_AUTHORITY_SEED: string = "treasury-authority";
export const USDC_MINT: string = "9cmYPgxT1wGP6ySgSDHCmTrLYzeDp1iVssy4grjdjDyQ";
export const BONK_MINT: string = "GAKS74QSGdt4tN4SLH6bHhJfAucYu3e8Dwf6hRRcJaU1";
export const USDT_MINT: string = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
export const SOL_MINT: string = "11111111111111111111111111111111";
export const WSOL_MINT: string = "So11111111111111111111111111111111111111112";
export const WETH_MINT: string = "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs";
export const WBTC_MINT: string = "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh";
export const BONK_PRICE_ID: string = "0x72b021217ca3fe68922a19aaf990109cb9d84e9ad004b4d2025ad6f529314419";
export const USDC_PRICE_ID: string = "0xeaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a";
export const SOL_PRICE_ID: string = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
export const STF_MINT: string = "3HWcdN9fxD3ytB7L2FG5c3WJXQin3QFUNZoESCQriLD7";
export const PERSONAL_SEED: string = "personal-saturn";
export const ESCROW: string = "escrow";

// Meteora
export const VAULT_PREFIX: string = "vault";
export const TOKEN_VAULT_PREFIX: string = "token_vault";
export const TREASURY_METEORA_LP: string = "treasury_meteora_lp";
export const LP_MINT_PREFIX: string = "lp_mint";
export const COLLATERAL_VAULT_PREFIX: string = "collateral_vault";
export const FEE_VAULT_PREFIX: string = "fee_vault";
export const SOLEND_OBLIGATION_PREFIX: string = "solend_obligation";
export const SOLEND_OBLIGATION_OWNER_PREFIX: string = "solend_obligation_owner";
export const APRICOT_USER_INFO_SIGNER_PREFIX: string = "apricot_user_info_signer";

/// Fee denominator
export const FEE_DENOMINATOR: BN = new BN(10_000);
export const DEFAULT_FEE_RATIO: BN = new BN(5_000); // 50%

/// Virtual price precision
export const PRICE_PRECISION: BN = new BN(1_000_000_000_000);