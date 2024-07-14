
import { Program, BN } from "@coral-xyz/anchor";
import { PublicKey, Transaction } from "@solana/web3.js";
import { SaturnV1 } from "../idl/saturn_v_1";

export type SaturnV1Program = Program<SaturnV1>;

export type DefaultProgramAccounts = {
    tokenProgram: PublicKey,
    systemProgram: PublicKey,
    rent: PublicKey,
    instruction: PublicKey
}

export interface Result {
    success: boolean;
    msg: null | string;
    tx: null | Transaction;
}

export type SaturnV1Implementation = {
    
    // instructions
    initialize: (
        admin: PublicKey,
        treasury: PublicKey,
        sequenceFlag: PublicKey,
    ) => Promise<Result>;

    applyBond: (
        tokenAmount: BN,
        spotPrice: BN,
        creator: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        escrow: PublicKey,
        creatorTokenAccount,
        treasuryTokenAccount,
        treasuryStfTokenAccount,
        priceUpdate,
        tokenMintAddress,
        stfTokenMint,
    ) => Promise<Result>;

    finishBond: (
        user: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        destStfAccount: PublicKey,
        escrow: PublicKey,
        stfTokenMint,
        tokenProgram
    ) => Promise<Result>;

    stakeStf: (
        amountToStake: BN,
        user: PublicKey,
        userStakeAccount: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        userTokenAccount: PublicKey,
        treasuryTokenAccount: PublicKey,
        stfTokenMint: PublicKey,
    ) => Promise<Result>;

    unstakeStf: (
        amountToUnstake: BN,
        user: PublicKey,
        userStakeAccount: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        userTokenAccount: PublicKey,
        treasuryTokenAccount: PublicKey,
        stfTokenMint: PublicKey
    ) => Promise<Result>;

    swap: (
        data: any,
        fromAmount: BN,
        payer: PublicKey,
        treasuryAuthority: PublicKey,
        fromTreasuryTokenAccount: PublicKey,
        fromMint: PublicKey,
        toTreasuryTokenAccount: PublicKey,
        toMint: PublicKey,
        jupiterProgram: PublicKey,
    ) => Promise<Result>;

    meteoraDeposit: (
        tokenAmount: BN,
        minimumLpTokenAmount: BN,
        signer: PublicKey,
        partner: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        sequenceFlag: PublicKey,
        meteoraVaultProgram: PublicKey,
        vault: PublicKey,
        tokenVault: PublicKey,
        vaultLpMint: PublicKey,
        treasuryToken: PublicKey,
        treasuryLp: PublicKey,
    ) => Promise<Result>;

    meteoraWithdraw: (
        unmintAmount: BN,
        mintOutAmount: BN,
        signer: PublicKey,
        partner: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        sequenceFlag: PublicKey,
        vaultProgram: PublicKey,
        vault: PublicKey,
        tokenVault: PublicKey,
        vaultLpMint: PublicKey,
        treasuryToken: PublicKey,
        treasuryLp: PublicKey,
    ) => Promise<Result>;

    initLendingAccounts: (
        owner: PublicKey,
        treasuryAuthority: PublicKey,
        marginfiProgram: PublicKey,
        marginfiGroup: PublicKey,
        klendProgram: PublicKey,
        seed1Account: PublicKey,
        seed2Account: PublicKey,
        lendingMarket: PublicKey,
        obligation: PublicKey,
        userMetadata: PublicKey,
        marginfiAccount: PublicKey,
    ) => Promise<Result>;

    klendLend: (
        amount: BN,
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        sequenceFlag: PublicKey,
        obligation: PublicKey,
        lendingMarket: PublicKey,
        lendingMarketAuthority: PublicKey,
        reserve: PublicKey,
        reserveLiquiditySupply: PublicKey,
        reserveCollateralMint: PublicKey,
        reserveDestinationDepositCollateral: PublicKey,
        userSourceLiquidity: PublicKey,
        userDestinationCollateral: PublicKey,
        klendProgram: PublicKey,
    ) => Promise<Result>;

    klendWithdraw: (
        amount: BN,
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        sequenceFlag: PublicKey,
        userDestinationLiquidity: PublicKey,
        klendProgram: PublicKey,
        obligation: PublicKey,
        lendingMarket: PublicKey,
        withdrawReserve: PublicKey,
        reserveSourceCollateral: PublicKey,
        reserveCollateralMint: PublicKey,
        reserveLiquiditySupply: PublicKey,
        lendingMarketAuthority: PublicKey,
        userDestinationCollateral: PublicKey,
    ) => Promise<Result>;

    marginfiLend: (
        amount: BN,
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        sequenceFlag: PublicKey,
        marginfiProgram: PublicKey,
        marginfiGroup: PublicKey,
        marginfiAccount: PublicKey,
        bank: PublicKey,
        userLiquidity: PublicKey,
        bankLiquidityVault: PublicKey,
    ) => Promise<Result>;

    marginfiWithdraw: (
        amount: BN,
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        sequenceFlag: PublicKey,
        marginfiProgram: PublicKey,
        marginfiGroup: PublicKey,
        marginfiAccount: PublicKey,
        bank: PublicKey,
        userLiquidity: PublicKey,
        bankLiquidityVault: PublicKey,
        bankLiquidityVaultAuthority: PublicKey
    ) => Promise<Result>;

    getValueInMeteora: (
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        treasuryLp: PublicKey,
        vaultLpMint: PublicKey,
        meteoraVault: PublicKey,
    ) => Promise<Result>;

    getValueInKamino: (
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        lendingMarket: PublicKey,
        solReserve: PublicKey,
        usdcReserve: PublicKey,
        usdtReserve: PublicKey,
        wbtcReserve: PublicKey,
        wethReserve: PublicKey,
        bonkReserve: PublicKey,
        obligation: PublicKey,
    ) => Promise<Result>;

    getValueInMarginfi: (
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        marginfiGroup: PublicKey,
        marginfiAccount: PublicKey,
        solBank: PublicKey,
        usdcBank: PublicKey,
        usdtBank: PublicKey,
        wbtcBank: PublicKey,
        wethBank: PublicKey,
        bonkBank: PublicKey,
    ) => Promise<Result>;

    calcuBalance: (
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        sequenceFlag: PublicKey,
        usdcTokenAccount: PublicKey,
        wbtcTokenAccount: PublicKey,
        priceUpdate: PublicKey,
    ) => Promise<Result>;

    reallocate: (
        returnRate: number [],
        riskRating: number [],
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        sequenceFlag: PublicKey,
        usdcTokenAccount: PublicKey,
        wbtcTokenAccount: PublicKey,
        priceUpdate: PublicKey,
    ) => Promise<Result>;

    cashingoutReedem: (
        amount: BN,
        signer: PublicKey,
        user: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        escrow: PublicKey,
        userTokenAccount: PublicKey,
        treasuryTokenAccount: PublicKey,
        feeWalletTokenAccount: PublicKey,
        treasuryStfTokenAccount: PublicKey,
        tokenMintAddress: PublicKey,
        stfTokenMint: PublicKey,
    ) => Promise<Result>;
    //getter

};
