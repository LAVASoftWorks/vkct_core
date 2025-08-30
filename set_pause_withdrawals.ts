/**
 * Token registry adder
 * Author: lava.caballero@gmail.com
 *
 * Usage: ts-node set_pause_withdrawals.ts <address>
 */

import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import fs from "fs";
import os from 'os';
import minimist from 'minimist';
const argv = minimist(process.argv.slice(2));

if( argv._[0] === undefined || ! (argv._[0] === "true" || argv._[0] === "false") )
{
    console.log("\nMissing/invalid argument.\n");
    console.log("Usage: ts-node set_pause_withdrawals.ts true|false");
    process.exit(1);
}

const PAUSE_FLAG            = argv._[0] === 'true';
const REGISTRY_KEY          = "VkctPiggyBankV100tRegistry";
const IDL_FILE_PATH         = "target/idl/piggybank.json";
const ADMIN_KEYPAIR_PATH    = os.homedir() + "/.config/solana/id.json";
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

    await program.methods.setPauseWithdrawals(PAUSE_FLAG).accounts({
        registry: registryPda,
        admin: provider.wallet.publicKey,
    }).rpc();

    console.log("✅ Operation succeeded.");
})();
