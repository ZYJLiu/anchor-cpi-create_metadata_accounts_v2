import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import { Metadata } from "../target/types/metadata";
import { findMetadataPda } from "@metaplex-foundation/js";

import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

import {
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
} from "@solana/spl-token";

describe("metadata", () => {
  // Configure the client to use the local cluster.

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Metadata as Program<Metadata>;
  const connection = anchor.getProvider().connection;
  const wallet = anchor.workspace.Metadata.provider.wallet;

  it("Is initialized!", async () => {
    const [mint, bump] = await PublicKey.findProgramAddress(
      [Buffer.from("MINT"), wallet.publicKey.toBuffer()],
      program.programId
    );

    const metadataPDA = await findMetadataPda(mint);

    const tx = await program.methods
      .createMint(
        "https://arweave.net/OwXDf7SM6nCVY2cvQ4svNjtV7WBTz3plbI4obN9JNkk",
        "name",
        "SYMBOL"
      )
      .accounts({
        mint: mint,
        metadata: metadataPDA,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        user: wallet.publicKey,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
