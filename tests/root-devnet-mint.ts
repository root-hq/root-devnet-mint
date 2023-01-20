import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ASSOCIATED_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";
import { RootDevnetMint } from "../target/types/root_devnet_mint";
import { createKeypair, GLOBAL_MINT_AUTHORITY_SEED, provider } from "./common";
import {
  getAssociatedTokenAddress,
} from "@solana/spl-token-latest";

describe("root-devnet-mint", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RootDevnetMint as Program<RootDevnetMint>;

  it("Initialize mint!", async () => {
    // Add your test here.

    const signer = await createKeypair(provider);
    
    const token_mint = anchor.web3.Keypair.generate();

    const [globalMintAuthorityAddr,] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        GLOBAL_MINT_AUTHORITY_SEED
      ],
      program.programId
    );

    const tx = await program
      .methods
      .initializeMint()
      .accounts({
        signer: signer.publicKey,
        globalMintAuthority: globalMintAuthorityAddr,
        tokenMint: token_mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([signer, token_mint])
      .rpc();

      
    console.log("Your transaction signature", tx);
  });

  it("Initialize mint and mint some tokens!", async () => {
    // Add your test here.

    const signer = await createKeypair(provider);
    
    const token_mint = anchor.web3.Keypair.generate();

    const [globalMintAuthorityAddr,] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        GLOBAL_MINT_AUTHORITY_SEED
      ],
      program.programId
    );

    const tx = await program
      .methods
      .initializeMint()
      .accounts({
        signer: signer.publicKey,
        globalMintAuthority: globalMintAuthorityAddr,
        tokenMint: token_mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([signer, token_mint])
      .rpc();

    const signerTokenAta = await getAssociatedTokenAddress(
      token_mint.publicKey,
      signer.publicKey,
      false
    );

    const txMint = await program
      .methods
      .mintTokens(new anchor.BN(100))
      .accounts({
        signer: signer.publicKey,
        globalMintAuthority: globalMintAuthorityAddr,
        tokenMint: token_mint.publicKey,
        signerTokenAta: signerTokenAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      })
      .signers([signer])
      .rpc();

      
    console.log("Initialize mint: ", tx);
    console.log("Mint new tokens: ", txMint);
  });
});
