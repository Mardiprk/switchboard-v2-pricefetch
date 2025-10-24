import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Switchboard } from "../target/types/switchboard";
import { PublicKey } from "@solana/web3.js";

describe("switchboard", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Switchboard as Program<Switchboard>;

  // Switchboard V2 Devnet Aggregator Feeds
  const BTC_FEED = new PublicKey("8SXvChNYFhRq4EZuZvnhjrB3jJRQCv4k3P4W6hesH3Ee");
  const SOL_FEED = new PublicKey("GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR");
  const ETH_FEED = new PublicKey("JBu1AL4obBcCMqKBBxhpWCNUt136ijcuMZLFvTP7iWdB");

  
  it("☑️ Fetching BTC, SOL, ETH Prices from Switchboard V2", async () => {
    try {
      const tx = await program.methods.getPrices().accounts({
        btcFeed: BTC_FEED,
        solFeed: SOL_FEED,
        ethFeed: ETH_FEED,
      }).rpc();

      console.log(`✔️ Transaction Signature: https://solana.fm/address/${tx}/transactions?cluster=devnet-alpha`);

      await provider.connection.confirmTransaction(tx, "confirmed");

      const txDetails = await provider.connection.getTransaction(tx, {
        maxSupportedTransactionVersion: 0,
        commitment: "confirmed",
      });

      console.log("\n> Price Data:");
      console.log("=".repeat(60));
      txDetails?.meta?.logMessages?.forEach((log) => {
        console.log(log);
      });
      console.log("=".repeat(60));
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });
});