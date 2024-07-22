import { BN } from "@coral-xyz/anchor"
import { PublicKey } from "@solana/web3.js";
export const PROGRAM_ID = ""


export const TREASURY_SEED: string = "global-treasury-2";
export const TREASURY_AUTHORITY_SEED: string = "treasury-authority";
export const SEQUENCE_FLAG_SEED: string = "seqence_flag";
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
export const USDT_PRICE_ID: string = "0x2b89b9dc8fdf9f34709a5b106b472f0f39bb6ca9ce04b0fd7f2e971688e2e53b";
export const WBTC_PRICE_ID: string = "0xc9d8b075a5c69303365ae23633d4e085199bf5c520a3b90fed1322a0342ffc33";

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

// klend
/**
 * User metadata seed
 */
export const BASE_SEED_USER_METADATA = 'user_meta';
export const LENDING_MARKET_AUTH = 'lma';
export const DEFAULT_RECENT_SLOT_DURATION_MS = 450;
export const KAMINO_PROGRAM_ID = new PublicKey("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD");