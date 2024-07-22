import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";
import adminJson from "./users/admin.json";
import userJson from "./users/user.json";
import { airDropSol, getBalance, getOrCreateATA, getSolBalance, mockWallet, toTokenAmount } from './utils';
import SaturnV1Impl from "./integration";
import {
    Connection,
    Keypair,
    PublicKey,
    PublicKeyInitData,
    Transaction,
    TransactionInstruction,
    AddressLookupTableAccount,
    TransactionMessage,
    VersionedTransaction,
    SystemProgram
} from "@solana/web3.js";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { MarginfiClient, getConfig, MarginfiConfig, Bank } from '@mrgnlabs/marginfi-client-v2';
import { KaminoMarket, KaminoObligation, KaminoReserve, lendingMarketAuthPda, Reserve, VanillaObligation } from "@kamino-finance/klend-sdk";

import { SaturnV1 } from "../target/types/saturn_v_1";
import { SaturnV1Program } from "./integration/types";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import * as constants from "./integration/constants";
import { Result } from "./integration/types";
import { PriceServiceConnection } from "@pythnetwork/price-service-client";
import { InstructionWithEphemeralSigners, PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";

export const jupiterProgramId = new PublicKey(
    "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"
);

describe('saturn', async () => {

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.SaturnV1 as SaturnV1Program;
    const provider = program.provider;
    const connection = program.provider.connection;

    let saturnV1: SaturnV1Impl = new SaturnV1Impl(program);

    const admin = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(adminJson));
    console.log(">>> create admin publickey : ", admin.publicKey.toBase58());
    //5vSwrp6mk5Po9d4L9uN6Vd18w26wVdrsVHjcNJKX62aG

    const user = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(userJson));
    console.log(">>> create user publickey : ", user.publicKey.toBase58());
    //3qtahdn6ez4hwRy3UwEC3Cf9pPdbMeGUvspWvgY2N6Ws 

    let treasury = saturnV1.getTreasury();
    let treasuryAuthority = saturnV1.getTreasuryAuthority();
    let escrow = saturnV1.getEscrow();
    let sequenceFlag = saturnV1.getSequenceFlag();
    // token accounts
    let treasuryUsdcTokenAccount: PublicKey;
    let treasuryUsdtTokenAccount: PublicKey;
    let treasuryWbtcTokenAccount: PublicKey;
    let treasuryStfTokenAccount: PublicKey;
    let treasuryWsolTokenAccount: PublicKey;
    let userUsdcTokenAccount: PublicKey;
    let userStfTokenAccount: PublicKey;
    let adminUsdcTokenAccount: PublicKey;
    let adminUsdtTokenAccount: PublicKey;

    let config: MarginfiConfig;
    let marginfiClient: MarginfiClient;
    let marginfiBank: Bank
    let marginfiAccountKey: PublicKey;

    let kaminoMarket: KaminoMarket;
    let kaminoUserMetadata: any;
    let kaminoUsdcRerserve: KaminoReserve;

    before(async () => {
        // marginfi
        config = getConfig("dev");
        marginfiClient = await MarginfiClient.fetch(config, new anchor.Wallet(admin), connection);
        const maringfiBankLabel = "USDT";
        marginfiBank = marginfiClient.getBankByTokenSymbol(maringfiBankLabel);
        if (!marginfiBank) throw Error(`${maringfiBankLabel} bank not found`);
        marginfiAccountKey = Keypair.generate().publicKey;

        // kamino lending
        // There are three levels of data you can request (and cache) about the lending market.
        // 1. Initalize market with parameters and metadata
        kaminoMarket = await KaminoMarket.load(
            connection,
            new PublicKey("7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF"), // main market address. Defaults to 'Main' market
            constants.DEFAULT_RECENT_SLOT_DURATION_MS,
            constants.KAMINO_PROGRAM_ID,
            true
        );
        // 2. Refresh reserves
        await kaminoMarket.loadReserves();
        kaminoUsdcRerserve = kaminoMarket.getReserveBySymbol("USDC");
        // Refresh all cached data
        kaminoMarket.refreshAll();
        kaminoUserMetadata = await kaminoMarket.getUserMetadata(admin.publicKey);
        // const obligation = await kaminoMarket.getObligationByWallet(admin.publicKey, new VanillaObligation(constants.KAMINO_PROGRAM_ID));

        //user spl accounts
        userUsdcTokenAccount = await getOrCreateATA(
            connection,
            new PublicKey(constants.USDC_MINT),
            user.publicKey,
            user,
        );
        userStfTokenAccount = await getOrCreateATA(
            connection,
            new PublicKey(constants.STF_MINT),
            user.publicKey,
            user,
        );
        // admin spl accounts
        adminUsdcTokenAccount = await getOrCreateATA(
            connection,
            new PublicKey(constants.USDC_MINT),
            admin.publicKey,
            admin,
        );
        // treasury spl accounts
        treasuryWsolTokenAccount = await getOrCreateATA(
            connection,
            new PublicKey(constants.WSOL_MINT),
            treasuryAuthority,
            admin,
        );
        treasuryUsdcTokenAccount = await getOrCreateATA(
            connection,
            new PublicKey(constants.USDC_MINT),
            treasuryAuthority,
            admin,
        );
        treasuryUsdtTokenAccount = await getOrCreateATA(
            connection,
            new PublicKey(constants.USDT_MINT),
            treasuryAuthority,
            admin,
        );
        treasuryWbtcTokenAccount = await getOrCreateATA(
            connection,
            new PublicKey(constants.WBTC_MINT),
            treasuryAuthority,
            admin,
        );
        treasuryStfTokenAccount = await getOrCreateATA(
            connection,
            new PublicKey(constants.STF_MINT),
            treasuryAuthority,
            admin,
        );
    });

    // initialize
    it("initialize by admin", async () => {
        let { success, msg, ix } = await saturnV1.initialize(
            admin.publicKey,
        );

        let tx = new Transaction();
        tx.add(ix);
        // console.log(">>> initialize treasury tx : \n", tx);
        try {
            const txId = await provider.sendAndConfirm(tx, [admin], {
                commitment: "confirmed",
                skipPreflight: true
            });

            console.log(">>> initialize treasury transaction = ", txId);
        } catch (error) {
            console.log(error);
        };
    });

    it("init_lending_account by admin", async () => {
        // let reserveAccount = Keypair.generate();
        // let lendingMarket = Keypair.generate();
        // let seed1 = PublicKey.default;
        // let seed2 = PublicKey.default;
        // let obligation = PublicKey.findProgramAddressSync(
        //     [
        //         lendingMarket.publicKey.toBuffer(),
        //         admin.publicKey.toBuffer(),
        //         new anchor.BN(0).toBuffer(),
        //         new anchor.BN(0).toBuffer(),
        //         seed1.toBuffer(),
        //         seed2.toBuffer()
        //     ],
        //     saturnV1.getKlendProgramId()
        // )[0];

        const obligationType = new VanillaObligation(constants.KAMINO_PROGRAM_ID);
        const obligation = obligationType.toPda(kaminoMarket.getAddress(), admin.publicKey);

        let { success, msg, ix } = await saturnV1.initLendingAccounts(
            admin.publicKey,
            marginfiBank.group,
            obligationType.toArgs().seed1,
            obligationType.toArgs().seed2,
            kaminoMarket.getAddress(),
            obligation,
            kaminoUserMetadata,
            marginfiAccountKey
        );

        let tx = new Transaction();
        tx.add(ix);
        // console.log(">>> initLendingAccounts tx : \n", tx);
        try {
            const txId = await provider.sendAndConfirm(tx, [admin], {
                commitment: "confirmed",
                skipPreflight: true
            });

            console.log(">>> initLendingAccounts transaction = ", txId);
        } catch (error) {
            console.log(error);
        };

    });
    it("apply_bond by user", async () => {
        try {
            // Add your test here.
            const depositAmount = new anchor.BN(160);
            const HERMES_URL = "https://hermes.pyth.network/";
            const priceServiceConnection = new PriceServiceConnection(HERMES_URL, {
                priceFeedRequestConfig: { binary: true },
            });
            const pythSolanaReceiver = new PythSolanaReceiver({
                connection: program.provider.connection,
                wallet: new anchor.Wallet(user),
            });

            const priceUpdateData = await priceServiceConnection.getLatestVaas([
                constants.SOL_PRICE_ID,
                constants.USDC_PRICE_ID,
                constants.BONK_PRICE_ID
            ]);

            const transactionBuilder = pythSolanaReceiver.newTransactionBuilder({
                closeUpdateAccounts: true,
            });
            await transactionBuilder.addPostPriceUpdates(priceUpdateData);

            await transactionBuilder.addPriceConsumerInstructions(
                async (
                    getPriceUpdateAccount: (priceFeedId: string) => anchor.web3.PublicKey
                ): Promise<InstructionWithEphemeralSigners[]> => {
                    let { success, msg, ix } = await saturnV1.applyBond(
                        toTokenAmount(10, 6),
                        new anchor.BN(100),
                        user.publicKey,
                        userUsdcTokenAccount,
                        treasuryUsdcTokenAccount,
                        treasuryStfTokenAccount,
                        getPriceUpdateAccount(constants.SOL_PRICE_ID),
                        getPriceUpdateAccount(constants.USDC_PRICE_ID),
                        getPriceUpdateAccount(constants.BONK_PRICE_ID),
                        new PublicKey(constants.USDC_MINT),
                        new PublicKey(constants.STF_MINT)
                    );
                    return [
                        /*
                        saturnV1.applyBond(
                        toTokenAmount(10, 6),
                        new anchor.BN(100),
                        user.publicKey,
                        userUsdcTokenAccount,
                        treasuryUsdcTokenAccount,
                        treasuryStfTokenAccount,
                        priceUpdate: getPriceUpdateAccount(SOL_PRICE_FEED_ID),
                        new PublicKey(constants.USDC_MINT),
                        new PublicKey(constants.STF_MINT)
                        )
                        */
                        {
                            instruction: ix,
                            signers: [admin],
                        },
                    ];
                }
            );
            console.log(">>> \n \n ============== \n", JSON.stringify(transactionBuilder.transactionInstructions));

            await pythSolanaReceiver.provider.sendAll(
                await transactionBuilder.buildVersionedTransactions({
                    computeUnitPriceMicroLamports: 50000,
                }),
                { skipPreflight: true }
            );
        }
        catch (e) {
            console.log("apply_bond error =>", e)
        }
    });

    it("stake by user", async () => {
        let amountToStake = toTokenAmount(5, 2);
        const { success, msg, ix } = await saturnV1.stakeStf(
            amountToStake,
            user.publicKey,
            userStfTokenAccount,
            treasuryStfTokenAccount,
            new PublicKey(constants.STF_MINT),
        );
        let tx = new Transaction();
        tx.add(ix);
        // console.log(">>> stakeStf tx : \n", tx);

        try {
            const txId = await provider.sendAndConfirm(tx, [user], {
                commitment: "confirmed",
                skipPreflight: true
            });

            console.log(">>> stakeStf transaction = ", txId);
        } catch (error) {
            console.log(error);
        };
    });

    // initial workflow
    it("calculate balance by admin (initial workflow)", async () => {
        try {
            // Add your test here.
            const depositAmount = new anchor.BN(160);
            const HERMES_URL = "https://hermes.pyth.network/";
            const priceServiceConnection = new PriceServiceConnection(HERMES_URL, {
                priceFeedRequestConfig: { binary: true },
            });
            const pythSolanaReceiver = new PythSolanaReceiver({
                connection: program.provider.connection,
                wallet: new anchor.Wallet(user),
            });

            const priceUpdateData = await priceServiceConnection.getLatestVaas([
                constants.SOL_PRICE_ID,
                constants.USDC_PRICE_ID,
                constants.WBTC_PRICE_ID
            ]);

            const transactionBuilder = pythSolanaReceiver.newTransactionBuilder({
                closeUpdateAccounts: true,
            });
            await transactionBuilder.addPostPriceUpdates(priceUpdateData);

            await transactionBuilder.addPriceConsumerInstructions(
                async (
                    getPriceUpdateAccount: (priceFeedId: string) => anchor.web3.PublicKey
                ): Promise<InstructionWithEphemeralSigners[]> => {
                    let { success, msg, ix } = await saturnV1.calcuBalance(
                        admin.publicKey,
                        treasuryUsdcTokenAccount,
                        treasuryWbtcTokenAccount,
                        getPriceUpdateAccount(constants.SOL_PRICE_ID),
                        getPriceUpdateAccount(constants.USDC_PRICE_ID),
                        getPriceUpdateAccount(constants.WBTC_PRICE_ID),
                    )
                    return [
                        {
                            instruction: ix,
                            signers: [admin],
                        },
                    ];
                }
            );
            console.log(">>> \n \n ============== \n", JSON.stringify(transactionBuilder.transactionInstructions));

            await pythSolanaReceiver.provider.sendAll(
                await transactionBuilder.buildVersionedTransactions({
                    computeUnitPriceMicroLamports: 50000,
                }),
                { skipPreflight: true }
            );
        }
        catch (e) {
            console.log("calcuBalance error =>", e)
        }
    });

    it("reallocate by admin (initial workflow)", async () => {
        let { success, msg, ix } = await saturnV1.reallocate(
            [0.3, 0.2, 0.2, 0.1, 0.2, 0.1, 0.1],
            [0.3, 0.2, 0.2, 0.1, 0.2, 0.1, 0.1],
            admin.publicKey,
        )

        let tx = new Transaction();
        tx.add(ix);
        // console.log(">>> reallocate tx : \n", tx);

        try {
            const txId = await provider.sendAndConfirm(tx, [admin], {
                commitment: "confirmed",
                skipPreflight: true
            });

            console.log(">>> reallocate transaction = ", txId);
        } catch (error) {
            console.log(error);
        };
    });

    let result: any;

    it("swap for all in usdc (initial workflow)", async () => {
        // ### sol -> usdc
        let fromAmount = await getSolBalance(connection, treasuryAuthority);
        // Find the best Quote from the Jupiter API
        let quote = await getQuote(new PublicKey(constants.WSOL_MINT), new PublicKey(constants.USDC_MINT), fromAmount); // getQuote(fromMint, toMint, amount)
        // const quote = JSON.parse('{"inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","inAmount":"1000000","outputMint":"So11111111111111111111111111111111111111112","outAmount":"6015074","otherAmountThreshold":"5984999","swapMode":"ExactIn","slippageBps":50,"platformFee":null,"priceImpactPct":"0.0000251694641401135192706162","routePlan":[{"swapInfo":{"ammKey":"8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj","label":"Raydium CLMM","inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","outputMint":"So11111111111111111111111111111111111111112","inAmount":"1000000","outAmount":"6015074","feeAmount":"84","feeMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"},"percent":100}],"contextSlot":269097074,"timeTaken":0.002004022}')
        console.log(">>> get quote SOL->USDC 1000000 : ", JSON.stringify(quote));
        // Convert the Quote into a Swap instruction
        result = await getSwapIx(treasuryAuthority, treasuryUsdcTokenAccount, quote);
        // console.log(">>> getSwapIx = \n", result);

        if ("error" in result) {
            console.log({ result });
            return result;
        }

        // We have now both the instruction and the lookup table addresses.
        let {
            computeBudgetInstructions, // The necessary instructions to setup the compute budget.
            swapInstruction, // The actual swap instruction.
            addressLookupTableAddresses, // The lookup table addresses that you can use if you are using versioned transaction.
        } = result;

        await swap(
            connection,
            provider,
            saturnV1,
            computeBudgetInstructions,
            swapInstruction,
            addressLookupTableAddresses,
            admin,
            new PublicKey(constants.WSOL_MINT),
            treasuryWsolTokenAccount,
            new PublicKey(constants.USDC_MINT),
            treasuryUsdcTokenAccount,
            new anchor.BN(fromAmount)
        );

        // ### usdt -> usdc
        fromAmount = await getBalance(connection, treasuryUsdtTokenAccount);
        // Find the best Quote from the Jupiter API
        quote = await getQuote(new PublicKey(constants.USDT_MINT), new PublicKey(constants.USDC_MINT), fromAmount); // getQuote(fromMint, toMint, amount)
        // const quote = JSON.parse('{"inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","inAmount":"1000000","outputMint":"So11111111111111111111111111111111111111112","outAmount":"6015074","otherAmountThreshold":"5984999","swapMode":"ExactIn","slippageBps":50,"platformFee":null,"priceImpactPct":"0.0000251694641401135192706162","routePlan":[{"swapInfo":{"ammKey":"8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj","label":"Raydium CLMM","inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","outputMint":"So11111111111111111111111111111111111111112","inAmount":"1000000","outAmount":"6015074","feeAmount":"84","feeMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"},"percent":100}],"contextSlot":269097074,"timeTaken":0.002004022}')
        console.log(">>> get quote USDT->USDC : ", JSON.stringify(quote));
        // Convert the Quote into a Swap instruction
        result = await getSwapIx(treasuryAuthority, treasuryUsdcTokenAccount, quote);
        // console.log(">>> getSwapIx = \n", result);

        if ("error" in result) {
            console.log({ result });
            return result;
        }

        // We have now both the instruction and the lookup table addresses.
        let {
            computeBudgetInstructions1, // The necessary instructions to setup the compute budget.
            swapInstruction1, // The actual swap instruction.
            addressLookupTableAddresses1, // The lookup table addresses that you can use if you are using versioned transaction.
        } = result;

        await swap(
            connection,
            provider,
            saturnV1,
            computeBudgetInstructions1,
            swapInstruction1,
            addressLookupTableAddresses1,
            admin,
            new PublicKey(constants.USDT_MINT),
            treasuryUsdtTokenAccount,
            new PublicKey(constants.USDC_MINT),
            treasuryUsdcTokenAccount,
            new anchor.BN(fromAmount)
        );

        // ### wbtc -> usdc
        fromAmount = await getBalance(connection, treasuryWbtcTokenAccount);
        // Find the best Quote from the Jupiter API
        quote = await getQuote(new PublicKey(constants.WBTC_MINT), new PublicKey(constants.USDC_MINT), fromAmount); // getQuote(fromMint, toMint, amount)
        // const quote = JSON.parse('{"inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","inAmount":"1000000","outputMint":"So11111111111111111111111111111111111111112","outAmount":"6015074","otherAmountThreshold":"5984999","swapMode":"ExactIn","slippageBps":50,"platformFee":null,"priceImpactPct":"0.0000251694641401135192706162","routePlan":[{"swapInfo":{"ammKey":"8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj","label":"Raydium CLMM","inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","outputMint":"So11111111111111111111111111111111111111112","inAmount":"1000000","outAmount":"6015074","feeAmount":"84","feeMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"},"percent":100}],"contextSlot":269097074,"timeTaken":0.002004022}')
        console.log(">>> get quote WBTC->USDC : ", JSON.stringify(quote));
        // Convert the Quote into a Swap instruction
        result = await getSwapIx(treasuryAuthority, treasuryUsdcTokenAccount, quote);
        // console.log(">>> getSwapIx = \n", result);

        if ("error" in result) {
            console.log({ result });
            return result;
        }

        // We have now both the instruction and the lookup table addresses.
        let {
            computeBudgetInstructions2, // The necessary instructions to setup the compute budget.
            swapInstruction2, // The actual swap instruction.
            addressLookupTableAddresses2, // The lookup table addresses that you can use if you are using versioned transaction.
        } = result;

        await swap(
            connection,
            provider,
            saturnV1,
            computeBudgetInstructions2,
            swapInstruction2,
            addressLookupTableAddresses2,
            admin,
            new PublicKey(constants.WBTC_MINT),
            treasuryWbtcTokenAccount,
            new PublicKey(constants.USDC_MINT),
            treasuryUsdcTokenAccount,
            new anchor.BN(fromAmount)
        );

    });



    it("klend_lend (initial workflow)", async () => {
        // let reserveAccount = Keypair.generate();
        // let lendingMarket = Keypair.generate();
        // let seed1 = PublicKey.default;
        // let seed2 = PublicKey.default;
        // let obligation = PublicKey.findProgramAddressSync(
        //     [
        //         lendingMarket.publicKey.toBuffer(),
        //         admin.publicKey.toBuffer(),
        //         new anchor.BN(0).toBuffer(),
        //         new anchor.BN(0).toBuffer(),
        //         seed1.toBuffer(),
        //         seed2.toBuffer()
        //     ],
        //     saturnV1.getKlendProgramId()
        // )[0];
        // let lendingMarketAuth = PublicKey.findProgramAddressSync(
        //     [
        //         Buffer.from(constants.LENDING_MARKET_AUTH),
        //         lendingMarket.publicKey.toBuffer(),
        //     ],
        //     saturnV1.getKlendProgramId()
        // )[0];
        const obligation = await kaminoMarket.getObligationByWallet(admin.publicKey, new VanillaObligation(constants.KAMINO_PROGRAM_ID));

        let { success, msg, ix } = await saturnV1.klendLend(
            admin.publicKey,
            obligation.obligationAddress,
            kaminoMarket.getAddress(),
            kaminoMarket.getLendingMarketAuthority(),
            kaminoUsdcRerserve.address,
            kaminoUsdcRerserve.state.liquidity.supplyVault,
            kaminoUsdcRerserve.state.collateral.mintPubkey,
            kaminoUsdcRerserve.state.collateral.supplyVault,
            await getOrCreateATA(connection, kaminoUsdcRerserve.state.liquidity.mintPubkey, treasuryAuthority, admin),
            await getOrCreateATA(connection, kaminoUsdcRerserve.state.collateral.mintPubkey, treasuryAuthority, admin),
        );
        let tx = new Transaction();
        tx.add(ix);
        // console.log(">>> klendLend tx : \n", tx);

        try {
            const txId = await provider.sendAndConfirm(tx, [admin], {
                commitment: "confirmed",
                skipPreflight: true
            });

            console.log(">>> klendLend transaction = ", txId);
        } catch (error) {
            console.log(error);
        };
    });

    it("swap for marginfi_lend in usdt (initial workflow)", async () => {
        // ### usdc -> usdt
        const fromAmount = treasury;
        // Find the best Quote from the Jupiter API
        const quote = await getQuote(new PublicKey(constants.WSOL_MINT), new PublicKey(constants.USDC_MINT), fromAmount); // getQuote(fromMint, toMint, amount)
        // const quote = JSON.parse('{"inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","inAmount":"1000000","outputMint":"So11111111111111111111111111111111111111112","outAmount":"6015074","otherAmountThreshold":"5984999","swapMode":"ExactIn","slippageBps":50,"platformFee":null,"priceImpactPct":"0.0000251694641401135192706162","routePlan":[{"swapInfo":{"ammKey":"8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj","label":"Raydium CLMM","inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","outputMint":"So11111111111111111111111111111111111111112","inAmount":"1000000","outAmount":"6015074","feeAmount":"84","feeMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"},"percent":100}],"contextSlot":269097074,"timeTaken":0.002004022}')
        console.log(">>> get quote SOL->USDC 1000000 : ", JSON.stringify(quote));
        // Convert the Quote into a Swap instruction
        result = await getSwapIx(treasuryAuthority, treasuryUsdcTokenAccount, quote);
        // console.log(">>> getSwapIx = \n", result);

        if ("error" in result) {
            console.log({ result });
            return result;
        }

        // We have now both the instruction and the lookup table addresses.
        let {
            computeBudgetInstructions, // The necessary instructions to setup the compute budget.
            swapInstruction, // The actual swap instruction.
            addressLookupTableAddresses, // The lookup table addresses that you can use if you are using versioned transaction.
        } = result;

        await swapToSol(
            connection,
            computeBudgetInstructions,
            swapInstruction,
            addressLookupTableAddresses
        );

    });
    it("marginfi_lend (initial workflow)", async () => { });

    it("swap for meteora_deposit in usdt (initial workflow)", async () => { });
    it("meteora_deposit (initial workflow)", async () => { });

    it("swap for JLP (initial workflow)", async () => { });
    it("swap for WBTC (initial workflow)", async () => { });
    it("swap for SOL (initial workflow)", async () => { });


    // workflow 2
    it("get_value_in_kamino (workflow2)", async () => { });
    it("get_value_in_marginfi (workflow2)", async () => { });
    it("get_value_in_meteora (workflow2)", async () => { });

    it("calculate balance (workflow2)", async () => { });
    it("reallocate (workflow2)", async () => { });

    it("klend_withdraw (workflow2)", async () => { });
    it("marginfi_withdraw (workflow2)", async () => { });
    it("meteora_withdraw (workflow2)", async () => { });

    it("swap for all in usdc (workflow2)", async () => { });
    it("klend_lend (workflow2)", async () => { });

    it("swap for marginfi_lend in usdt (workflow2)", async () => { });
    it("marginfi_lend (workflow2)", async () => { });

    it("swap for meteora_deposit in usdt (workflow2)", async () => { });
    it("meteora_deposit (workflow2)", async () => { });

    it("swap for JLP (workflow2)", async () => { });
    it("swap for WBTC (workflow2)", async () => { });
    it("swap for SOL (workflow2)", async () => { });

    // at the end
    it("unstake by user", async () => { });
    it("finish_bond by user", async () => { });
    it("cashingout_redeem by user", async () => { });
});




