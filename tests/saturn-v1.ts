import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SaturnV1 } from "../target/types/saturn_v_1";
import {
  PublicKey,
  Transaction,
  AddressLookupTableAccount,
  TransactionInstruction,
  SystemProgram,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  NATIVE_MINT,
} from "@solana/spl-token";
import fetch from "node-fetch";

const RPC_URL = "https://api.devnet.solana.com";

const TREASURY_SEED = "global-treasury-2";
const EMPTY_USER = "11111111111111111111111111111111";

const PROGRAM_ID = "HqWuLVZLBZ5MbDNvLqieWiERNNmpTBG7q5t99CtmGYQa";
const STF_TOKEN = "3HWcdN9fxD3ytB7L2FG5c3WJXQin3QFUNZoESCQriLD7";
const USDC_TOKEN = "9cmYPgxT1wGP6ySgSDHCmTrLYzeDp1iVssy4grjdjDyQ";

const ESCROW_SIZE = 112;
const DECIMALS = 100;
const DAY = 3600 * 24 * 1000

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SaturnV1 as Program<SaturnV1>;
const provider = program.provider;
const connection = program.provider.connection;

const programId = program.programId;
const jupiterProgramId = new PublicKey(
  "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"
);

// ### initializing project test scenario ###
describe("# test scenario - initialize project", () => {
  //test initialize project
  it("initialize the project", async () => {
    const ix = await program.methods.initialize().accounts({
      admin: provider.publicKey,
      treasury: treasuryAuthority,
      systemProgram: SystemProgram.programId,
    }).instruction();

    let tx = new Transaction();
    tx.add(ix);
    // console.log(">>> initialize project tx : \n", tx);
    try {
      const txId = await provider.sendAndConfirm(tx, [], {
        commitment: "confirmed",
      });

      console.log(">>> initialize project transaction = ", txId);
    } catch (error) {
      console.log(error);
    };

  });
});

// ### bond test scenario ###
describe("# test scenario - bond", () => {
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

      // console.log("HERE", userAddress);
      // console.log(treasuryAuthority.toBase58());

      //Source Token Account 
      let ownerNftAccount = await getAssociatedTokenAccount(userAddress, escrow_mint);
      // console.log("Src USDC Account = ", ownerNftAccount.toBase58());
      let ix0 = await getATokenAccountsNeedCreate(
        connection,
        userAddress,
        treasuryAuthority,
        [escrow_mint]
      );
      // console.log("Dest USDC Account = ", ix0.destinationAccounts[0].toBase58());

      //Create STF Token Account on the user and on the treasury

      let ix1 = await getATokenAccountsNeedCreate(
        connection,
        userAddress,
        userAddress,
        [new PublicKey(STF_TOKEN)]
      );

      // console.log("Creator STF Account = ", ix1.destinationAccounts[0].toBase58());

      let ix3 = await getATokenAccountsNeedCreate(
        connection,
        userAddress,
        treasuryAuthority,
        [new PublicKey(STF_TOKEN)]
      );

      // console.log("Treasury STF Account = ", ix3.destinationAccounts[0].toBase58());

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
          createrTokenAccount: ownerNftAccount,
          destTokenAccount: ix0.destinationAccounts[0],
          destStfAccount: ix3.destinationAccounts[0],
          priceUpdate: new PublicKey("Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX"),
          tokenMintAddress: escrow_mint,
          stfTokenMint: new PublicKey(STF_TOKEN),
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
        new PublicKey(USDC_TOKEN), //assume USDC
        1000, //tokenAmount,
        15 * 10 ** 6 //spot_price per small unit for STF which is DECIMAL of 2 (8 - 6)
      );
      const txId = await provider.sendAndConfirm(tx, [], {
        commitment: "confirmed",
      });
      console.log(">>> create bond transaction =", txId);
    } catch (error) {

      console.log(error);
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
        [new PublicKey(STF_TOKEN)]
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
            stfTokenMint: new PublicKey(STF_TOKEN),
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
  })
})


