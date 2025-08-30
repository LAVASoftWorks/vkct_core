/**
 * Withdrawal status
 * Author: lava.caballero@gmail.com
 *
 * Usage: ts-node show_withdrawal_status.ts
 */

import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import fs from "fs";
import os from 'os';

const REGISTRY_SEED         = "VkctPiggyBankV100tRegistry";
const SOLANA_NETWORK_TARGET = "https://api.devnet.solana.com";
const ADMIN_KEYPAIR_PATH    = os.homedir() + "/.config/solana/id.json";
const IDL_FILE_PATH         = "target/idl/piggybank.json";

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

    // Derive TokenRegistry PDA
    const [registryPda] = PublicKey.findProgramAddressSync(
        [Buffer.from(REGISTRY_SEED)],
        program.programId
    );

    // @ts-ignore Fetch and decode the account
    const registry = await program.account.tokenRegistry.fetch(registryPda);

    // console.log("📜 Withdrawal status");
    // console.log("Admin:", registry.admin.toBase58());
    const paused: boolean = Boolean(registry.pausedWithdrawals);
    console.log(`Withdrawals are ${paused ? "PAUSED 🔴" : "ACTIVE 🟢"}`);
})();
