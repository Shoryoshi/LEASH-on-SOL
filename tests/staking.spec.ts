import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LeashStaking } from "../target/types/leash_staking";
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { assert } from "chai";

describe("leash-staking", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.LeashStaking as Program<LeashStaking>;
  
  // Test accounts
  const authority = Keypair.generate();
  const user = Keypair.generate();
  const leashMint = Keypair.generate();
  const xleashMint = Keypair.generate();
  const treasury = Keypair.generate();
  
  // PDAs
  let globalStakingState: PublicKey;
  let userStakingPosition: PublicKey;
  
  before(async () => {
    // Airdrop SOL to test accounts
    const signature = await provider.connection.requestAirdrop(authority.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature);
    
    const userSignature = await provider.connection.requestAirdrop(user.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(userSignature);
    
    // Derive PDAs
    [globalStakingState] = PublicKey.findProgramAddressSync(
      [Buffer.from("global_staking_state")],
      program.programId
    );
    
    [userStakingPosition] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("user_staking_position"),
        user.publicKey.toBuffer(),
        globalStakingState.toBuffer()
      ],
      program.programId
    );
  });

  it("Initializes the staking program", async () => {
    const rewardRate = new anchor.BN(1000); // 1000 tokens per second per staked token
    const minStakeAmount = new anchor.BN(1000000); // 1 LEASH (assuming 6 decimals)
    const maxStakeAmount = new anchor.BN(1000000000000); // 1,000,000 LEASH
    const lockPeriod = new anchor.BN(86400); // 1 day
    
    try {
      await program.methods
        .initialize(rewardRate, minStakeAmount, maxStakeAmount, lockPeriod)
        .accounts({
          globalStakingState,
          leashMint: leashMint.publicKey,
          xleashMint: xleashMint.publicKey,
          treasury: treasury.publicKey,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([authority])
        .rpc();
      
      // Verify the global staking state was created
      const globalState = await program.account.globalStakingState.fetch(globalStakingState);
      assert.equal(globalState.authority.toString(), authority.publicKey.toString());
      assert.equal(globalState.rewardRate.toString(), rewardRate.toString());
      assert.equal(globalState.minStakeAmount.toString(), minStakeAmount.toString());
      assert.equal(globalState.maxStakeAmount.toString(), maxStakeAmount.toString());
      assert.equal(globalState.lockPeriod.toString(), lockPeriod.toString());
      assert.equal(globalState.totalStaked.toString(), "0");
      assert.equal(globalState.isPaused, false);
      
      console.log("✅ Staking program initialized successfully");
    } catch (error) {
      console.error("❌ Failed to initialize staking program:", error);
      throw error;
    }
  });

  it("Stakes LEASH tokens", async () => {
    const stakeAmount = new anchor.BN(10000000); // 10 LEASH
    
    try {
      await program.methods
        .stake(stakeAmount)
        .accounts({
          globalStakingState,
          userStakingPosition,
          userLeashAccount: user.publicKey, // This would be the actual token account
          userXleashAccount: user.publicKey, // This would be the actual token account
          leashMint: leashMint.publicKey,
          xleashMint: xleashMint.publicKey,
          treasury: treasury.publicKey,
          user: user.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([user])
        .rpc();
      
      // Verify the stake was recorded
      const userPosition = await program.account.userStakingPosition.fetch(userStakingPosition);
      assert.equal(userPosition.owner.toString(), user.publicKey.toString());
      assert.equal(userPosition.stakedAmount.toString(), stakeAmount.toString());
      
      const globalState = await program.account.globalStakingState.fetch(globalStakingState);
      assert.equal(globalState.totalStaked.toString(), stakeAmount.toString());
      
      console.log("✅ LEASH tokens staked successfully");
    } catch (error) {
      console.error("❌ Failed to stake LEASH tokens:", error);
      throw error;
    }
  });

  it("Gets staking statistics", async () => {
    try {
      const stats = await program.methods
        .getStakingStats()
        .accounts({
          globalStakingState,
        })
        .view();
      
      assert.equal(stats.totalStaked.toString(), "10000000");
      assert.equal(stats.isPaused, false);
      
      console.log("✅ Staking statistics retrieved successfully");
      console.log("Total staked:", stats.totalStaked.toString());
      console.log("Reward rate:", stats.rewardRate.toString());
    } catch (error) {
      console.error("❌ Failed to get staking statistics:", error);
      throw error;
    }
  });
});
