import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Switchboard } from "../target/types/switchboard";
import { PublicKey } from "@solana/web3.js";

describe("switchboard", () => {
  // using local rider provider to talk to devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  
  const program = anchor.workspace.switchboard as Program<Switchboard>;

  const BTC_FEED = new PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");
  const SOL_FEED = new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix");
  const ETH_FEED = new PublicKey("EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw");
  
  it("☑️ Fetching BTC, SOL, ETH Prices from Pyth", async () => {
    try{
      const tx = await program.methods.getPrices().accounts({
        btcFeed: BTC_FEED,
        solFeed: SOL_FEED,
        ethFeed: ETH_FEED,
      }).rpc();

      console.log("☑️ Tx Signature: ", tx);

    await provider.connection.confirmTransaction(tx, "confirmed");

      const txDetails = await provider.connection.getTransaction(tx, {
        maxSupportedTransactionVersion: 0,
        commitment: "confirmed",
      });

      console.log("\n💰 Price Data:");
      txDetails?.meta?.logMessages?.forEach((log) => {
        console.log(log);
      });
    } catch (error) {
      console.error("❌ Error:", error);
      throw error;
    }
  });

  // it("☑️ Fetching Prices with Validation (5 min staleness)", async () => {
  //   try {
  //     const tx = await program.methods
  //       .getPricesWithValidation()
  //       .accounts({
  //         btcFeed: BTC_FEED,
  //         solFeed: SOL_FEED,
  //         ethFeed: ETH_FEED,
  //       })
  //       .rpc();

  //     console.log("✅ Validated prices fetched:", tx);

  //     await provider.connection.confirmTransaction(tx, "confirmed");

  //     const txDetails = await provider.connection.getTransaction(tx, {
  //       maxSupportedTransactionVersion: 0,
  //       commitment: "confirmed",
  //     });

  //     console.log("\n💰 Validated Prices:");
  //     txDetails?.meta?.logMessages?.forEach((log) => {
  //       if (log.includes("USD:")) {
  //         console.log(log);
  //       }
  //     });
  //   } catch (error) {
  //     console.error("⚠️ Validation failed (prices may be stale on devnet):", error.message);
  //   }
  // });
});
