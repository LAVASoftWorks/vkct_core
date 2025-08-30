/**
 * Collection registry closer
 * Author: lava.caballero@gmail.com
 *
 * Usage: ts-node close_collection_registry.ts
 */

import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import fs from "fs";
import os from 'os';

const COLLECTION_REGISTRY_SEED = "VkctPiggyBankV100cRegistry";
const SOLANA_NETWORK_TARGET    = "https://api.mainnet-beta.solana.com";
const ADMIN_KEYPAIR_PATH       = os.homedir() + "/.config/solana/id.json";
const IDL_FILE_PATH            = "target/idl/piggybank.json";

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

    // Derive the collection registry PDA
    const [collectionRegistryPda] = PublicKey.findProgramAddressSync(
        [Buffer.from(COLLECTION_REGISTRY_SEED)],
        program.programId
    );

    await program.methods
        .closeCollectionsRegistry()
        .accounts({
            registry: collectionRegistryPda,
            admin: wallet.publicKey,
        })
        .rpc();
})();
