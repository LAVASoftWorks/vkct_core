/**
 * Token registry initializer
 * Author: lava.caballero@gmail.com
 *
 * Usage: ts-node init_token_registry.ts
 */

import * as anchor from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import fs from "fs";

const REGISTRY_KEY          = "VkctPiggyBankV050tRegistry";
const IDL_FILE_PATH         = "target/idl/piggybank.json";
const ADMIN_KEYPAIR_PATH    = ".config/solana/id.json";
const SOLANA_NETWORK_TARGET = "https://api.devnet.solana.com";

(async () => {
    // Load the admin keypair
    const secret = JSON.parse(fs.readFileSync(ADMIN_KEYPAIR_PATH, "utf8"));
    const admin = anchor.web3.Keypair.fromSecretKey(new Uint8Array(secret));

    // Set up Anchor provider
    const connection = new anchor.web3.Connection(SOLANA_NETWORK_TARGET, "confirmed");
    const wallet = new anchor.Wallet(admin);
    const provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
    anchor.setProvider(provider);

    // Load your program IDL
    const idl = JSON.parse(fs.readFileSync(IDL_FILE_PATH, "utf8"));
    const program = new anchor.Program(idl, provider);
    const PROGRAM_ID = new PublicKey(idl.address);

    const [registryPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(REGISTRY_KEY)],
      PROGRAM_ID
    );

    console.log("Initializing registry at:", registryPda.toBase58());

    const tx = await program.methods.initializeRegistry().accounts({
      registry: registryPda,
      admin: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    }).rpc();

    console.log("✅ Registry initialized successfully");
    console.log("Transaction signature:", tx);
})();
