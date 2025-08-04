/**
 * Collection registry initializer
 * Author: lava.caballero@gmail.com
 *
 * Usage: ts-node init_collection_registry.ts
 */

import * as anchor from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import fs from "fs";

// Replace with your values
const COLLECTION_REGISTRY_SEED = "VkctPiggyBankV050cRegistry";
const ADMIN_KEYPAIR_PATH       = ".config/solana/id.json";
const IDL_FILE_PATH            = "target/idl/piggybank.json";
const SOLANA_NETWORK_TARGET    = "https://api.devnet.solana.com";

(async () => {
    // Load the admin keypair
    const secret = JSON.parse(fs.readFileSync(ADMIN_KEYPAIR_PATH, "utf8"));
    const admin = anchor.web3.Keypair.fromSecretKey(new Uint8Array(secret));

    // Set up Anchor provider
    const connection = new anchor.web3.Connection(SOLANA_NETWORK_TARGET, "confirmed");
    const wallet = new anchor.Wallet(admin);
    const provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
    anchor.setProvider(provider);

    // Load the IDL and program
    const idl = JSON.parse(fs.readFileSync(IDL_FILE_PATH, "utf8"));
    const program = new anchor.Program(idl, provider);
    const PROGRAM_ID = new PublicKey(idl.address);

    // Derive the collection registry PDA
    const [collectionRegistryPda] = PublicKey.findProgramAddressSync(
        [Buffer.from(COLLECTION_REGISTRY_SEED)],
        PROGRAM_ID
    );

    console.log("Initializing collection registry at:", collectionRegistryPda.toBase58());

    const tx = await program.methods
        .initializeCollectionRegistry()
        .accounts({
            collectionRegistry: collectionRegistryPda,
            admin: admin.publicKey,
            systemProgram: SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

    console.log("✅ Collection registry initialized");
    console.log("Transaction signature:", tx);
})();
