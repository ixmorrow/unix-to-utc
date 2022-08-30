import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TestClock } from "../target/types/test_clock";

describe("test-clock", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TestClock as Program<TestClock>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