// ### jupiter swap test scenario ###
describe("# test scenario - jupiter swap to sol", () => {
  //test jupiter swap
  it("jupiter swap to sol", async () => {
    const USDC = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    const SOL = new PublicKey("So11111111111111111111111111111111111111112");

    // Find the best Quote from the Jupiter API
    const quote = await getQuote(USDC, SOL, 1000000);
    console.log({ quote });

    // Convert the Quote into a Swap instruction
    const result = await getSwapIx(treasuryAuthority, treasuryWSOLAccount, quote);

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

    await swapToSol(
      computeBudgetInstructions,
      swapInstruction,
      addressLookupTableAddresses
    );
  })

});

// ### staking & unstaking STF test scenario ###
describe("# test scenario - skaking & unstaking SNF", () => {

  let userAccountToken: anchor.web3.PublicKey, treasuryTokenAccount: anchor.web3.PublicKey;
  it("setup", async () => {
    userAccountToken = await getAssociatedTokenAccount(provider.publicKey, new PublicKey(STF_TOKEN));
    treasuryTokenAccount = await getAssociatedTokenAccount(treasuryAuthority, new PublicKey(STF_TOKEN));
  });

  // test staking SNF
  it("stake SNF", async () => {
    let amountToStake = new anchor.BN(10 * 10 ** 2);
    const ix = await program.methods.stakeStf(amountToStake)
      .accounts({
        user: provider.publicKey,
        userProgramAccount: program.programId,
        treasury: treasuryAuthority,
        userAccountToken: userAccountToken,
        treasuryTokenAccount: treasuryTokenAccount,
        stfTokenMint: new PublicKey(STF_TOKEN),
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      }).instruction();

    let tx = new Transaction();
    tx.add(ix);
    // console.log(">>> stake SNF tx : \n", tx);

    try {
      const txId = await provider.sendAndConfirm(tx, [], {
        commitment: "confirmed"
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
        user: provider.publicKey,
        userProgramAccount: program.programId,
        treasury: treasuryAuthority,
        userAccountToken: userAccountToken,
        treasuryTokenAccount: treasuryTokenAccount,
        stfTokenMint: new PublicKey(STF_TOKEN),
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      }).instruction();

    let tx = new Transaction();
    tx.add(ix);
    // console.log(">>> unstake SNF tx : \n", tx);

    try {
      const txId = await provider.sendAndConfirm(tx, [], {
        commitment: "confirmed"
      });

      console.log(">>> unstaking STF transaction = ", txId);
    } catch (error) {
      console.log(error);
    };

  });

});





const API_ENDPOINT = "https://quote-api.jup.ag/v6";

const findTreasuryAuthority = (): PublicKey => {
  return PublicKey.findProgramAddressSync([Buffer.from(TREASURY_SEED)], programId)[0];
};
const treasuryAuthority = findTreasuryAuthority();

const findTreasuryWSOLAccount = (): PublicKey => {
  return PublicKey.findProgramAddressSync([Buffer.from("wsol")], programId)[0];
};
const treasuryWSOLAccount = findTreasuryWSOLAccount();

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
      isSigner: key.isSigner,
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

const swapToSol = async (
  computeBudgetPayloads: any[],
  swapPayload: any,
  addressLookupTableAddresses: string[]
) => {
  let swapInstruction = instructionDataToTransactionInstruction(swapPayload);

  const instructions = [
    ...computeBudgetPayloads.map(instructionDataToTransactionInstruction),
    await program.methods
      .swapToSol(swapInstruction.data)
      .accounts({
        treasuryAuthority: treasuryAuthority,
        treasuryWsolAccount: treasuryWSOLAccount,
        solMint: NATIVE_MINT,
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
  const messageV0 = new TransactionMessage({
    payerKey: treasuryAuthority,
    recentBlockhash: blockhash,
    instructions,
  }).compileToV0Message(addressLookupTableAccounts);

  const transaction = new VersionedTransaction(messageV0);
  // console.log(">>> swap to sol tx : \n", transaction);

  try {
    await provider.simulate(transaction, []);

    const txID = await provider.sendAndConfirm(transaction, []);
    console.log({ txID });
  } catch (e) {
    console.log({ simulationResponse: e.simulationResponse });
  }
};

const getATokenAccountsNeedCreate = async (
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
};

const createAssociatedTokenAccountInstruction = (
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
};


