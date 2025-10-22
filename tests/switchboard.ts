import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Switchboard } from "../target/types/switchboard";
import { PublicKey } from "@solana/web3.js";

describe("switchboard", () => {
  // using local rider provider to talk to devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  
  const program = anchor.workspace.switchboard as Program<Switchboard>;

  const BTC_FEED = new PublicKey("3NBReDRTLWws6Kmjh8gE4bM8KTZBnGjQhkU4M5oK2n2L");
  const SOL_FEED = new PublicKey("J83w4HKfqxwcq3B1t8WY2kM9K56g6cDxgtJb9u7rRrJ");
  const ETH_FEED = new PublicKey("EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw");
  
  it("☑️ Fetching BTC, SOL, ETH Prices from Pyth", async () => {
    const tx = await program.methods.getPrices().accounts({
      btcFeed: BTC_FEED,
      solFeed: SOL_FEED,
      ethFeed: ETH_FEED,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();

    console.log("☑️ Tx Signature: ", tx);
  });
});
