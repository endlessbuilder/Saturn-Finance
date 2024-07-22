import * as anchor from "@coral-xyz/anchor";

import {
    Program,
    AnchorProvider,
    BN,
}
    from "@coral-xyz/anchor";

import {
    PublicKey,
    Connection,
    Cluster,
    Transaction,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    SYSVAR_INSTRUCTIONS_PUBKEY,
    Commitment,
    TransactionInstruction
} from "@solana/web3.js";

import {
    getAccount,
    TOKEN_PROGRAM_ID
} from "@solana/spl-token"

import {
    PROGRAM_ID,
    TREASURY_SEED,
    TREASURY_AUTHORITY_SEED,
    USDC_MINT,
    BONK_MINT,
    USDT_MINT,
    SOL_MINT,
    WSOL_MINT,
    WETH_MINT,
    WBTC_MINT,
    BONK_PRICE_ID,
    USDC_PRICE_ID,
    SOL_PRICE_ID,
    STF_MINT,
    PERSONAL_SEED,
    ESCROW,
    VAULT_PREFIX,
    TOKEN_VAULT_PREFIX,
    TREASURY_METEORA_LP,
    LP_MINT_PREFIX,
    COLLATERAL_VAULT_PREFIX,
    FEE_VAULT_PREFIX,
    SOLEND_OBLIGATION_PREFIX,
    SOLEND_OBLIGATION_OWNER_PREFIX,
    APRICOT_USER_INFO_SIGNER_PREFIX,
    FEE_DENOMINATOR,
    DEFAULT_FEE_RATIO,
    PRICE_PRECISION,
    SEQUENCE_FLAG_SEED
} from "./constants";

import { DefaultProgramAccounts, Result, SaturnV1Implementation, SaturnV1Program } from "./types"
import { SaturnV1 as SaturnV1Idl, IDL } from "../../target/types/saturn_v_1";

const defaultSystemAccounts: DefaultProgramAccounts = {
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    rent: SYSVAR_RENT_PUBKEY,
    instruction: SYSVAR_INSTRUCTIONS_PUBKEY
}

export default class SaturnV1Impl implements SaturnV1Implementation {
    private connection: Connection;
    private cluster: Cluster = 'mainnet-beta';

    // SaturnV1
    private program: SaturnV1Program;

    constructor(
        program: SaturnV1Program,
        opt?: {
            cluster?: Cluster;
        },
    ) {
        this.connection = program.provider.connection;
        this.cluster = opt?.cluster ?? 'mainnet-beta';

        this.program = program;
    }

    public static async create(
        connection: Connection,
        opt?: {
            cluster?: Cluster;
            programId?: string;
        },
    ): Promise<SaturnV1Impl> {
        const provider = new AnchorProvider(connection, {} as any, AnchorProvider.defaultOptions());
        const program = new Program<SaturnV1Idl>(IDL as SaturnV1Idl, opt?.programId || PROGRAM_ID, provider);

        return new SaturnV1Impl(
            program,
            {
                ...opt,
            },
        );
    }

    public getPda(
        seeds: Buffer[],
        programId: PublicKey = this.program.programId,
    ): PublicKey {
        const seedsBuffer = [];
        return PublicKey.findProgramAddressSync(seeds, programId)[0];
    }

    public pdaCheck(
        PDAs: { pdaIdentifier: string; pdaSeeds: Buffer[], account: PublicKey }[]
    ): string {
        for (var pda of PDAs) {
            if (this.getPda(pda.pdaSeeds) !== pda.account)
                return "Invalid " + pda.pdaIdentifier + " account.";
        }
        return "";
    }

