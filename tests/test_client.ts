import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SaturnV1 } from "../target/types/saturn_v_1";
import {
    PublicKey,
    Keypair,
    SystemProgram,
    AccountMeta,
    SYSVAR_RENT_PUBKEY,
    SYSVAR_SLOT_HASHES_PUBKEY,
    SYSVAR_INSTRUCTIONS_PUBKEY,
    AddressLookupTableProgram,
    TransactionMessage,
    VersionedTransaction,
} from "@solana/web3.js";
import * as spl from "@solana/spl-token";
import fetch from "node-fetch";
import adminJson from "./users/admin.json";
import userJson from "./users/user.json";
import stfTokenMintJson from "./mint/stf_token_mint.json";
import { BN } from "bn.js";
import { PriceServiceConnection } from "@pythnetwork/price-service-client";
import { InstructionWithEphemeralSigners, PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver"

const TREASURY_AUTHORITY_SEED = "treasury-authority";
const TREASURY_SEED = "global-treasury-2";
const PERSONAL_SEED = "personal-saturn";

const USDC_TOKEN_MINT = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const BONK_TOKEN_MINT = "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263";

export class TestClient {
    provider: anchor.AnchorProvider;
    program: anchor.Program<SaturnV1>;
    printErrors: boolean;

    admin: Keypair;
    user: Keypair;
    stfTokenMint: Keypair;

    // pdas
    treasuryAuthority: { publicKey: PublicKey; bump: number };
    treasury: { publicKey: PublicKey; bump: number };

    //token accounts
    treasuryAuthorityStfTokenAccount: PublicKey;
    treasuryAuthorityUsdcTokenAccount: PublicKey;
    treasuryAuthorityBonkTokenAccount: PublicKey;
    userStfTokenAccount: PublicKey;
    userUsdcTokenAccount: PublicKey;
    userBonkTokenAccount: PublicKey;

    constructor() {
        this.provider = anchor.AnchorProvider.env();
        anchor.setProvider(this.provider);
        this.program = anchor.workspace.SaturnV1 as anchor.Program<SaturnV1>;
        this.printErrors = true;

        anchor.BN.prototype.toJSON = function () {
            return this.toString(10);
        };
    }

    requestAirdrop = async (pubkey: PublicKey) => {
        if ((await this.getSolBalance(pubkey)) < 1e9 / 2) {
            return this.provider.connection.requestAirdrop(pubkey, 1e9);
        }
    };

    mintTokens = async (
        uiAmount: number,
        decimals: number,
        mint: PublicKey,
        destiantionWallet: PublicKey
    ) => {
        await spl.mintToChecked(
            this.provider.connection,
            this.admin,
            mint,
            destiantionWallet,
            this.admin,
            this.toTokenAmount(uiAmount, decimals).toNumber(),
            decimals
        );
    };

    findProgramAddress = (label: string, extraSeeds = null) => {
        let seeds = [Buffer.from(anchor.utils.bytes.utf8.encode(label))];
        if (extraSeeds) {
            for (let extraSeed of extraSeeds) {
                if (typeof extraSeed === "string") {
                    seeds.push(Buffer.from(anchor.utils.bytes.utf8.encode(extraSeed)));
                } else if (Array.isArray(extraSeed)) {
                    seeds.push(Buffer.from(extraSeed));
                } else {
                    seeds.push(extraSeed.toBuffer());
                }
            }
        }
        let res = PublicKey.findProgramAddressSync(seeds, this.program.programId);
        return { publicKey: res[0], bump: res[1] };
    };

    confirmTx = async (txSignature: anchor.web3.TransactionSignature) => {
        const latestBlockHash = await this.provider.connection.getLatestBlockhash();

        await this.provider.connection.confirmTransaction(
            {
                blockhash: latestBlockHash.blockhash,
                lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
                signature: txSignature,
            },
            { commitment: "processed" }
        );
    };

    confirmAndLogTx = async (txSignature: anchor.web3.TransactionSignature) => {
        await this.confirmTx(txSignature);
        let tx = await this.provider.connection.getTransaction(txSignature, {
            commitment: "confirmed",
        });
        console.log(tx);
    };

    getBalance = async (pubkey: PublicKey) => {
        return spl
            .getAccount(this.provider.connection, pubkey)
            .then((account) => Number(account.amount))
            .catch(() => 0);
    };

    getSolBalance = async (pubkey: PublicKey) => {
        return this.provider.connection
            .getBalance(pubkey)
            .then((balance) => balance)
            .catch(() => 0);
    };

    getExtraSolBalance = async (pubkey: PublicKey) => {
        let balance = await this.provider.connection
            .getBalance(pubkey)
            .then((balance) => balance)
            .catch(() => 0);
        let accountInfo = await this.provider.connection.getAccountInfo(pubkey);
        let dataSize = accountInfo ? accountInfo.data.length : 0;
        let minBalance =
            await this.provider.connection.getMinimumBalanceForRentExemption(
                dataSize
            );
        return balance > minBalance ? balance - minBalance : 0;
    };

    getTokenAccount = async (pubkey: PublicKey) => {
        return spl.getAccount(this.provider.connection, pubkey);
    };

    getTime() {
        const now = new Date();
        const utcMilllisecondsSinceEpoch =
            now.getTime() + now.getTimezoneOffset() * 60 * 1000;
        return utcMilllisecondsSinceEpoch / 1000;
    }

    toTokenAmount(uiAmount: number, decimals: number) {
        return new BN(uiAmount * 10 ** decimals);
    }

    toUiAmount(token_amount: number, decimals: number) {
        return token_amount / 10 ** decimals;
    }

    ensureFails = async (promise, message = null) => {
        let printErrors = this.printErrors;
        this.printErrors = false;
        let res = null;
        try {
            await promise;
        } catch (err) {
            res = err;
        }
        this.printErrors = printErrors;
        if (!res) {
            throw new Error(message ? message : "Call should've failed");
        }
        return res;
    };

    initTestClient = async () => {
        // fixed addresses
        this.admin = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(adminJson));
        console.log(">>> create admin publickey : ", this.admin.publicKey.toBase58());
        //5vSwrp6mk5Po9d4L9uN6Vd18w26wVdrsVHjcNJKX62aG
        this.user = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(userJson));
        console.log(">>> create user publickey : ", this.user.publicKey.toBase58());
        //3qtahdn6ez4hwRy3UwEC3Cf9pPdbMeGUvspWvgY2N6Ws
        this.stfTokenMint = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(stfTokenMintJson));
        console.log(">>> create stf token mint publickey : ", this.stfTokenMint.publicKey.toBase58());
        //2qTMAdL7bC3kuNEvEpdbaEMLhyAtLDe2cPRamkRhkkwq

        //pdas
        this.treasury = this.findProgramAddress(TREASURY_SEED);
        this.treasuryAuthority = this.findProgramAddress(TREASURY_AUTHORITY_SEED);

        // airdrop funds
        await this.confirmTx(await this.requestAirdrop(this.admin.publicKey));
        await this.confirmTx(await this.requestAirdrop(this.user.publicKey));
        await this.confirmTx(await this.requestAirdrop(this.treasuryAuthority.publicKey));

        // stf token mint
        await spl.createMint(
            this.provider.connection,
            this.admin,
            this.admin.publicKey,
            null,
            2,
            this.stfTokenMint
        );

        this.treasuryAuthorityStfTokenAccount = await spl.createAssociatedTokenAccount(
            this.provider.connection,
            this.admin,
            this.stfTokenMint.publicKey,
            this.treasuryAuthority.publicKey
        );
        this.userStfTokenAccount = await spl.createAssociatedTokenAccount(
            this.provider.connection,
            this.admin,
            this.stfTokenMint.publicKey,
            this.user.publicKey
        );

        await this.mintTokens(
            1000,
            2,
            this.stfTokenMint.publicKey,
            this.treasuryAuthorityStfTokenAccount
        );
        await this.mintTokens(
            1000,
            2,
            this.stfTokenMint.publicKey,
            this.userStfTokenAccount
        );

        this.treasuryAuthorityUsdcTokenAccount = await spl.createAssociatedTokenAccount(
            this.provider.connection,
            this.admin,
            new PublicKey(USDC_TOKEN_MINT),
            this.treasuryAuthority.publicKey
        );
        this.treasuryAuthorityBonkTokenAccount = await spl.createAssociatedTokenAccount(
            this.provider.connection,
            this.admin,
            new PublicKey(BONK_TOKEN_MINT),
            this.treasuryAuthority.publicKey
        );

        let response = await this.provider.connection.getParsedTokenAccountsByOwner( this.user.publicKey, { mint: new PublicKey(USDC_TOKEN_MINT) });
        this.userUsdcTokenAccount = response.value[0].pubkey;
        response = await this.provider.connection.getParsedTokenAccountsByOwner( this.user.publicKey, { mint: new PublicKey(BONK_TOKEN_MINT) });
        this.userBonkTokenAccount = response.value[0].pubkey;


    }
}