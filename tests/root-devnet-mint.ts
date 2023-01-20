import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RootDevnetMint } from "../target/types/root_devnet_mint";

describe("root-devnet-mint", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RootDevnetMint as Program<RootDevnetMint>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
