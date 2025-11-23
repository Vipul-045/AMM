import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amms } from "../target/types/amms";

import {createMint, getOrCreateAssociatedTokenAccount, mintTo, getAssociatedTokenAddress} from "@solana/spl-token";

describe("amm creation", () =>{

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.amms as Program<Amms>;
  const connection = provider.connection;
  const admin = provider.wallet;

  let mintX : anchor.web3.PublicKey;
  let mintY : anchor.web3.PublicKey;
  let lpMint : anchor.web3.PublicKey;

  let configPda : anchor.web3.PublicKey;
  let vaultX : anchor.web3.PublicKey;
  let vaultY : anchor.web3.PublicKey;

  let userAtaX: anchor.web3.PublicKey;
  let userAtaY: anchor.web3.PublicKey;
  let userAtaLp: anchor.web3.PublicKey;

  const seed = new anchor.BN(42);
  const fee = 30;

  it("Initializing the AMM pool", async () => {
    mintX = await createMint(connection, admin.payer, admin.publicKey, null, 6)
  })
  
}

)