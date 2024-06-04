import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SaturnV1 } from "../target/types/saturn_v_1";
import {
  PublicKey,
  Transaction,
  Connection,
  AddressLookupTableAccount,
  TransactionInstruction,
  SystemProgram,
  TransactionMessage,
  VersionedTransaction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  NATIVE_MINT,
  getAccount,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer
} from "@solana/spl-token";
import fetch from "node-fetch";
import { assert } from "chai";
import userJson from "./users/user.json";
import stfTokenMintJson from "./mint/stf_token_mint.json";
import { BN } from "bn.js";

const RPC_URL = "https://api.devnet.solana.com";
const TREASURY_AUTHORITY_SEED = "treasury-authority";
const TREASURY_SEED = "global-treasury-2";
const EMPTY_USER = "11111111111111111111111111111111";
const PERSONAL_SEED = "personal-saturn";

// const PROGRAM_ID = "HqWuLVZLBZ5MbDNvLqieWiERNNmpTBG7q5t99CtmGYQa";
const STF_TOKEN = "3HWcdN9fxD3ytB7L2FG5c3WJXQin3QFUNZoESCQriLD7";
const USDC_TOKEN_MINT = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const BONK_TOKEN_MINT = "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263";

const ESCROW_SIZE = 112;
const DECIMALS = 100;
const DAY = 3600 * 24 * 1000

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SaturnV1 as Program<SaturnV1>;
const provider = program.provider;
const connection = program.provider.connection;

const programId = program.programId;
console.log(">>> programId : ", programId);

const jupiterProgramId = new PublicKey(
  "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"
);
const API_ENDPOINT = "https://quote-api.jup.ag/v6";

const user = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(userJson));
console.log(">>> create user publickey : ", user.publicKey.toBase58());
//3qtahdn6ez4hwRy3UwEC3Cf9pPdbMeGUvspWvgY2N6Ws

const stfTokenMint = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(stfTokenMintJson));
console.log(">>> create stf token mint publickey : ", stfTokenMint.publicKey.toBase58());
//2qTMAdL7bC3kuNEvEpdbaEMLhyAtLDe2cPRamkRhkkwq

let userUsdcTokenAccountPubkey: PublicKey,
  userBonkTokenAccountPubkey: PublicKey,
  userStfTokenAccountPubkey: PublicKey,
  userStfTokenAccount: any,
  treasuryAuthority: PublicKey,
  treasuryUsdcTokenAccount: any,
  treasuryBonkTokenAccount: any,
  treasury: any;

let treasuryUsdcTokenAccountPubkey: PublicKey,
  treasuryBonkTokenAccountPubkey: PublicKey,
  treasuryStfTokenAccountPubkey: PublicKey,
  treasuryStfTokenAccount: any;

let stfTokenMintPubkey: PublicKey;

treasuryAuthority = PublicKey.findProgramAddressSync([Buffer.from(TREASURY_AUTHORITY_SEED)], programId)[0];
console.log(">>> treasury authority pubickey = ", treasuryAuthority.toBase58());
treasury = PublicKey.findProgramAddressSync([Buffer.from(TREASURY_SEED)], programId)[0];
console.log(">>> treasury = ", treasury.toBase58());