const API_ENDPOINT = "https://quote-api.jup.ag/v6";

const getQuote = async (
    fromMint: PublicKey,
    toMint: PublicKey,
    amount: number
) => {
    return fetch(
        `${API_ENDPOINT}/quote?outputMint=${toMint.toBase58()}&inputMint=${fromMint.toBase58()}&amount=${amount}&slippage=0.5&onlyDirectRoutes=true`
    ).then((response) => response.json());
};

const getSwapIx = async (
    user: PublicKey,
    outputAccount: PublicKey,
    quote: any
) => {
    const data = {
        quoteResponse: quote,
        userPublicKey: user.toBase58(),
        destinationTokenAccount: outputAccount.toBase58(),
        useSharedAccounts: true,
    };
    return fetch(`${API_ENDPOINT}/swap-instructions`, {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    }).then((response) => response.json());
};


export const instructionDataToTransactionInstruction = (
    instructionPayload: any
) => {
    if (instructionPayload === null) {
        return null;
    }

    return new TransactionInstruction({
        programId: new PublicKey(instructionPayload.programId),
        keys: instructionPayload.accounts.map((key: { pubkey: anchor.web3.PublicKeyInitData; isSigner: any; isWritable: any; }) => ({
            pubkey: new PublicKey(key.pubkey),
            isSigner: key.isSigner,
            isWritable: key.isWritable,
        })),
        data: Buffer.from(instructionPayload.data, "base64"),
    });
};

