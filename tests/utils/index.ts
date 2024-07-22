import { BN, Wallet } from "@coral-xyz/anchor";
import { bs58 } from '"@coral-xyz/anchor/dist/cjs/utils/bytes';
import { TOKEN_PROGRAM_ID, Token, getAccount, mintToChecked } from '@solana/spl-token';
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';

export const airDropSol = async (connection: Connection, publicKey: PublicKey, amount = 1) => {
  try {
    const airdropSignature = await connection.requestAirdrop(publicKey, amount * LAMPORTS_PER_SOL);
    const latestBlockHash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: airdropSignature,
      },
      connection.commitment,
    );
  } catch (error) {
    console.error(error);
    throw error;
  }
};

export const airDropSolIfBalanceNotEnough = async (connection: Connection, publicKey: PublicKey, balance = 1) => {
  const walletBalance = await connection.getBalance(publicKey);
  if (walletBalance < balance * LAMPORTS_PER_SOL) {
    await airDropSol(connection, publicKey);
  }
};

export const getOrCreateATA = async (connection: Connection, mint: PublicKey, owner: PublicKey, payer: Keypair) => {
  const token = new Token(connection, mint, TOKEN_PROGRAM_ID, payer);
  const ata = await token.getOrCreateAssociatedAccountInfo(owner);

  return ata.address;
};

export const toTokenAmount = (uiAmount: number, decimals: number): BN => {
  return new BN(uiAmount * 10 ** decimals);
}

export const toUiAmount = (token_amount: number, decimals: number): number => {
  return token_amount / 10 ** decimals;
}

// return in lamports
export const getSolBalance = async (connection: Connection, pubkey: PublicKey) => {
  return connection
    .getBalance(pubkey)
    .then((balance) => balance)
    .catch(() => 0);
};

export const getBalance = async (connection: Connection, pubkey: PublicKey) => {
  return getAccount(connection, pubkey)
    .then((account) => Number(account.amount))
    .catch(() => 0);
};

export const mintTokens = async (
  connection: Connection,
  payer: Keypair,
  uiAmount: number,
  decimals: number,
  mint: PublicKey,
  destiantionWallet: PublicKey
) => {
  await mintToChecked(
      connection,
      payer,
      mint,
      destiantionWallet,
      payer.publicKey,
      toTokenAmount(uiAmount, decimals).toNumber(),
      decimals
  );
};

export const mockWallet = new Wallet(Keypair.generate());

// export const MAINNET = {
//   connection: new Connection(process.env.MAINNET_RPC_ENDPOINT as string),
//   cluster: 'mainnet-beta',
// };

// export const DEVNET = {
//   connection: new Connection('https://api.devnet.solana.com/', {
//     commitment: 'confirmed',
//   }),
//   cluster: 'devnet',
// };


export const API_ENDPOINT = "https://quote-api.jup.ag/v6";

const getQuote = async (
  fromMint: PublicKey,
  toMint: PublicKey,
  amount: number
) => {
  return fetch(
    `${API_ENDPOINT}/quote?outputMint=${toMint.toBase58()}&inputMint=${fromMint.toBase58()}&amount=${amount}&slippage=0.5&onlyDirectRoutes=true`
  ).then((response) => response.json());
};

export const getSwapIx = async (
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

export const swapToSol = async (
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
        userAccount: wallet.publicKey,
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
    payerKey: wallet.publicKey,
    recentBlockhash: blockhash,
    instructions,
  }).compileToV0Message(addressLookupTableAccounts);
  const transaction = new VersionedTransaction(messageV0);

  try {
    await provider.simulate(transaction, [wallet.payer]);

    const txID = await provider.sendAndConfirm(transaction, [wallet.payer]);
    console.log({ txID });
  } catch (e) {
    console.log({ simulationResponse: e.simulationResponse });
  }
};

export const findAssociatedTokenAddress = ({
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

export const getAdressLookupTableAccounts = async (
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

export const instructionDataToTransactionInstruction = (
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