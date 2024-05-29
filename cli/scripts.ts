import { Program,  web3 } from '@coral-xyz/anchor';
import * as anchor from '@coral-xyz/anchor';
import {
    Keypair,
    PublicKey,
    SystemProgram,
    Transaction,
} from '@solana/web3.js';
import fs from 'fs';

import NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet';

import { TOKEN_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/utils/token';
import { ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { IDL as SaturnIDL, Saturn } from "./raffle";

export const RPC_URL = "https://api.devnet.solana.com";

export const TREASURY_SEED = "global-treasury-2";
export const EMPTY_USER = "11111111111111111111111111111111";

export const PROGRAM_ID = "HqWuLVZLBZ5MbDNvLqieWiERNNmpTBG7q5t99CtmGYQa";
export const STF_TOKEN = "3HWcdN9fxD3ytB7L2FG5c3WJXQin3QFUNZoESCQriLD7";
export const ESCROW_SIZE = 112;
export const DECIMALS = 100;
export const DAY = 3600 * 24 * 1000


// Set the initial program and provider
let program: Program = null;
let provider: anchor.Provider = null;

// Address of the deployed program.
let programId = new anchor.web3.PublicKey(PROGRAM_ID);

anchor.setProvider(anchor.AnchorProvider.local(web3.clusterApiUrl("devnet")));
provider = anchor.getProvider();

let solConnection = anchor.getProvider().connection;

// Generate the program client from IDL.
program = new anchor.Program(SaturnIDL as anchor.Idl, programId);
console.log('ProgramId: ', program.programId.toBase58());


const main = async () => {
    const [globalAuthority, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(TREASURY_SEED)],
        program.programId
    );
    console.log('Treasury: ', globalAuthority.toBase58());

    console.log(provider.publicKey.toBase58());
    try {
        // const key = await getRaffleKey(new PublicKey("HYX4tS54K7d5SEtTRvwsFD5h8EamtiQqdfewX8ixQeDa"));
        // const poolInfo = await getStateByKey(key);
        // console.log(await getAllData());
        // await updateRafflePeriod(provider.publicKey, new PublicKey("6jQaq4t97KjghTcfZWMXZoimNNCtSGikeaTtCktDm5aK"), 1689099000)
        // console.log(poolInfo.whitelisted.toNumber())
        // console.log(await getStateByKey(key));
        // await initProject();
        await createBond();        
    } catch (e) {
        console.log(e);
    }
};

export const globalAuthority = async () => {
    const [globalAuthority, bump] = await PublicKey.findProgramAddressSync(
        [Buffer.from(TREASURY_SEED)],
        new PublicKey(PROGRAM_ID)
    );
    return globalAuthority.toBase58();
}

export const initProject = async () => {

    const [treasury, bump] = await PublicKey.findProgramAddressSync(
        [Buffer.from(TREASURY_SEED)],
        program.programId
    );

    let tx = new Transaction();

    console.log(provider.publicKey.toBase58(), treasury.toBase58());

    const ix = await program.methods.initialize().accounts({
        admin: provider.publicKey,
        treasury,
        systemProgram: SystemProgram.programId,
    }).instruction();
    tx.add(ix);

    const txId = await provider.sendAndConfirm(tx, [], {
        commitment: "confirmed",
    });
    
    console.log("txHash =", txId);

    return true;
}

/**
 * @dev CreateRaffle function
 * @param userAddress The raffle creator's address
 * @param nft_mint The nft_mint address
 * @param ticketPriceSol The ticket price by SOL 
 * @param ticketPriceApe The ticket price by SOLAPE token
 * @param endTimestamp The raffle end timestamp
 * @param max The max entrants of this raffle
 */

export const createBond = async () => {
    try {

        const tx = await createBondTx(
            program,
            provider.publicKey,
            new PublicKey("9cmYPgxT1wGP6ySgSDHCmTrLYzeDp1iVssy4grjdjDyQ"), //assume USDC
            1000, //tokenAmount,
            15 * 10 ** 6 //spot_price per small unit for STF which is DECIMAL of 2 (8 - 6)
        );
        const txId = await provider.sendAndConfirm(tx, [], {
            commitment: "confirmed",
        });
        console.log("txHash =", txId);
        // setLoading(false);
    } catch (error) {
        console.log(error);
        // setLoading(false);

    }
}
export const finishBond = async (
) => {
    try {

        const tx = await finishBondTx(
            program,
            provider.publicKey            
        );
        const txId = await provider.sendAndConfirm(tx, [], {
            commitment: "confirmed",
        });
        console.log("txHash =", txId);
        // setLoading(false);
    } catch (error) {
        console.log(error);
        // setLoading(false);

    }
}
//7S7KBb39kSH2wa6FwxB6Nk7YwpkFV5ZnVssFmhVPXvsP
export const createBondTx = async (
    program: anchor.Program,
    userAddress: PublicKey | undefined,
    escrow_mint: PublicKey,
    tokenAmount: number,
    spot_price: number,
) => {
    if (!userAddress) return;
    console.log("HERE", userAddress);
    const [treasury, bump] = await PublicKey.findProgramAddressSync(
        [Buffer.from(TREASURY_SEED)],
        program.programId
    );
    console.log(treasury.toBase58());
    //Source Token Account 
    let ownerNftAccount = await getAssociatedTokenAccount(userAddress, escrow_mint);
    console.log("Src USDC Account = ", ownerNftAccount.toBase58());
    let ix0 = await getATokenAccountsNeedCreate(
        solConnection,
        userAddress,
        treasury,
        [escrow_mint]
    );
    console.log("Dest USDC Account = ", ix0.destinationAccounts[0].toBase58());

    //Create STF Token Account on the user and on the treasury

    let ix1 = await getATokenAccountsNeedCreate(
        solConnection,
        userAddress,
        userAddress,
        [new PublicKey(STF_TOKEN)]
    );

    console.log("Creator STF Account = ", ix1.destinationAccounts[0].toBase58());

    let ix3 = await getATokenAccountsNeedCreate(
        solConnection,
        userAddress,
        treasury,
        [new PublicKey(STF_TOKEN)]
    );

    console.log("Treasury STF Account = ", ix3.destinationAccounts[0].toBase58());

    let escrow;
    let i;

    for (i = 11; i > 0; i--) {
        escrow = await PublicKey.createWithSeed(
            userAddress,
            escrow_mint.toBase58().slice(0, i),
            program.programId,
        );
        // let state = await getStateByKey(escrow);
        // if (state === null) {
        //     console.log(i);
        //     break;
        // }
        break;
    }

    console.log(escrow?.toBase58());
    let tx = new Transaction();
    if (escrow) {
        console.log(program.programId.toBase58());
        let ix = SystemProgram.createAccountWithSeed({
            fromPubkey: userAddress,
            basePubkey: userAddress,
            seed: escrow_mint.toBase58().slice(0, i),
            newAccountPubkey: escrow,
            lamports: await solConnection.getMinimumBalanceForRentExemption(ESCROW_SIZE),
            space: ESCROW_SIZE,
            programId: program.programId,
        });

        // let price_update = await PublicKey.createWithSeed(
        //     userAddress,
        //     "PRICE_ACCOUNT___",
        //     new PublicKey("rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ"),
        // );
        // let ix_for_price = SystemProgram.createAccountWithSeed({
        //     fromPubkey: userAddress,
        //     basePubkey: userAddress,
        //     seed: "PRICE_ACCOUNT___",
        //     newAccountPubkey: price_update,
        //     lamports: await solConnection.getMinimumBalanceForRentExemption(134),
        //     space: 134,
        //     programId: new PublicKey("rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ"),
        // });
        // console.log(ix_for_price.keys);
        // tx.add(ix_for_price);
        // //5SRy5arRpGmeohJGeF2UvT4aTuFio38qRsDevJ5mTC5C returned for owner of oracle
        // return tx;

        tx.add(ix);
        if (ix0.instructions.length > 0 && escrow_mint.toBase58() != EMPTY_USER) tx.add(...ix0.instructions)
        if (ix1.instructions.length > 0) tx.add(...ix1.instructions)
        if (ix3.instructions.length > 0) tx.add(...ix3.instructions)

        const applyBondArgs = {
            token_amount: new anchor.BN(100 * 10 ** 2),    // replace with your desired amount
            spot_price: new anchor.BN(15),      // replace with the spot price
        }

        const ix2 = await program.instruction.applyBond(new anchor.BN(tokenAmount), new anchor.BN(spot_price), bump, 
            {//Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX
                accounts: {
                    admin: userAddress,
                    treasury: treasury,
                    escrow,
                    createrTokenAccount: ownerNftAccount,
                    destTokenAccount: ix0.destinationAccounts[0],
                    destStfAccount: ix3.destinationAccounts[0],
                    priceUpdate: new PublicKey("Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX"),
                    tokenMintAddress: escrow_mint,
                    stfTokenMint: new PublicKey(STF_TOKEN),
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId
                },
                instructions: [],
                signers: []
            })
        tx.add(ix2);
    }

    console.log(tx);
    return tx;
}

export const finishBondTx = async (
    program: anchor.Program,
    userAddress: PublicKey | undefined,
) => {
    if (!userAddress) return;    
    const [treasury, bump] = await PublicKey.findProgramAddressSync(
        [Buffer.from(TREASURY_SEED)],
        program.programId
    );    

    let ix3 = await getATokenAccountsNeedCreate(
        solConnection,
        userAddress,
        userAddress,
        [new PublicKey(STF_TOKEN)]
    );

    console.log("userAddress STF Account = ", ix3.destinationAccounts[0].toBase58());

    let escrow = new PublicKey("Bcs4HEbeBgXxy2TFLRTZ4SMWR6dapyRrLr6qVJ93wuxA");
    console.log(escrow?.toBase58());

    let tx = new Transaction();
    if (escrow) {
        if (ix3.instructions.length > 0) tx.add(...ix3.instructions)

        const ix2 = await program.instruction.finishBond(bump, 
            {//Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX
                accounts: {
                    admin: userAddress,
                    treasury: treasury,
                    destStfAccount: ix3.destinationAccounts[0],
                    escrow,
                    stfTokenMint: new PublicKey(STF_TOKEN),
                    tokenProgram: TOKEN_PROGRAM_ID                    
                },
                instructions: [],
                signers: []
            })
        tx.add(ix2);
    }

    console.log(tx);
    return tx;
}

// var byteArrayToInt = function (byteArray: Buffer) {
//     var value = 0;
//     for (var i = 0; i <= byteArray.length - 1; i++) {
//         value = (value * 256) + byteArray[i];
//     }
//     return value;
// };

// export const getAllData = async () => {
//     let cloneWindow: any = window;
//     let provider = new anchor.AnchorProvider(solConnection, cloneWindow['solana'], anchor.AnchorProvider.defaultOptions())
//     const program = new anchor.Program(RaffleIDL as anchor.Idl, PROGRAM_ID, provider);
//     let poolAccounts = await solConnection.getProgramAccounts(
//         program.programId,
//         {
//             filters: [
//                 {
//                     dataSize: RAFFLE_SIZE,
//                 },
//             ]
//         }
//     );

//     let result = [];
//     for (let i = 0; i < poolAccounts.length; i++) {
//         const data = poolAccounts[i].account.data;

//         const creator = new PublicKey(data.slice(8, 40));
//         const token_program = new PublicKey(data.slice(40, 72));
//         const token_mint = new PublicKey(data.slice(72, 104));
//         let buf = data.slice(104, 112).reverse();
//         const token_amount = byteArrayToInt(buf);
//         const ticket_token_program = new PublicKey(data.slice(112, 144));

//         buf = data.slice(144, 152).reverse();
//         const ticket_price = byteArrayToInt(buf);

//         buf = data.slice(152, 160).reverse();
//         const count = byteArrayToInt(buf);

//         buf = data.slice(160, 168).reverse();
//         const no_repeat = byteArrayToInt(buf);

//         buf = data.slice(168, 176).reverse();
//         const max_entrants = byteArrayToInt(buf);

//         buf = data.slice(176, 184).reverse();
//         const start_timestamp = byteArrayToInt(buf);

//         buf = data.slice(184, 192).reverse();
//         const end_timestamp = byteArrayToInt(buf);

//         buf = data.slice(192, 200).reverse();
//         const whitelisted = byteArrayToInt(buf);

//         const winner = new PublicKey(data.slice(200, 232)).toBase58();

//         let entrants = [];

//         for (let j = 0; j < count; j++) {
//             const entrant = new PublicKey(data.slice(232 + j * 32, 264 + j * 32));
//             entrants.push(entrant.toBase58());
//         }

//         result.push({
//             raffleKey: poolAccounts[i].pubkey.toBase58(),
//             creator: creator.toBase58(),
//             token_program: token_program.toBase58(),
//             token_mint: token_mint.toBase58(),
//             ticket_token_program: ticket_token_program.toBase58(),
//             token_amount,
//             ticket_price,
//             count,
//             no_repeat,
//             max_entrants,
//             start_timestamp,
//             end_timestamp,
//             whitelisted,
//             winner,
//             entrants: entrants
//         });

//     }

//     return result;
// }

// export const getTreasury = async (
// ) => {
//     let cloneWindow: any = window;
//     let provider = new anchor.AnchorProvider(solConnection, cloneWindow['solana'], anchor.AnchorProvider.defaultOptions())
//     const program = new anchor.Program(SaturnIDL as anchor.Idl, PROGRAM_ID, provider);
//     let poolAccounts = await solConnection.getProgramAccounts(
//         program.programId,
//         {
//             filters: [
//                 {
//                     dataSize: 72
//                 },
//             ]
//         }
//     );
//     if (poolAccounts.length !== 0) {
//         for (let i = 0; i < poolAccounts.length; i++) {
//             const data = poolAccounts[i].account.data;
//             // const buf = data.slice(128, 136).reverse();
//             console.log(data);
//         }
//     }
// }

// export const getRaffleKey = async (
//     nft_mint: PublicKey
// ): Promise<PublicKey | null> => {
//     let cloneWindow: any = window;
//     let provider = new anchor.AnchorProvider(solConnection, cloneWindow['solana'], anchor.AnchorProvider.defaultOptions())
//     const program = new anchor.Program(RaffleIDL as anchor.Idl, PROGRAM_ID, provider);
//     let poolAccounts = await solConnection.getProgramAccounts(
//         program.programId,
//         {
//             filters: [
//                 {
//                     dataSize: RAFFLE_SIZE
//                 },
//                 {
//                     memcmp: {
//                         "offset": 40,
//                         "bytes": nft_mint.toBase58()
//                     }
//                 }
//             ]
//         }
//     );
//     if (poolAccounts.length !== 0) {
//         let maxId = 0;
//         let used = 0;
//         for (let i = 0; i < poolAccounts.length; i++) {
//             const data = poolAccounts[i].account.data;
//             const buf = data.slice(128, 136).reverse();
//             if ((new anchor.BN(buf)).toNumber() === 1) {
//                 maxId = i;
//                 used = 1;
//             }

//         }
//         let raffleKey: PublicKey = PublicKey.default;

//         if (used === 1) raffleKey = poolAccounts[maxId].pubkey;

//         console.log(raffleKey.toBase58())
//         return raffleKey;
//     } else {
//         return null;
//     }
// }

// export const getStateByKey = async (
//     raffleKey: PublicKey
// ): Promise<RafflePool | null> => {

//     let cloneWindow: any = window;
//     let provider = new anchor.AnchorProvider(solConnection, cloneWindow['solana'], anchor.AnchorProvider.defaultOptions())
//     const program = new anchor.Program(RaffleIDL as anchor.Idl, PROGRAM_ID, provider);
//     console.log(raffleKey.toBase58());
//     try {
//         let rentalState = await program.account.rafflePool.fetch(raffleKey);
//         return rentalState as unknown as RafflePool;
//     } catch {
//         return null;
//     }
// }
const getAssociatedTokenAccount = async (ownerPubkey: PublicKey, mintPk: PublicKey): Promise<PublicKey> => {
    let associatedTokenAccountPubkey = (await PublicKey.findProgramAddressSync(
        [
            ownerPubkey.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            mintPk.toBuffer(), // mint address
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID
    ))[0];
    return associatedTokenAccountPubkey;
}

export const getATokenAccountsNeedCreate = async (
    connection: anchor.web3.Connection,
    walletAddress: anchor.web3.PublicKey,
    owner: anchor.web3.PublicKey,
    nfts: anchor.web3.PublicKey[],
) => {
    let instructions = [], destinationAccounts = [];
    for (const mint of nfts) {
        const destinationPubkey = await getAssociatedTokenAccount(owner, mint);
        let response = await connection.getAccountInfo(destinationPubkey);
        if (!response) {
            const createATAIx = createAssociatedTokenAccountInstruction(
                destinationPubkey,
                walletAddress,
                owner,
                mint,
            );
            instructions.push(createATAIx);
        }
        destinationAccounts.push(destinationPubkey);
    }
    return {
        instructions,
        destinationAccounts,
    };
}

export const createAssociatedTokenAccountInstruction = (
    associatedTokenAddress: anchor.web3.PublicKey,
    payer: anchor.web3.PublicKey,
    walletAddress: anchor.web3.PublicKey,
    splTokenMintAddress: anchor.web3.PublicKey
) => {
    const keys = [
        { pubkey: payer, isSigner: true, isWritable: true },
        { pubkey: associatedTokenAddress, isSigner: false, isWritable: true },
        { pubkey: walletAddress, isSigner: false, isWritable: false },
        { pubkey: splTokenMintAddress, isSigner: false, isWritable: false },
        {
            pubkey: anchor.web3.SystemProgram.programId,
            isSigner: false,
            isWritable: false,
        },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        {
            pubkey: anchor.web3.SYSVAR_RENT_PUBKEY,
            isSigner: false,
            isWritable: false,
        },
    ];
    return new anchor.web3.TransactionInstruction({
        keys,
        programId: ASSOCIATED_TOKEN_PROGRAM_ID,
        data: Buffer.from([]),
    });
}

main();