const getAdressLookupTableAccounts = async (
    connection: Connection,
    keys: string[]
): Promise<AddressLookupTableAccount[]> => {
    const addressLookupTableAccountInfos =
        await connection.getMultipleAccountsInfo(
            keys.map((key) => new PublicKey(key))
        );

    return addressLookupTableAccountInfos.reduce((acc, accountInfo, index) => {
        const addressLookupTableAddress = keys[index];
        if (accountInfo) {
            const addressLookupTableAccount = new AddressLookupTableAccount({
                key: new PublicKey(addressLookupTableAddress),
                state: AddressLookupTableAccount.deserialize(accountInfo.data),
            });
            acc.push(addressLookupTableAccount);
        }

        return acc;
    }, new Array<AddressLookupTableAccount>());
};

const swap = async (
    connection: Connection,
    provider: anchor.Provider,
    saturnV1: SaturnV1Impl,
    computeBudgetPayloads: any[],
    swapPayload: any,
    addressLookupTableAddresses: string[],
    signer: Keypair,
    fromMint: PublicKey,
    fromMintAccount: PublicKey,
    toMint: PublicKey,
    toMintAccount: PublicKey,
    fromAmount: anchor.BN,
) => {
    let swapInstruction = instructionDataToTransactionInstruction(swapPayload);
    let { success, msg, ix } = await saturnV1.swap(
        swapInstruction.data,
        fromAmount,
        signer.publicKey,
        fromMintAccount,
        fromMint,
        toMintAccount,
        toMint
    );

    const instructions = [
        ...computeBudgetPayloads.map(instructionDataToTransactionInstruction),
        ix
    ];

    const blockhash = (await connection.getLatestBlockhash()).blockhash;

    // If you want, you can add more lookup table accounts here
    const addressLookupTableAccounts = await getAdressLookupTableAccounts(
        connection,
        addressLookupTableAddresses
    );
    const messageV0 = new TransactionMessage({
        payerKey: signer.publicKey,
        recentBlockhash: blockhash,
        instructions,
    }).compileToV0Message(addressLookupTableAccounts);
    const transaction = new VersionedTransaction(messageV0);

    try {
        await provider.simulate(transaction, [signer]);

        const txID = await provider.sendAndConfirm(transaction, [signer]);
        console.log({ txID });
    } catch (e) {
        console.log({ simulationResponse: e.simulationResponse });
    }
};