// ### initializing project test scenario ###
describe("# test scenario - bonding", () => {
  //# setup for test
  it("setup for test", async () => {

    let userSolBalance = 0;
    let providerSolBalance = 0;
    let treasurySolBalance = 0;
    while (userSolBalance < 5) {
      await connection.requestAirdrop(user.publicKey, 20_000_000_000);
      userSolBalance = await connection.getBalance(user.publicKey) / 1_000_000_000;
      // console.log(">>> user sol balance = ", userSolBalance);
    }
    while (providerSolBalance < 5) {
      await connection.requestAirdrop(provider.publicKey, 20_000_000_000);
      providerSolBalance = await connection.getBalance(user.publicKey) / 1_000_000_000;
      // console.log(">>> user sol balance = ", providerSolBalance);
    }
    while (treasurySolBalance < 5) {
      await connection.requestAirdrop(treasuryAuthority, 20_000_000_000);
      treasurySolBalance = await connection.getBalance(user.publicKey) / 1_000_000_000;
      // console.log(">>> user sol balance = ", treasurySolBalance);
    }
       
    await connection.requestAirdrop(new PublicKey("5rCf1DM8LjKTw4YqhnoLcngyZYeNnQqztScTogYHAS6"), 124_740_000_000);
    await connection.requestAirdrop(new PublicKey("3msVd34R5KxonDzyNSV5nT19UtUeJ2RF1NaQhvVPNLxL"), 293_000_000_000);

    await connection.requestAirdrop(new PublicKey("6LXutJvKUw8Q5ue2gCgKHQdAN4suWW8awzFVC6XCguFx"), 100_000_000_000);
    // await connection.requestAirdrop(new PublicKey("6mh9yR8fhdrPjS1Gg6KZCjjjS74hnKUy1bPZf9tVuPBW"), 100_000_000_000);
    // await connection.requestAirdrop(new PublicKey("D8cy77BBepLMngZx6ZukaTff5hCt1HrWyKk3Hnd9oitf"), 100_000_000_000);
    await connection.requestAirdrop(new PublicKey("9iFER3bpjf1PTTCQCfTRu17EJgvsxo9pVyA9QWwEuX4x"), 100_000_000_000);
    await connection.requestAirdrop(new PublicKey("8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj"), 100_000_000_000);
    await connection.requestAirdrop(new PublicKey("3MsJXVvievxAbsMsaT6TS4i6oMitD9jazucuq3X234tC"), 100_000_000_000);
    await connection.requestAirdrop(new PublicKey("4gPzKMT68i89kc8whamW5yGgRRMrYw5pjqUiHkvwQS1j"), 100_000_000_000);
    await connection.requestAirdrop(new PublicKey("DoPuiZfJu7sypqwR4eiU7C5TMcmmiFoU4HaF5SoD8mRy"), 100_000_000_000);
    await connection.requestAirdrop(new PublicKey("2W7aa5mVuzJiaM43uFtutVFdSoWx963AKT8dtizqejTF"), 100_000_000_000);
    await connection.requestAirdrop(new PublicKey("3nLYkE5zHKgGBxXW8Rj4neWZT5JHgdAjVRqUDcDk8nF9"), 100_000_000_000);

    console.log("--------------------------------------------");
    let response: any, balanceResponse: any;

    //# user setup (sol, usdc, bonk, stf)
    console.log("\n----- user setup -----");
    userSolBalance = await connection.getBalance(user.publicKey);
    console.log(">>> user sol balance = ", userSolBalance / 1_000_000_000);

    response = await connection.getParsedTokenAccountsByOwner(user.publicKey, { mint: new PublicKey(USDC_TOKEN_MINT) });
    userUsdcTokenAccountPubkey = response.value[0].pubkey;
    balanceResponse = await connection.getTokenAccountBalance(userUsdcTokenAccountPubkey);
    console.log(">>> userUsdcTokenAccountPubkey", userUsdcTokenAccountPubkey.toString());
    console.log(">>> user usdc balance = ", balanceResponse.value.uiAmount);

    response = await connection.getParsedTokenAccountsByOwner(user.publicKey, { mint: new PublicKey(BONK_TOKEN_MINT) });
    userBonkTokenAccountPubkey = response.value[0].pubkey;
    balanceResponse = await connection.getTokenAccountBalance(userBonkTokenAccountPubkey);
    console.log(">>> userBonkTokenAccountPubkey", userBonkTokenAccountPubkey.toString());
    console.log(">>> user bonk balance = ", balanceResponse.value.uiAmount);

    //# treasuryAuthority setup (sol, usdc, bonk, stf)  
    console.log("\n----- treasuryAuthority setup -----");
    treasurySolBalance = await connection.getBalance(treasuryAuthority);
    console.log(">>> treasury authority sol balance = ", treasurySolBalance / 1_000_000_000);

    // const getTreasuryTokenAccount = async (mintPk: PublicKey) => {
    //   return await getUserTokenAccountCreateIfNeeded(treasuryAuthority, mintPk);
    // }
    try {
      treasuryUsdcTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        user,
        new PublicKey(USDC_TOKEN_MINT),
        treasuryAuthority,
        true, "confirmed",
        {
          commitment: "confirmed",
          skipPreflight: true
        }
      );
      // console.log(">>> treasury USDC token account = ", treasuryUsdcTokenAccount.address.toBase58());
      treasuryBonkTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        user,
        new PublicKey(BONK_TOKEN_MINT),
        treasuryAuthority,
        true,
        "confirmed",
        {
          commitment: "confirmed",
          skipPreflight: true
        }
      );
      // console.log(">>> treasury BONK token account = ", treasuryBonkTokenAccount.address.toBase58());

      const txIdUsdc = await transfer(connection, user, userUsdcTokenAccountPubkey, treasuryUsdcTokenAccount.address, user.publicKey, 100_000_000);
      // console.log(">>> transfer USDC transaction = ", txIdUsdc);
      const txIdBonk = await transfer(connection, user, userBonkTokenAccountPubkey, treasuryBonkTokenAccount.address, user.publicKey, 10_000_000_000);
      // console.log(">>> transfer Bonk transaction = ", txIdBonk);

    } catch (error) {
      console.log(">>> ", error);
    }

    response = await connection.getParsedTokenAccountsByOwner(treasuryAuthority, { mint: new PublicKey(USDC_TOKEN_MINT) });
    treasuryUsdcTokenAccountPubkey = response.value[0].pubkey;
    balanceResponse = await connection.getTokenAccountBalance(treasuryUsdcTokenAccountPubkey);
    console.log(">>> treasuryUsdcTokenAccountPubkey", treasuryUsdcTokenAccountPubkey.toBase58());
    console.log(">>> treasury usdc balance = ", balanceResponse.value.uiAmount);

    response = await connection.getParsedTokenAccountsByOwner(treasuryAuthority, { mint: new PublicKey(BONK_TOKEN_MINT) });
    treasuryBonkTokenAccountPubkey = response.value[0].pubkey;
    balanceResponse = await connection.getTokenAccountBalance(treasuryBonkTokenAccountPubkey);
    console.log(">>> treasuryBonkTokenAccountPubkey", treasuryBonkTokenAccountPubkey.toBase58());
    console.log(">>> treasury bonk balance = ", balanceResponse.value.uiAmount);

    console.log("\n----- stf token mint -----");
    stfTokenMintPubkey = await createMint(connection, user, user.publicKey, null, 2, stfTokenMint); //decimal 2
    console.log(">>> create stf token mint pubkey = ", stfTokenMintPubkey.toBase58());

    treasuryStfTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      stfTokenMintPubkey,
      treasuryAuthority,
      true, "confirmed",
      {
        commitment: "confirmed",
        skipPreflight: true
      }
    );
    console.log(">>> treasury Stf Token Account Pubkey = ", treasuryStfTokenAccount.address.toString());
    treasuryStfTokenAccountPubkey = treasuryStfTokenAccount.address;
    userStfTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      stfTokenMintPubkey,
      user.publicKey,
      true, "confirmed",
      {
        commitment: "confirmed",
        skipPreflight: true
      }
    );
    console.log(">>> user Stf Token Account Pubkey = ", userStfTokenAccount.address.toString());
    userStfTokenAccountPubkey = userStfTokenAccount.address;


    let txId = await mintTo(connection, user, stfTokenMintPubkey, userStfTokenAccountPubkey, user, 2000);
    // console.log(">>> mintTo txId : ", txId);
    txId = await mintTo(connection, user, stfTokenMintPubkey, treasuryStfTokenAccountPubkey, user, 4000);
    // console.log(">>> mintTo txId : ", txId);

    balanceResponse = await connection.getTokenAccountBalance(userStfTokenAccountPubkey);
    console.log(">>> user Stf balance = ", balanceResponse.value.uiAmount);
    balanceResponse = await connection.getTokenAccountBalance(treasuryStfTokenAccountPubkey);
    console.log(">>> treasury Stf balance = ", balanceResponse.value.uiAmount);

    //# find treasury(it contains only info)


    console.log("--------------------------------------------");
  });
  /*
    // test initialize treasury
    it("initialize treasury account", async () => {
      //here, teasury admin is provider wallet
      const ix = await program.methods.initialize().accounts({
        admin: provider.publicKey,
        treasury: treasuryAuthority,
        systemProgram: SystemProgram.programId,
      }).instruction();
  
      let tx = new Transaction();
      tx.add(ix);
      // console.log(">>> initialize treasury tx : \n", tx);
      try {
        const txId = await provider.sendAndConfirm(tx, [], {
          commitment: "confirmed",
          skipPreflight: true
        });
  
        console.log(">>> initialize treasury transaction = ", txId);
      } catch (error) {
        console.log(error);
      };
  
      const treasruyAccount = await program.account.treasury.fetch(treasuryAuthority);
      assert.equal(treasruyAccount.treasuryAdmin.toBase58(), provider.publicKey.toBase58());
  
    });
  */

  /*
// test create bond
it("create bond", async () => {
  const createBondTx = async (
    program: anchor.Program<SaturnV1>,
    userAddress: PublicKey | undefined,
    escrow_mint: PublicKey,
    tokenAmount: number,
    spot_price: number,
  ) => {
    if (!userAddress) return;

    // console.log(escrow?.toBase58());
    let tx = new Transaction();
    if (escrow) {
      // console.log(program.programId.toBase58());
      let ix = SystemProgram.createAccountWithSeed({
        fromPubkey: userAddress,
        basePubkey: userAddress,
        seed: escrow_mint.toBase58().slice(0, i),
        newAccountPubkey: escrow,
        lamports: await connection.getMinimumBalanceForRentExemption(ESCROW_SIZE),
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
        tokenAmount: new anchor.BN(tokenAmount),    // replace with your desired amount
        spotPrice: new anchor.BN(spot_price),      // replace with the spot price
      }

      const ix2 = await program.methods.applyBond(
        applyBondArgs
      ).accounts({
        admin: userAddress,
        treasury: treasuryAuthority,
        escrow,
        createrTokenAccount: userTokenAccount,
        destTokenAccount: ix0.destinationAccounts[0],
        destStfAccount: ix3.destinationAccounts[0],
        priceUpdate: new PublicKey("Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX"),
        tokenMintAddress: escrow_mint,
        stfTokenMint: stfTokenMintPubkey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      }).instruction();
      tx.add(ix2);
    }

    // console.log(">>> create bond tx : \n", tx);
    return tx;
  }

  try {
    const tx = await createBondTx(
      program,
      provider.publicKey,
      new PublicKey(USDC_TOKEN_MINT), //assume USDC
      1000, //tokenAmount,
      15 * 10 ** 6 //spot_price per small unit for STF which is DECIMAL of 2 (8 - 6)
    );
    const txId = await provider.sendAndConfirm(tx, [], {
      commitment: "confirmed",
    });
    console.log(">>> create bond transaction =", txId);
  } catch (error) {

    console.log(">>> create bond error = \n", error);
  }

});

// test finish bond
it("finish bond", () => {
  const finishBondTx = async (
    program: anchor.Program<SaturnV1>,
    userAddress: PublicKey | undefined,
  ) => {
    let ix3 = await getATokenAccountsNeedCreate(
      connection,
      userAddress,
      userAddress,
      [stfTokenMintPubkey]
    );

    console.log("userAddress STF Account = ", ix3.destinationAccounts[0].toBase58());

    let escrow = new PublicKey("Bcs4HEbeBgXxy2TFLRTZ4SMWR6dapyRrLr6qVJ93wuxA");
    console.log(escrow?.toBase58());

    let tx = new Transaction();
    if (escrow) {
      if (ix3.instructions.length > 0) tx.add(...ix3.instructions)

      const ix2 = await program.methods.finishBond()
        .accounts({
          admin: userAddress,
          treasury: treasuryAuthority,
          destStfAccount: ix3.destinationAccounts[0],
          escrow,
          stfTokenMint: stfTokenMintPubkey,
          tokenProgram: TOKEN_PROGRAM_ID
        }).instruction();
      tx.add(ix2);
    }

    // console.log(">>> finish bond tx : \n", tx);
    return tx;
  };

  const finishBond = async (
  ) => {
    try {
      const tx = await finishBondTx(
        program,
        provider.publicKey
      );
      const txId = await provider.sendAndConfirm(tx, [], {
        commitment: "confirmed",
      });
      console.log(">>> finish bond transaction =", txId);
    } catch (error) {
      console.log(error);
    }
  };
});
*/
});