    public getTreasury(): PublicKey {
        return this.getPda([Buffer.from(TREASURY_SEED)]);
    }
    public getTreasuryAuthority(): PublicKey {
        return this.getPda([Buffer.from(TREASURY_AUTHORITY_SEED)]);
    }
    public getEscrow(): PublicKey {
        return this.getPda([Buffer.from(ESCROW)]);
    }
    public getUserStakeAccount(): PublicKey {
        return this.getPda([Buffer.from[PERSONAL_SEED]]);
    }
    public getSequenceFlag(): PublicKey {
        return this.getPda([Buffer.from(SEQUENCE_FLAG_SEED)]);
    }
    public getKlendProgramId(): PublicKey {
        return new PublicKey("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD");
    }
    public getMarginfiProgramId(): PublicKey {
        return new PublicKey("MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA");
    }
    public getDynamicAmmProgramId(): PublicKey {
        return new PublicKey("ammbh4CQztZ6txJ8AaQgPsWjd6o7GhmvopS2JAo5bCB");
    }
    public getDynamicVaultProgramId(): PublicKey {
        return new PublicKey("24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi");
    }
    public getJupiterProgramId(): PublicKey {
        return new PublicKey("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");
    }

    public async validateATA(
        ata: PublicKey,
        mint: PublicKey,
        owner: PublicKey,
        commitment: Commitment = "confirmed",
        programId = TOKEN_PROGRAM_ID,
        connection = this.connection,
    ): Promise<boolean> {
        try {
            const account = await getAccount(connection, ata, commitment, programId);
            if (account.mint === mint && account.owner === owner) return true;
            return false;
        } catch (error: unknown) {
            return false;
        }
    }

