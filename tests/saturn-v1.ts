import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import {
  PublicKey,
  Keypair,
  Connection,
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
import { IDL, SaturnV1 } from "../target/types/saturn_v_1";

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SaturnV1 as Program<SaturnV1>;

const programId = program.programId;
const jupiterProgramId = new PublicKey(
  "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"
);
const connection = program.provider.connection;
const provider = program.provider;

describe("saturn-v1", () => {

  it("jupiter swap to sol!", async () => {
    const USDC = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    const SOL = new PublicKey("So11111111111111111111111111111111111111112");

    // Find the best Quote from the Jupiter API
    const quote = await getQuote(USDC, SOL, 1000000);
    console.log({ quote });

    // Convert the Quote into a Swap instruction
    const result = await getSwapIx(program.provider.publicKey, programWSOLAccount, quote);

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

    
  const findProgramAuthority = (): PublicKey => {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      programId
    )[0];
  };
  const programAuthority = findProgramAuthority();

  const findProgramWSOLAccount = (): PublicKey => {
    return PublicKey.findProgramAddressSync([Buffer.from("wsol")], programId)[0];
  };
  const programWSOLAccount = findProgramWSOLAccount();

  const findAssociatedTokenAddress = ({
    walletAddress,
    tokenMintAddress,
  }: {
    walletAddress: PublicKey;
    tokenMintAddress: PublicKey;
  }): PublicKey => {
    return PublicKey.findProgramAddressSync(
      [
        walletAddress.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        tokenMintAddress.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
    )[0];
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
          programAuthority: programAuthority,
          programWsolAccount: programWSOLAccount,
          userAccount: program.provider.publicKey,
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
      payerKey: program.provider.publicKey,
      recentBlockhash: blockhash,
      instructions,
    }).compileToV0Message(addressLookupTableAccounts);
    const transaction = new VersionedTransaction(messageV0);

    try {
      await provider.simulate(transaction, []);

      const txID = await provider.sendAndConfirm(transaction, []);
      console.log({ txID });
    } catch (e) {
      console.log({ simulationResponse: e.simulationResponse });
    }
  };