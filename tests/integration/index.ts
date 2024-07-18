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
    Commitment
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
    PRICE_PRECISION
} from "./constants";

import { DefaultProgramAccounts, Result, SaturnV1Implementation, SaturnV1Program } from "./types"
import { SaturnV1 as SaturnV1Idl, IDL } from "./idl/saturn_v_1";

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

    private constructor(
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
        treasury: PublicKey,
        sequenceFlag: PublicKey,
    ): Promise<Result> {

        //constraints here
        let msg = "";

        //pda check
        msg = this.pdaCheck([
            { pdaIdentifier: "treasury", pdaSeeds: [Buffer.from(TREASURY_SEED)], account: treasury }
        ]);

        //treasury
        const treasuryAccount = await this.program.account.treasury.fetchNullable(treasury);
        if (treasuryAccount)
            msg = "Invalid treasury account.";

        if (msg !== "")
            return {
                success: false,
                msg: msg,
                tx: null
            };
        //

        let initializeTx: Transaction;
        initializeTx = await this.program.methods
            .initialize()
            .accounts({
                admin,
                treasury,
                sequenceFlag,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: admin, ...(await this.connection.getLatestBlockhash()) }).add(initializeTx)
        };
    }

    public async applyBond(
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
    ): Promise<Result> {

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
                tx: null
            };

        //

        let applyBondTx: Transaction;
        applyBondTx = await this.program.methods
            .applyBond({ tokenAmount, spotPrice })
            .accounts({
                creator,
                treasuryAuthority,
                treasury,
                escrow,
                creatorTokenAccount,
                treasuryTokenAccount,
                treasuryStfTokenAccount,
                priceUpdate,
                tokenMintAddress,
                stfTokenMint,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: creator, ...(await this.connection.getLatestBlockhash()) }).add(applyBondTx)
        };
    }

    public async finishBond(
        user: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        destStfAccount: PublicKey,
        escrow: PublicKey,
        stfTokenMint,
        tokenProgram
    ): Promise<Result> {

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
                tx: null
            };
        //

        let finishBondTx: Transaction;
        finishBondTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: user, ...(await this.connection.getLatestBlockhash()) }).add(finishBondTx)
        };
    }

    public async stakeStf(
        amountToStake: BN,
        user: PublicKey,
        userStakeAccount: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        userTokenAccount: PublicKey,
        treasuryTokenAccount: PublicKey,
        stfTokenMint: PublicKey,
    ): Promise<Result> {

        //constraints here

        //

        let stakeStfTx: Transaction;
        stakeStfTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: user, ...(await this.connection.getLatestBlockhash()) }).add(stakeStfTx)
        };
    }

    public async unstakeStf(
        amountToUnstake: BN,
        user: PublicKey,
        userStakeAccount: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        userTokenAccount: PublicKey,
        treasuryTokenAccount: PublicKey,
        stfTokenMint: PublicKey
    ): Promise<Result> {

        //constraints here

        //

        let unstakeStfTx: Transaction;
        unstakeStfTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: user, ...(await this.connection.getLatestBlockhash()) }).add(unstakeStfTx)
        };
    }

    public async swap(
        data: any,
        fromAmount: BN,
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        wbtcTreasuryTokenAccount: PublicKey,
        wbtcMint: PublicKey,
        usdtTreasuryTokenAccount: PublicKey,
        usdtMint: PublicKey,
        usdcTreasuryTokenAccount: PublicKey,
        usdcMint: PublicKey,
        solMint: PublicKey,
        priceUpdate: PublicKey,
        jupiterProgram: PublicKey,
    ): Promise<Result> {

        //constraints here

        //

        let swapTx: Transaction;
        swapTx = await this.program.methods
            .swap(Buffer.from(data, "base64"), fromAmount)
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                wbtcTreasuryTokenAccount,
                wbtcMint,
                usdtTreasuryTokenAccount,
                usdtMint,
                usdcTreasuryTokenAccount,
                usdcMint,
                solMint,
                priceUpdate,
                jupiterProgram,
                ...defaultSystemAccounts
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(swapTx)
        };
    }

    public async meteoraDeposit(
        poolTokenAmount: BN,
        maximumTokenAAmount: BN,
        maximumTokenBAmount: BN,
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
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

        //constraints here

        //

        let meteoraDepositTx: Transaction;
        meteoraDepositTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(meteoraDepositTx)
        };
    }

    public async meteoraWithdraw(
        poolTokenAmount: BN,
        maximumTokenAAmount: BN,
        maximumTokenBAmount: BN,
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
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

        //constraints here

        //

        let meteoraWithdrawTx: Transaction;
        meteoraWithdrawTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(meteoraWithdrawTx)
        };
    }

    public async initLendingAccounts(
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
    ): Promise<Result> {

        //constraints here

        //

        let initLendingAccountsTx: Transaction;
        initLendingAccountsTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: owner, ...(await this.connection.getLatestBlockhash()) }).add(initLendingAccountsTx)
        };
    }

    public async klendLend(
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
    ): Promise<Result> {

        //constraints here

        //

        let klendLendTx: Transaction;
        klendLendTx = await this.program.methods
            .klendLend(amount)
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(klendLendTx)
        };
    }

    public async klendWithdraw(
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
    ): Promise<Result> {

        //constraints here

        //

        let klendWithdrawTx: Transaction;
        klendWithdrawTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(klendWithdrawTx)
        };
    }

    public async marginfiLend(
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
    ): Promise<Result> {

        //constraints here

        //

        let marginfiLendTx: Transaction;
        marginfiLendTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(marginfiLendTx)
        };
    }

    public async marginfiWithdraw(
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
    ): Promise<Result> {

        //constraints here

        //

        let marginfiWithdrawTx: Transaction;
        marginfiWithdrawTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(marginfiWithdrawTx)
        };
    }

    public async getValueInMeteora(
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        pool: PublicKey,
        aVault: PublicKey,
        bVault: PublicKey,
        userPoolLp: PublicKey,
    ): Promise<Result> {

        //constraints here

        //

        let getValueInMeteoraTx: Transaction;
        getValueInMeteoraTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(getValueInMeteoraTx)
        };
    }

    public async getValueInKamino(
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
    ): Promise<Result> {

        //constraints here

        //

        let getValueInKaminoTx: Transaction;
        getValueInKaminoTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(getValueInKaminoTx)
        };
    }

    public async getValueInMarginfi(
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
    ): Promise<Result> {

        //constraints here

        //

        let getValueInMarginfiTx: Transaction;
        getValueInMarginfiTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(getValueInMarginfiTx)
        };
    }

    public async calcuBalance(
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        sequenceFlag: PublicKey,
        usdcTokenAccount: PublicKey,
        wbtcTokenAccount: PublicKey,
        priceUpdate: PublicKey,
    ): Promise<Result> {

        //constraints here

        //

        let calcuBalanceTx: Transaction;
        calcuBalanceTx = await this.program.methods
            .calcuBalance()
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                sequenceFlag,
                usdcTokenAccount,
                wbtcTokenAccount,
                priceUpdate,
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(calcuBalanceTx)
        };
    }

    public async reallocate(
        returnRate: number[],
        riskRating: number[],
        signer: PublicKey,
        treasuryAuthority: PublicKey,
        treasury: PublicKey,
        sequenceFlag: PublicKey,
        usdcTokenAccount: PublicKey,
        wbtcTokenAccount: PublicKey,
        priceUpdate: PublicKey,
    ): Promise<Result> {

        //constraints here

        //

        let reallocateTx: Transaction;
        reallocateTx = await this.program.methods
            .reallocate(returnRate, riskRating)
            .accounts({
                signer,
                treasuryAuthority,
                treasury,
                sequenceFlag,
                usdcTokenAccount,
                wbtcTokenAccount,
                priceUpdate,
            })
            // .preInstructions()  add pre instructions if needed
            // .postInstructions() add post instructions if needed
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(reallocateTx)
        };
    }

    public async cashingoutReedem(
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
    ): Promise<Result> {

        //constraints here

        //

        // *IMPORTANT. Can't find signer at the contract code on main branch
        let cashinngoutReedemTx: Transaction;
        cashinngoutReedemTx = await this.program.methods
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
            .transaction();

        return {
            success: true,
            msg: null,
            tx: new Transaction({ feePayer: signer, ...(await this.connection.getLatestBlockhash()) }).add(cashinngoutReedemTx)
        };
    }
}