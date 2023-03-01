import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { createAssociatedTokenAccount, createMint } from "@solana/spl-token";
import { DrinkChallengeTask } from "../target/types/drink_challenge_task";

describe("drink-challenge-task", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .DrinkChallengeTask as Program<DrinkChallengeTask>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("start challenge!", async () => {
    // Add your test here.

    console.log("1-wallet", provider.wallet.publicKey);

    const nft = await createMint(
      provider.connection,
      provider.wallet.payer, // payer can be used as the wallet signer
      provider.wallet.publicKey,
      provider.wallet.publicKey,
      0
    );
    console.log("2-nft", nft);

    const holderATA = await createAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer, // payer can be used as the wallet signer
      nft,
      provider.wallet.publicKey
    );
    console.log("3-holder", holderATA);

    const [firstOwnerPDA, _] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("first-owner"), nft.toBuffer()],
      program.programId
    );
    console.log("4-firstOwnerPDA", firstOwnerPDA);

    const [challengeNftListPDA, bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("challenge-nft-list"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );
    console.log("5-challengeNftListPDA", challengeNftListPDA);

    await program.methods
      .startChallenge()
      .accounts({
        owner: provider.wallet.publicKey,
        nftMint: nft,
        holder: holderATA,
        firstOwner: firstOwnerPDA,
        challengeNftList: challengeNftListPDA,
      })
      .rpc();

    let firstOwner = await program.account.firstOwner.fetch(firstOwnerPDA);
    console.log("result-firstOwner", firstOwner);

    let challengeNftList = await program.account.challengeNftList.fetch(
      challengeNftListPDA
    );
    console.log("result-challengeNftList", challengeNftList);

    // await program.methods
    //   .startChallenge()
    //   .accounts({
    //     owner: provider.wallet.publicKey,
    //     nftMint: nft,
    //     holder: holderATA,
    //     firstOwner: firstOwnerPDA,
    //     challengeNftList: challengeNftListPDA,
    //   })
    //   .rpc();
    //
    // firstOwner = await program.account.firstOwner.fetch(firstOwnerPDA);
    // console.log("result-firstOwner-2", firstOwner);
    //
    // challengeNftList = await program.account.challengeNftList.fetch(
    //   challengeNftListPDA
    // );
    // console.log("result-challengeNftList-2", challengeNftList);

    await program.methods
      .endChallenge()
      .accounts({
        owner: provider.wallet.publicKey,
        nftMint: nft,
        holder: holderATA,
        challengeNftList: challengeNftListPDA,
      })
      .rpc();
    challengeNftList = await program.account.challengeNftList.fetch(
      challengeNftListPDA
    );
    console.log("result-challengeNftList-end", challengeNftList);
  });
});