    public async initialize(
        admin: PublicKey,
        // treasury: PublicKey,
        // sequenceFlag: PublicKey,
    ): Promise<Result> {

        let treasury = this.getTreasury();
        let treasuryAuthority = this.getTreasuryAuthority();
        let sequenceFlag = this.getSequenceFlag();

        //constraints here
        let msg = "";

        //pda check
        // msg = this.pdaCheck([
        //     { pdaIdentifier: "treasury", pdaSeeds: [Buffer.from(TREASURY_SEED)], account: treasury }
        // ]);

        //treasury
        // const treasuryAccount = await this.program.account.treasury.fetchNullable(treasury);
        // if (treasuryAccount)
        //     msg = "Invalid treasury account.";

        // if (msg !== "")
        //     return {
        //         success: false,
        //         msg: msg,
        //         ix: null
        //     };
        //

        let initializeIx: TransactionInstruction;
        initializeIx = await this.program.methods
            .initialize()
            .accounts({
                admin,
                treasury,
                sequenceFlag,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: initializeIx
        };
    }

    public async applyBond(
        tokenAmount: BN,
        spotPrice: BN,
        creator: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        // escrow: PublicKey,
        creatorTokenAccount,
        treasuryTokenAccount,
        treasuryStfTokenAccount,
        solPriceUpdate,
        usdcPriceUpdate,
        wbtcPriceUpdate,
        tokenMintAddress,
        stfTokenMint,
    ): Promise<Result> {

        let treasury = this.getTreasury();
        let treasuryAuthority = this.getTreasuryAuthority();
        let sequenceFlag = this.getSequenceFlag();
        let escrow = this.getEscrow();

        //constraints here
        let msg = "";

        //pda check
        msg = this.pdaCheck([
            { pdaIdentifier: "treasury", pdaSeeds: [Buffer.from(TREASURY_SEED)], account: treasury },
            { pdaIdentifier: "treasuryAuthority", pdaSeeds: [Buffer.from(TREASURY_AUTHORITY_SEED)], account: treasuryAuthority },
            { pdaIdentifier: "escrow", pdaSeeds: [Buffer.from(ESCROW), creator.toBuffer()], account: treasury },
        ]);

        //treasury
        const treasuryAccount = await this.program.account.treasury.fetchNullable(treasury);
        if (!treasuryAccount)
            msg = "Invalid treasury account"

        //creatorTokenAccount
        const validCreatorTokenAccount: boolean = await this.validateATA(creatorTokenAccount, tokenMintAddress, creator);
        if (!validCreatorTokenAccount)
            msg = "Invalid creatorTokenAccount.";

        //treasuryTokenAccount
        const validTreasuryTokenAccount: boolean = await this.validateATA(treasuryTokenAccount, tokenMintAddress, creator);
        if (!validTreasuryTokenAccount)
            msg = "Invalid treasuryTokenAccount.";

        //treasuryStfTokenAccount
        const validTreasuryStfTokenAccount: boolean = await this.validateATA(treasuryStfTokenAccount, stfTokenMint, treasuryAuthority);
        if (!validTreasuryStfTokenAccount)
            msg = "Invalid treasuryStfTokenAccount.";

        //priceUpdate


        if (msg !== "")
            return {
                success: false,
                msg: msg,
                ix: null
            };

        //

        let applyBondIx: TransactionInstruction;
        applyBondIx = await this.program.methods
            .applyBond({ tokenAmount, spotPrice })
            .accounts({
                creator,
                treasuryAuthority,
                treasury,
                escrow,
                creatorTokenAccount,
                treasuryTokenAccount,
                treasuryStfTokenAccount,
                solPriceUpdate,
                usdcPriceUpdate,
                bonkPriceUpdate,
                tokenMintAddress,
                stfTokenMint,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: applyBondIx
        };
    }

    public async finishBond(
        user: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        destStfAccount: PublicKey,
        // escrow: PublicKey,
        stfTokenMint: PublicKey,
    ): Promise<Result> {

        let treasury = this.getTreasury();
        let treasuryAuthority = this.getTreasuryAuthority();
        let escrow = this.getEscrow();

        //constraints here
        let msg = "";
        //pda check
        msg = this.pdaCheck([
            { pdaIdentifier: "treasury", pdaSeeds: [Buffer.from(TREASURY_SEED)], account: treasury },
            { pdaIdentifier: "treasuryAuthority", pdaSeeds: [Buffer.from(TREASURY_AUTHORITY_SEED)], account: treasuryAuthority },
            { pdaIdentifier: "escrow", pdaSeeds: [Buffer.from(ESCROW), user.toBuffer()], account: treasury },
        ]);

        //treasury
        const treasuryAccount = await this.program.account.treasury.fetchNullable(treasury);
        if (!treasuryAccount)
            msg = "Invalid treasury account";

        //destStfAccount
        const validDestStfAccount: boolean = await this.validateATA(destStfAccount, stfTokenMint, user);
        if (!validDestStfAccount)
            msg = "Invalid destStfAccount.";

        //escrow
        const escrowAccount = await this.program.account.escrow.fetchNullable(escrow);
        if (!escrowAccount)
            msg = "Invalid escrow account";

        if (msg !== "")
            return {
                success: false,
                msg: msg,
                ix: null
            };
        //

        let finishBondIx: TransactionInstruction;
        finishBondIx = await this.program.methods
            .finishBond()
            .accounts({
                user,
                treasuryAuthority,
                treasury,
                destStfAccount,
                escrow,
                stfTokenMint,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: finishBondIx
        };
    }

    public async stakeStf(
        amountToStake: BN,
        user: PublicKey,
        // userStakeAccount: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        userTokenAccount: PublicKey,
        treasuryTokenAccount: PublicKey,
        stfTokenMint: PublicKey,
    ): Promise<Result> {

        let treasury = this.getTreasury();
        let treasuryAuthority = this.getTreasuryAuthority();
        let userStakeAccount = this.getUserStakeAccount();

        //constraints here

        //

        let stakeStfIx: TransactionInstruction;
        stakeStfIx = await this.program.methods
            .stakeStf(amountToStake)
            .accounts({
                user,
                userStakeAccount,
                treasuryAuthority,
                treasury,
                userTokenAccount,
                treasuryTokenAccount,
                stfTokenMint,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: stakeStfIx
        };
    }

    public async unstakeStf(
        amountToUnstake: BN,
        user: PublicKey,
        // userStakeAccount: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        userTokenAccount: PublicKey,
        treasuryTokenAccount: PublicKey,
        stfTokenMint: PublicKey
    ): Promise<Result> {

        let treasury = this.getTreasury();
        let treasuryAuthority = this.getTreasuryAuthority();
        let userStakeAccount = this.getUserStakeAccount();

        //constraints here

        //

        let unstakeStfIx: TransactionInstruction;
        unstakeStfIx = await this.program.methods
            .unstakeStf(amountToUnstake)
            .accounts({
                user,
                userStakeAccount,
                treasuryAuthority,
                treasury,
                userTokenAccount,
                treasuryTokenAccount,
                stfTokenMint,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: unstakeStfIx
        };
    }

    public async swap(
        data: any,
        fromAmount: BN,
        signer: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        fromTreasuryTokenAccount: PublicKey,
        fromMint: PublicKey,
        toTreasuryTokenAccount: PublicKey,
        toMint: PublicKey,
    ): Promise<Result> {

        let treasury = this.getTreasury();
        let treasuryAuthority = this.getTreasuryAuthority();
        let jupiterProgram = this.getJupiterProgramId();

        //constraints here

        //

        let swapIx: TransactionInstruction;
        swapIx = await this.program.methods
            .swap(Buffer.from(data, "base64"), fromAmount)
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                fromTreasuryTokenAccount,
                fromMint,
                toTreasuryTokenAccount,
                toMint,
                jupiterProgram,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: swapIx
        };
    }

    public async meteoraDeposit(
        poolTokenAmount: BN,
        maximumTokenAAmount: BN,
        maximumTokenBAmount: BN,
        signer: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        sequenceFlag: PublicKey,
        pool: PublicKey,
        lpMint: PublicKey,
        userPoolLp: PublicKey,
        aVaultLp: PublicKey,
        bVaultLp: PublicKey,
        aVault: PublicKey,
        bVault: PublicKey,
        aVaultLpMint: PublicKey,
        bVaultLpMint: PublicKey,
        aTokenVault: PublicKey,
        bTokenVault: PublicKey,
        userAToken: PublicKey,
        userBToken: PublicKey,
        user: PublicKey,
        vaultProgram: PublicKey,
        dynamicAmmProgram: PublicKey,
    ): Promise<Result> {

        let treasury = this.getTreasury();
        let treasuryAuthority = this.getTreasuryAuthority();

        //constraints here

        //

        let meteoraDepositIx: TransactionInstruction;
        meteoraDepositIx = await this.program.methods
            .meteoraDeposit(poolTokenAmount, maximumTokenAAmount, maximumTokenBAmount)
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                sequenceFlag,
                pool,
                lpMint,
                userPoolLp,
                aVaultLp,
                bVaultLp,
                aVault,
                bVault,
                aVaultLpMint,
                bVaultLpMint,
                aTokenVault,
                bTokenVault,
                userAToken,
                userBToken,
                user,
                vaultProgram,
                dynamicAmmProgram,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: meteoraDepositIx
        };
    }

    public async meteoraWithdraw(
        poolTokenAmount: BN,
        maximumTokenAAmount: BN,
        maximumTokenBAmount: BN,
        signer: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        sequenceFlag: PublicKey,
        pool: PublicKey,
        lpMint: PublicKey,
        userPoolLp: PublicKey,
        aVaultLp: PublicKey,
        bVaultLp: PublicKey,
        aVault: PublicKey,
        bVault: PublicKey,
        aVaultLpMint: PublicKey,
        bVaultLpMint: PublicKey,
        aTokenVault: PublicKey,
        bTokenVault: PublicKey,
        userAToken: PublicKey,
        userBToken: PublicKey,
        user: PublicKey,
        vaultProgram: PublicKey,
        dynamicAmmProgram: PublicKey,
    ): Promise<Result> {

        let treasury = this.getTreasury();
        let treasuryAuthority = this.getTreasuryAuthority();

        //constraints here

        //

        let meteoraWithdrawIx: TransactionInstruction;
        meteoraWithdrawIx = await this.program.methods
            .meteoraWithdraw(poolTokenAmount, maximumTokenAAmount, maximumTokenBAmount) // unknown error, recommed that params may be defined at the right above to context
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                sequenceFlag,
                pool,
                lpMint,
                userPoolLp,
                aVaultLp,
                bVaultLp,
                aVault,
                bVault,
                aVaultLpMint,
                bVaultLpMint,
                aTokenVault,
                bTokenVault,
                userAToken,
                userBToken,
                user,
                vaultProgram,
                dynamicAmmProgram,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: meteoraWithdrawIx
        };
    }

    public async initLendingAccounts(
        owner: PublicKey,
        // treasuryAuthority: PublicKey,
        // marginfiProgram: PublicKey,
        marginfiGroup: PublicKey,
        // klendProgram: PublicKey,
        seed1Account: PublicKey,
        seed2Account: PublicKey,
        lendingMarket: PublicKey,
        obligation: PublicKey,
        userMetadata: PublicKey,
        marginfiAccount: PublicKey,
    ): Promise<Result> {


        let treasuryAuthority = this.getPda([Buffer.from(TREASURY_AUTHORITY_SEED)]);
        let marginfiProgram = this.getMarginfiProgramId();
        let klendProgram = this.getKlendProgramId();

        //constraints here

        //

        let initLendingAccountsIx: TransactionInstruction;
        initLendingAccountsIx = await this.program.methods
            .initLendingAccounts()
            .accounts({
                owner,
                treasuryAuthority,
                marginfiProgram,
                marginfiGroup,
                klendProgram,
                seed1Account,
                seed2Account,
                lendingMarket,
                obligation,
                userMetadata,
                marginfiAccount,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: initLendingAccountsIx
        };
    }

    public async klendLend(
        signer: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        // sequenceFlag: PublicKey,
        obligation: PublicKey,
        lendingMarket: PublicKey,
        lendingMarketAuthority: PublicKey,
        reserve: PublicKey,
        reserveLiquiditySupply: PublicKey,
        reserveCollateralMint: PublicKey,
        reserveDestinationDepositCollateral: PublicKey,
        userSourceLiquidity: PublicKey,
        userDestinationCollateral: PublicKey,
        // klendProgram: PublicKey,
    ): Promise<Result> {

        let treasury = this.getPda([Buffer.from(TREASURY_SEED)]);
        let sequenceFlag = this.getPda([Buffer.from(SEQUENCE_FLAG_SEED)]);
        let treasuryAuthority = this.getPda([Buffer.from(TREASURY_AUTHORITY_SEED)]);
        let klendProgram = this.getKlendProgramId();

        //constraints here

        //

        let klendLendIx: TransactionInstruction;
        klendLendIx = await this.program.methods
            .klendLend()
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                sequenceFlag,
                obligation,
                lendingMarket,
                lendingMarketAuthority,
                reserve,
                reserveLiquiditySupply,
                reserveCollateralMint,
                reserveDestinationDepositCollateral,
                userSourceLiquidity,
                userDestinationCollateral,
                klendProgram,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: klendLendIx
        };
    }

    public async klendWithdraw(
        amount: BN,
        signer: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        // sequenceFlag: PublicKey,
        userDestinationLiquidity: PublicKey,
        // klendProgram: PublicKey,
        obligation: PublicKey,
        lendingMarket: PublicKey,
        withdrawReserve: PublicKey,
        reserveSourceCollateral: PublicKey,
        reserveCollateralMint: PublicKey,
        reserveLiquiditySupply: PublicKey,
        lendingMarketAuthority: PublicKey,
        userDestinationCollateral: PublicKey,
    ): Promise<Result> {

        let treasury = this.getPda([Buffer.from(TREASURY_SEED)]);
        let sequenceFlag = this.getPda([Buffer.from(SEQUENCE_FLAG_SEED)]);
        let treasuryAuthority = this.getPda([Buffer.from(TREASURY_AUTHORITY_SEED)]);
        let klendProgram = this.getKlendProgramId();

        //constraints here

        //

        let klendWithdrawIx: TransactionInstruction;
        klendWithdrawIx = await this.program.methods
            .klendWithdraw(amount)
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                sequenceFlag,
                userDestinationLiquidity,
                klendProgram,
                obligation,
                lendingMarket,
                withdrawReserve,
                reserveSourceCollateral,
                reserveCollateralMint,
                reserveLiquiditySupply,
                lendingMarketAuthority,
                userDestinationCollateral,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: klendWithdrawIx
        };
    }

    public async marginfiLend(
        amount: BN,
        signer: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        // sequenceFlag: PublicKey,
        // marginfiProgram: PublicKey,
        marginfiGroup: PublicKey,
        marginfiAccount: PublicKey,
        bank: PublicKey,
        userLiquidity: PublicKey,
        bankLiquidityVault: PublicKey,
    ): Promise<Result> {

        let treasury = this.getPda([Buffer.from(TREASURY_SEED)]);
        let sequenceFlag = this.getPda([Buffer.from(SEQUENCE_FLAG_SEED)]);
        let treasuryAuthority = this.getPda([Buffer.from(TREASURY_AUTHORITY_SEED)]);
        let marginfiProgram = this.getMarginfiProgramId();

        //constraints here

        //

        let marginfiLendIx: TransactionInstruction;
        marginfiLendIx = await this.program.methods
            .marginfiLend(amount)
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                sequenceFlag,
                marginfiProgram,
                marginfiGroup,
                marginfiAccount,
                bank,
                userLiquidity,
                bankLiquidityVault,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: marginfiLendIx
        };
    }

