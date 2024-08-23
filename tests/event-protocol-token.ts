import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EventProtocolToken } from "../target/types/event_protocol_token";

describe("event-protocol-token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EventProtocolToken as Program<EventProtocolToken>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.hello().rpc();
    console.log("Your transaction signature", tx);
  });
});
