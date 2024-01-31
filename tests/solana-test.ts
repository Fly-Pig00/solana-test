import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaTest } from "../target/types/solana_test";
import { BN } from "@project-serum/anchor";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { createMint, getAssociatedTokenAddressSync, createAccount, getAccount, mintTo } from "@solana/spl-token";
import { assert } from "chai";

describe("solana-test", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaTest as Program<SolanaTest>;

  it("Initialized Launchpad", async () => {
    // Add your test here.
    const creator = Keypair.generate();
    const price = new BN(10);
    const amount = new BN(10000);
    const softCap = new BN(10000);
    const hardCap = new BN(10000);
    const launchTime = new BN(111);
    const endTime =  new BN(222);

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(creator.publicKey, 10000000000),
      "confirmed"
    );

    // Create a new mint and initialize it
    const mint = await createMint(
      program.provider.connection,
      creator,
      creator.publicKey,
      null,
      0
    );

    const fromAta = await createAccount(provider.connection, creator, mint, creator.publicKey);

    await mintTo(provider.connection, creator, mint, fromAta, creator, amount.toNumber());

    const [launchpadDetails, bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("launchpad")), mint.toBuffer(), creator.publicKey.toBuffer()],
      program.programId
    );

    const toAta = getAssociatedTokenAddressSync(mint, launchpadDetails, true);

    const tiers = [new BN(4000), new BN(3000), new BN(2000), new BN(1000)];

    const tx = await program.methods
      .initializeLaunchpad(price, amount, tiers, softCap, hardCap, launchTime, endTime)
      .accounts({
        launchpadDetails: launchpadDetails,
        creator: creator.publicKey,
        tokenMint: mint,
        fromAta,
        toAta,
      })
      .signers([creator])
      .rpc();

    const launchpadDetailsData = await program.account.details.fetch(launchpadDetails);

    const toTokenAccount = await getAccount(provider.connection, toAta);

    assert.ok(launchpadDetailsData.creator.equals(creator.publicKey), "The Creator Address is not the same.");
    assert.ok(launchpadDetailsData.tokenMint.equals(mint), "The token mint is not the same.");
    assert.equal(launchpadDetailsData.price.toNumber(), price.toNumber(), "The price is not 10.");
    assert.equal(launchpadDetailsData.amount.toNumber(), amount.toNumber(), "The amount is not 10000.");
    assert.equal(launchpadDetailsData.tiers.length, 4, "The tiers length must be 4.");
    assert.equal(launchpadDetailsData.tiers.reduce((total, num) => total + num.toNumber(), 0), amount.toNumber(), "The tiers sum must be amount.");
    assert.equal(launchpadDetailsData.softCap.toNumber(), softCap.toNumber(), "The soft cap must be 10000.");
    assert.equal(launchpadDetailsData.hardCap.toNumber(), hardCap.toNumber(), "The hard cap must be 10000.");
    assert.equal(launchpadDetailsData.launchTime.toNumber(), launchTime.toNumber(), "The launch time must be 10000.");
    assert.equal(launchpadDetailsData.endTime.toNumber(), endTime.toNumber(), "The end time must be 10000.");
    
    assert.equal(Number(toTokenAccount.amount), amount.toNumber(), "The transferred token amount must be 10000.");

    console.log(launchpadDetailsData);
  });
});