    public async marginfiWithdraw(
        amount: BN,
        signer: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        // sequenceFlag: PublicKey,
        // marginfiProgram: PublicKey,
        marginfiGroup: PublicKey,
        marginfiAccount: PublicKey,
        bank: PublicKey,
        userLiquidity: PublicKey,
        bankLiquidityVault: PublicKey,
        bankLiquidityVaultAuthority: PublicKey
    ): Promise<Result> {

        let treasury = this.getPda([Buffer.from(TREASURY_SEED)]);
        let sequenceFlag = this.getPda([Buffer.from(SEQUENCE_FLAG_SEED)]);
        let treasuryAuthority = this.getPda([Buffer.from(TREASURY_AUTHORITY_SEED)]);
        let marginfiProgram = this.getMarginfiProgramId();

        //constraints here

        //

        let marginfiWithdrawIx: TransactionInstruction;
        marginfiWithdrawIx = await this.program.methods
            .marginfiWithdraw(amount)
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                sequenceFlag,
                marginfiProgram,
                marginfiGroup,
                marginfiAccount,
                bank,
                userLiquidity,
                bankLiquidityVault,
                bankLiquidityVaultAuthority,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: marginfiWithdrawIx
        };
    }

    public async getValueInMeteora(
        signer: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        pool: PublicKey,
        aVault: PublicKey,
        bVault: PublicKey,
        userPoolLp: PublicKey,
    ): Promise<Result> {

        let treasury = this.getPda([Buffer.from(TREASURY_SEED)]);
        let sequenceFlag = this.getPda([Buffer.from(SEQUENCE_FLAG_SEED)]);
        let treasuryAuthority = this.getPda([Buffer.from(TREASURY_AUTHORITY_SEED)]);
        let marginfiProgram = this.getMarginfiProgramId();

        //constraints here

        //

        let getValueInMeteoraIx: TransactionInstruction;
        getValueInMeteoraIx = await this.program.methods
            .getValueInMeteora()
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                pool,
                aVault,
                bVault,
                userPoolLp,
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: getValueInMeteoraIx
        };
    }

    public async getValueInKamino(
        signer: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        lendingMarket: PublicKey,
        solReserve: PublicKey,
        usdcReserve: PublicKey,
        usdtReserve: PublicKey,
        wbtcReserve: PublicKey,
        wethReserve: PublicKey,
        bonkReserve: PublicKey,
        obligation: PublicKey,
    ): Promise<Result> {

        let treasury = this.getPda([Buffer.from(TREASURY_SEED)]);
        let treasuryAuthority = this.getPda([Buffer.from(TREASURY_AUTHORITY_SEED)]);

        //constraints here

        //

        let getValueInKaminoIx: TransactionInstruction;
        getValueInKaminoIx = await this.program.methods
            .getValueInKamino()
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                lendingMarket,
                solReserve,
                usdcReserve,
                usdtReserve,
                wbtcReserve,
                wethReserve,
                bonkReserve,
                obligation,
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: getValueInKaminoIx
        };
    }

    public async getValueInMarginfi(
        signer: PublicKey,
        marginfiGroup: PublicKey,
        marginfiAccount: PublicKey,
        solBank: PublicKey,
        usdcBank: PublicKey,
        usdtBank: PublicKey,
        wbtcBank: PublicKey,
        wethBank: PublicKey,
        bonkBank: PublicKey,
    ): Promise<Result> {

        let treasury = this.getPda([Buffer.from(TREASURY_SEED)]);
        let treasuryAuthority = this.getPda([Buffer.from(TREASURY_AUTHORITY_SEED)]);

        //constraints here

        //

        let getValueInMarginfiIx: TransactionInstruction;
        getValueInMarginfiIx = await this.program.methods
            .getValueInMarginfi()
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                marginfiGroup,
                marginfiAccount,
                solBank,
                usdcBank,
                usdtBank,
                wbtcBank,
                wethBank,
                bonkBank,
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: getValueInMarginfiIx
        };
    }

    public async calcuBalance(
        signer: PublicKey,
        usdcTokenAccount: PublicKey,
        wbtcTokenAccount: PublicKey,
        sol_priceUpdate: PublicKey,
        usdc_priceUpdate: PublicKey,
        wbtc_priceUpdate: PublicKey,
    ): Promise<Result> {

        let treasury = this.getPda([Buffer.from(TREASURY_SEED)]);
        let sequenceFlag = this.getPda([Buffer.from(SEQUENCE_FLAG_SEED)]);
        let treasuryAuthority = this.getPda([Buffer.from(TREASURY_AUTHORITY_SEED)]);

        //constraints here

        //

        let calcuBalanceIx: TransactionInstruction;
        calcuBalanceIx = await this.program.methods
            .calcuBalance()
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                sequenceFlag,
                usdcTokenAccount,
                wbtcTokenAccount,
                sol_priceUpdate,
                usdc_priceUpdate,
                wbtc_priceUpdate
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: calcuBalanceIx
        };
    }

    public async reallocate(
        returnRate: number[],
        riskRating: number[],
        signer: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        // sequenceFlag: PublicKey,
    ): Promise<Result> {

        let treasury = this.getPda([Buffer.from(TREASURY_SEED)]);
        let sequenceFlag = this.getPda([Buffer.from(SEQUENCE_FLAG_SEED)]);
        let treasuryAuthority = this.getPda([Buffer.from(TREASURY_AUTHORITY_SEED)]);

        //constraints here

        //

        let reallocateIx: TransactionInstruction;
        reallocateIx = await this.program.methods
            .reallocate(returnRate, riskRating)
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                sequenceFlag,
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: reallocateIx
        };
    }

    public async cashingoutReedem(
        amount: BN,
        signer: PublicKey,
        user: PublicKey,
        // treasuryAuthority: PublicKey,
        // treasury: PublicKey,
        // escrow: PublicKey,
        userTokenAccount: PublicKey,
        treasuryTokenAccount: PublicKey,
        feeWalletTokenAccount: PublicKey,
        treasuryStfTokenAccount: PublicKey,
        tokenMintAddress: PublicKey,
        stfTokenMint: PublicKey,
    ): Promise<Result> {

        let treasury = this.getPda([Buffer.from(TREASURY_SEED)]);
        let treasuryAuthority = this.getPda([Buffer.from(TREASURY_AUTHORITY_SEED)]);
        let escrow = new PublicKey([Buffer.from(ESCROW)]);

        //constraints here

        //

        // *IMPORTANT. Can't find signer at the contract code on main branch
        let cashinngoutReedemIx: TransactionInstruction;
        cashinngoutReedemIx = await this.program.methods
            .cashingoutReedem(amount)
            .accounts({
                user,
                treasuryAuthority,
                treasury,
                escrow,
                userTokenAccount,
                treasuryTokenAccount,
                feeWalletTokenAccount,
                treasuryStfTokenAccount,
                tokenMintAddress,
                stfTokenMint,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .instruction();

        return {
            success: true,
            msg: null,
            ix: cashinngoutReedemIx
        };
    }
}