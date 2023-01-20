import * as anchor from "@project-serum/anchor";

export const createKeypair = async (provider: anchor.Provider) => {
    const keypair = new anchor.web3.Keypair();
    const txn = await provider.connection.requestAirdrop(
      keypair.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(txn);
    return keypair;
};

export const provider = anchor.getProvider() as anchor.AnchorProvider;

export const GLOBAL_MINT_AUTHORITY_SEED = Buffer.from("global_mint_authority");