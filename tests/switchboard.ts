import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Switchboard } from "../target/types/switchboard";
import { PublicKey } from "@solana/web3.js";

describe("switchboard", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Switchboard as Program<Switchboard>;

  const BTC_FEED = new PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");
  const SOL_FEED = new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix");
  const ETH_FEED = new PublicKey("EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw");
  
  it("☑️ Fetching BTC, SOL, ETH Prices from Pyth (Devnet)", async () => {
    try {
      const tx = await program.methods.getPrices().accounts({
        btcFeed: BTC_FEED,
        solFeed: SOL_FEED,
        ethFeed: ETH_FEED,
      }).rpc();

      console.log("✅ Transaction Signature:", tx);

      await provider.connection.confirmTransaction(tx, "confirmed");

      const txDetails = await provider.connection.getTransaction(tx, {
        maxSupportedTransactionVersion: 0,
        commitment: "confirmed",
      });

      console.log("\n💰 Price Data:");
      console.log("=".repeat(60));
      txDetails?.meta?.logMessages?.forEach((log) => {
        console.log(log);
      });
      console.log("=".repeat(60));
    } catch (error) {
      console.error("❌ Error:", error);
      throw error;
    }
  });
});