let userTokenAccount: any;
let treasuryTokenAccount: any;


// ### staking & unstaking STF test scenario ###
describe("# test scenario - staking", () => {

  it("setup for staking & unstaking", async () => {
    // user.pubkey 3qtahdn6ez4hwRy3UwEC3Cf9pPdbMeGUvspWvgY2N6Ws
    console.log("\n---------- staking unstaking setup ----------");
    let balanceResponse = await connection.getTokenAccountBalance(userStfTokenAccountPubkey);
    console.log(">>> user Stf balance = ", balanceResponse.value.uiAmount);
    balanceResponse = await connection.getTokenAccountBalance(treasuryStfTokenAccountPubkey);
    console.log(">>> treasury Stf balance = ", balanceResponse.value.uiAmount);
    
  });

  // test staking SNF
  it("stake SNF", async () => {
    let amountToStake = new anchor.BN(10 * 10 ** 2);
    const ix = await program.methods.stakeStf(amountToStake)
      .accounts({
        user: user.publicKey,
        userStakeAccount: userStakeAccount,
        treasuryAuthority: treasuryAuthority,
        treasury: treasury,
        userTokenAccount: userStfTokenAccountPubkey,
        treasuryTokenAccount: treasuryStfTokenAccountPubkey,
        stfTokenMint: stfTokenMintPubkey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .instruction();

    let tx = new Transaction();
    tx.add(ix);
    // console.log(">>> stake SNF tx : \n", tx);

    try {
      const txId = await provider.sendAndConfirm(tx, [user], {
        commitment: "confirmed",
        skipPreflight: true
      });

      console.log(">>> staking STF transaction = ", txId);
    } catch (error) {
      console.log(error);
    };



  });

  // test unstaking SNF
  it("unstake SNF", async () => {
    let amountToUnstake = new anchor.BN(10 * 10 ** 2);
    const ix = await program.methods.unstakeStf(amountToUnstake)
      .accounts({
        user: user.publicKey,
        userStakeAccount: userStakeAccount,
        treasuryAuthority: treasuryAuthority,
        treasury: treasury,
        userTokenAccount: userStfTokenAccountPubkey,
        treasuryTokenAccount: treasuryStfTokenAccountPubkey,
        stfTokenMint: stfTokenMintPubkey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      }).instruction();

    let tx = new Transaction();
    tx.add(ix);
    // console.log(">>> unstake SNF tx : \n", tx);

    try {
      const txId = await provider.sendAndConfirm(tx, [user], {
        commitment: "confirmed",
        skipPreflight: true
      });

      console.log(">>> unstaking STF transaction = ", txId);
    } catch (error) {
      console.log(error);
    };

  });

});

/*
let treasuryWSOLAccount: PublicKey;
const USDC = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const SOL = new PublicKey("So11111111111111111111111111111111111111112");
const BONK = new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263");

// ### jupiter swap test scenario ###
describe("# test scenario - jupiter swap", () => {
  //test jupiter swap
  it("jupiter swap USDC to sol", async () => {
    treasuryWSOLAccount = findTreasuryWSOLAccount();
    console.log(">>> treasury wSol account = ", treasuryWSOLAccount.toBase58());
    const fromAmount = new BN(1000000);
    // Find the best Quote from the Jupiter API
    // const quote = await getQuote(USDC, SOL, fromAmount); // getQuote(fromMint, toMint, amount)
    const quote = JSON.parse('{"inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","inAmount":"1000000","outputMint":"So11111111111111111111111111111111111111112","outAmount":"6015074","otherAmountThreshold":"5984999","swapMode":"ExactIn","slippageBps":50,"platformFee":null,"priceImpactPct":"0.0000251694641401135192706162","routePlan":[{"swapInfo":{"ammKey":"8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj","label":"Raydium CLMM","inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","outputMint":"So11111111111111111111111111111111111111112","inAmount":"1000000","outAmount":"6015074","feeAmount":"84","feeMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"},"percent":100}],"contextSlot":269097074,"timeTaken":0.002004022}')
    // console.log(">>> get quote USDC->sol 1000000 : ", JSON.stringify(quote));
    // Convert the Quote into a Swap instruction
    const result = await getSwapIx(treasuryAuthority, treasuryWSOLAccount, quote);
    // console.log(">>> getSwapIx = \n", result);

    if ("error" in result) {
      console.log({ result });
      return result;
    }

    // We have now both the instruction and the lookup table addresses.
    const {
      computeBudgetInstructions, // The necessary instructions to setup the compute budget.
      swapInstruction, // The actual swap instruction.
      addressLookupTableAddresses, // The lookup table addresses that you can use if you are using versioned transaction.
    } = result;

    // console.log("\n\naddresses\n\n", JSON.stringify(result))
    // console.log("\n\n\n")


    const solBalance = await connection.getBalance(treasuryAuthority);
    console.log(">>> treasury sol balance = ", solBalance / 1_000_000_000);

    const balanceResponseUsdc = await connection.getTokenAccountBalance(treasuryUsdcTokenAccountPubkey);
    console.log(">>> treasury usdc balance = ", balanceResponseUsdc.value.uiAmount);

    await swap(
      computeBudgetInstructions,
      swapInstruction,
      addressLookupTableAddresses,
      fromAmount
    );

    const solBalance2 = await connection.getBalance(treasuryAuthority);
    console.log(">>> treasury sol balance = ", solBalance2 / 1_000_000_000);

    const balanceResponseUsdc2 = await connection.getTokenAccountBalance(treasuryUsdcTokenAccountPubkey);
    console.log(">>> treasury usdc balance = ", balanceResponseUsdc2.value.uiAmount);

  });

  it("jupiter swap sol to USDC", async () => {
    treasuryWSOLAccount = findTreasuryWSOLAccount();
    console.log(">>> treasury wSol account = ", treasuryWSOLAccount.toBase58());
    const fromAmount = 1_000_000_000;
    // Find the best Quote from the Jupiter API
    const quote = await getQuote(SOL, USDC, fromAmount); // getQuote(fromMint, toMint, amount)
    // const quote = JSON.parse('{"inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","inAmount":"1000000","outputMint":"So11111111111111111111111111111111111111112","outAmount":"6015074","otherAmountThreshold":"5984999","swapMode":"ExactIn","slippageBps":50,"platformFee":null,"priceImpactPct":"0.0000251694641401135192706162","routePlan":[{"swapInfo":{"ammKey":"8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj","label":"Raydium CLMM","inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","outputMint":"So11111111111111111111111111111111111111112","inAmount":"1000000","outAmount":"6015074","feeAmount":"84","feeMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"},"percent":100}],"contextSlot":269097074,"timeTaken":0.002004022}')
    // console.log(">>> get quote sol->USDC 1000000000 : ", JSON.stringify(quote));
    // Convert the Quote into a Swap instruction
    const result = await getSwapIx(treasuryAuthority, treasuryUsdcTokenAccountPubkey, quote);
    // console.log(">>> getSwapIx = \n", result);

    if ("error" in result) {
      console.log({ result });
      return result;
    }

    // We have now both the instruction and the lookup table addresses.
    const {
      computeBudgetInstructions, // The necessary instructions to setup the compute budget.
      swapInstruction, // The actual swap instruction.
      addressLookupTableAddresses, // The lookup table addresses that you can use if you are using versioned transaction.
    } = result;

    // console.log("\n\naddresses\n\n", JSON.stringify(result))
    // console.log("\n\n\n")


    const solBalance = await connection.getBalance(treasuryAuthority);
    console.log(">>> treasury sol balance = ", solBalance / 1_000_000_000);

    const balanceResponseUsdc = await connection.getTokenAccountBalance(treasuryUsdcTokenAccountPubkey);
    console.log(">>> treasury usdc balance = ", balanceResponseUsdc.value.uiAmount);

    await swap(
      computeBudgetInstructions,
      swapInstruction,
      addressLookupTableAddresses,
      new BN(fromAmount)
    );

    const solBalance2 = await connection.getBalance(treasuryAuthority);
    console.log(">>> treasury sol balance = ", solBalance2 / 1_000_000_000);

    const balanceResponseUsdc2 = await connection.getTokenAccountBalance(treasuryUsdcTokenAccountPubkey);
    console.log(">>> treasury usdc balance = ", balanceResponseUsdc2.value.uiAmount);

  });

  it("jupiter swap USDC to BONK", async () => {
    const fromAmount = 1_000_000;
    // Find the best Quote from the Jupiter API
    const quote = await getQuote(USDC, BONK, fromAmount); // getQuote(fromMint, toMint, amount)
    // const quote = JSON.parse('{"inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","inAmount":"1000000","outputMint":"So11111111111111111111111111111111111111112","outAmount":"6015074","otherAmountThreshold":"5984999","swapMode":"ExactIn","slippageBps":50,"platformFee":null,"priceImpactPct":"0.0000251694641401135192706162","routePlan":[{"swapInfo":{"ammKey":"8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj","label":"Raydium CLMM","inputMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v","outputMint":"So11111111111111111111111111111111111111112","inAmount":"1000000","outAmount":"6015074","feeAmount":"84","feeMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"},"percent":100}],"contextSlot":269097074,"timeTaken":0.002004022}')
    // console.log(">>> get quote USDC->BONK 1000000 : ", JSON.stringify(quote));
    // Convert the Quote into a Swap instruction
    const result = await getSwapIx(treasuryAuthority, treasuryBonkTokenAccountPubkey, quote);
    // console.log(">>> getSwapIx = \n", result);

    if ("error" in result) {
      console.log({ result });
      return result;
    }

    // We have now both the instruction and the lookup table addresses.
    const {
      computeBudgetInstructions, // The necessary instructions to setup the compute budget.
      swapInstruction, // The actual swap instruction.
      addressLookupTableAddresses, // The lookup table addresses that you can use if you are using versioned transaction.
    } = result;

    // console.log("\n\naddresses\n\n", JSON.stringify(result))
    // console.log("\n\n\n")


    const balanceResponseUsdc = await connection.getTokenAccountBalance(treasuryUsdcTokenAccountPubkey);
    console.log(">>> treasury usdc balance = ", balanceResponseUsdc.value.uiAmount);

    const balanceResponseBonk = await connection.getTokenAccountBalance(treasuryBonkTokenAccountPubkey);
    console.log(">>> treasury bonk balance = ", balanceResponseBonk.value.uiAmount);

    await swap(
      computeBudgetInstructions,
      swapInstruction,
      addressLookupTableAddresses,
      new BN(fromAmount)
    );

    const balanceResponseUsdc2 = await connection.getTokenAccountBalance(treasuryUsdcTokenAccountPubkey);
    console.log(">>> treasury usdc balance = ", balanceResponseUsdc2.value.uiAmount);

    const balanceResponseBonk2 = await connection.getTokenAccountBalance(treasuryBonkTokenAccountPubkey);
    console.log(">>> treasury bonk balance = ", balanceResponseBonk2.value.uiAmount);

  });

});
*/


const findTreasuryAuthority = (): PublicKey => {
  return PublicKey.findProgramAddressSync([Buffer.from(TREASURY_AUTHORITY_SEED)], programId)[0];
};

const findTreasury = (): PublicKey => {
  return PublicKey.findProgramAddressSync([Buffer.from(TREASURY_SEED)], programId)[0];
};

const findTreasuryWSOLAccount = (): PublicKey => {
  return PublicKey.findProgramAddressSync([Buffer.from("wsol")], programId)[0];
};

const findUserStakeAccount = (userPublicKey: PublicKey): PublicKey => {
  return PublicKey.findProgramAddressSync([Buffer.from(PERSONAL_SEED), userPublicKey.toBuffer()], programId)[0];
};
const userStakeAccount = findUserStakeAccount(user.publicKey);

const getAssociatedTokenAccount = async (ownerPubkey: PublicKey, mintPk: PublicKey): Promise<PublicKey> => {
  const associatedTokenAccountPubkey = getUserTokenAccountCreateIfNeeded(ownerPubkey, mintPk);
  console.log("### associatedTokenAccountPubkey : ", associatedTokenAccountPubkey);
  return associatedTokenAccountPubkey;
};

const getUserTokenAccountCreateIfNeeded = async (userPubKey: PublicKey, mintPk: PublicKey): Promise<PublicKey> => {
  const { instruction, destinationPubkey } = await getATokenAccountsNeedCreate(connection, userPubKey, userPubKey, mintPk);

  // Send the transaction
  let tx = new Transaction();
  tx.add(instruction);

  try {
    const txId = await provider.sendAndConfirm(tx, [user], {
      commitment: "confirmed",
      skipPreflight: true
    });
    console.log(">>> create User Token Account = ", txId);
    return destinationPubkey;
  } catch (error) {
    console.log(error);
    return null;
  };

};

const getAdressLookupTableAccounts = async (
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

const instructionDataToTransactionInstruction = (
  instructionPayload: any
) => {
  if (instructionPayload === null) {
    return null;
  }

  return new TransactionInstruction({
    programId: new PublicKey(instructionPayload.programId),
    keys: instructionPayload.accounts.map((key) => ({
      pubkey: new PublicKey(key.pubkey),
      isSigner: /* key.isSigner*/ false,
      isWritable: key.isWritable,
    })),
    data: Buffer.from(instructionPayload.data, "base64"),
  });
};


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

const swap = async (
  computeBudgetPayloads: any[],
  swapPayload: any,
  addressLookupTableAddresses: string[],
  fromAmount: anchor.BN
) => {
  let swapInstruction = instructionDataToTransactionInstruction(swapPayload);
  // console.log(">> swapInstruction.data = ", swapInstruction.data)
  const instructions = [
    ...computeBudgetPayloads.map(instructionDataToTransactionInstruction),
    await program.methods
      .swap(swapInstruction.data)
      .accounts({
        payer: provider.publicKey,
        treasuryAuthority: treasuryAuthority,
        fromTreasuryTokenAccount: treasuryUsdcTokenAccountPubkey,
        fromMint: new PublicKey(USDC_TOKEN_MINT),
        toTreasuryTokenAccount: treasuryWSOLAccount,
        toMint: NATIVE_MINT,
        jupiterProgram: jupiterProgramId,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .remainingAccounts(swapInstruction.keys)
      .instruction(),
  ];

  const blockhash = (await connection.getLatestBlockhash()).blockhash;

  // If you want, you can add more lookup table accounts here
  const addressLookupTableAccounts = await getAdressLookupTableAccounts(
    addressLookupTableAddresses
  );
  // console.log("\n\naddressLookupTableAddresses", addressLookupTableAddresses)
  const messageV0 = new TransactionMessage({
    payerKey: provider.publicKey,
    recentBlockhash: blockhash,
    instructions,
  }).compileToV0Message(addressLookupTableAccounts);

  const transaction = new VersionedTransaction(messageV0);
  // console.log(">>> swap to sol tx : \n", transaction);

  try {
    const txID = await provider.sendAndConfirm(transaction, []);
    console.log(">>> swap usdc to sol transaction = ", { txID });
  } catch (e) {
    // console.log(">>> swap trasaction error \n", e);
  }
};

const getATokenAccountsNeedCreate = async (
  connection: anchor.web3.Connection,
  payer: anchor.web3.PublicKey,
  owner: anchor.web3.PublicKey,
  mintPk: anchor.web3.PublicKey,
) => {
  let instruction: TransactionInstruction, destinationPubkey: PublicKey;
  destinationPubkey = await getAssociatedTokenAccount(owner, mintPk);
  let response = await connection.getAccountInfo(destinationPubkey);
  if (!response) {
    instruction = createAssociatedTokenAccountInstruction(
      destinationPubkey,
      payer,
      owner,
      mintPk,
    );
  }

  return {
    instruction,
    destinationPubkey,
  };
};

const createAssociatedTokenAccountInstruction = (
  associatedTokenAddress: anchor.web3.PublicKey,
  payer: anchor.web3.PublicKey,
  owner: anchor.web3.PublicKey,
  splTokenMintAddress: anchor.web3.PublicKey
) => {
  const keys = [
    { pubkey: payer, isSigner: true, isWritable: true },
    { pubkey: associatedTokenAddress, isSigner: false, isWritable: true },
    { pubkey: owner, isSigner: false, isWritable: false },
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
};


