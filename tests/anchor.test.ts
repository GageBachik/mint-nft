// No imports needed: web3, anchor, pg and more are globally available
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  mintTo,
  MINT_SIZE,
  createMint,
  createInitializeMintInstruction,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
} from "@solana/spl-token";

describe("Test", () => {
  it("mint-nft", async () => {
    // Add your test here.
    // console.log("INF");
    const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
      "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );
    const lamports =
      await pg.program.provider.connection.getMinimumBalanceForRentExemption(
        MINT_SIZE
      );
    const mintKey = anchor.web3.Keypair.generate();

    let ata = await getAssociatedTokenAddress(
      mintKey.publicKey, // mint
      pg.program.provider.wallet.publicKey // owner
    );

    // console.log("ACCOUNT", res);
    console.log("MINTKEY", mintKey.publicKey.toString());
    console.log("USER", pg.program.provider.wallet.publicKey.toString());
    const [metadatakey] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKey.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );
    console.log("METDATA", metadatakey.toString());
    const [masterKey] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKey.publicKey.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    console.log("MA", masterKey.toString());
    const tx = await pg.program.methods
      .mintNft(
        "Solana Hacker",
        "SHACK",
        "https://ipfs.sea.tube/ipfs/QmUsxNproGJfYAmx975LKHZYJsjz1rFFBjNVEiz9Andz66",
        500
      )
      .accounts({
        masterEdition: masterKey,
        metadataMint: mintKey.publicKey,
        updateAuth: pg.program.provider.wallet.publicKey,
        mplProgram: TOKEN_METADATA_PROGRAM_ID,
        metadataAccount: metadatakey,
        recieverAccount: ata,
        metadata: metadatakey,
        mint: mintKey.publicKey,
        payer: pg.program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        ataProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .preInstructions([
        anchor.web3.SystemProgram.createAccount({
          fromPubkey: pg.program.provider.wallet.publicKey,
          newAccountPubkey: mintKey.publicKey,
          space: MINT_SIZE,
          lamports,
          programId: TOKEN_PROGRAM_ID,
        }),
        createInitializeMintInstruction(
          mintKey.publicKey, // mint pubkey
          0, // decimals
          pg.program.provider.wallet.publicKey, // mint authority
          pg.program.provider.wallet.publicKey // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
        ),
        createAssociatedTokenAccountInstruction(
          pg.program.provider.wallet.publicKey,
          ata,
          pg.program.provider.wallet.publicKey,
          mintKey.publicKey
        ),
      ])
      .signers([mintKey])
      .rpc();

    console.log("MINTED", tx);
  });
});
