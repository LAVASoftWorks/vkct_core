/**
 * Collection registry adder
 * Author: lava.caballero@gmail.com
 *
 * Usage: ts-node add_collection_to_registry.ts <address>
 */

import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import fs from "fs";
import os from 'os';
import minimist from 'minimist';
const argv = minimist(process.argv.slice(2));

if( argv._[0] === undefined )
{
    console.log("\nMissing collection mint address.\n");
    console.log("Usage: ts-node add_collection_to_registry.ts <nft_collection_mint>");
    process.exit(1);
}

const COLLECTION_MINT          = new PublicKey(argv._[0]);
const COLLECTION_REGISTRY_SEED = "VkctPiggyBankV100cRegistry";
const ADMIN_KEYPAIR_PATH       = os.homedir() + "/.config/solana/id.json";
const IDL_FILE_PATH            = "target/idl/piggybank.json";
const SOLANA_NETWORK_TARGET    = "https://api.mainnet-beta.solana.com";

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

    // Derive the registry PDA
    const [collectionRegistryPda] = PublicKey.findProgramAddressSync(
        [Buffer.from(COLLECTION_REGISTRY_SEED)],
        PROGRAM_ID
    );

    console.log("Adding collection:", COLLECTION_MINT.toBase58());

    const tx = await program.methods
        .addCollection(COLLECTION_MINT)
        .accounts({
            collectionRegistry: collectionRegistryPda,
            admin: admin.publicKey,
        })
        .signers([admin])
        .rpc();

    console.log("✅ Collection added to registry");
    console.log("Transaction signature:", tx);
})();
