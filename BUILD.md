
# Volkachain Tokenizer Core Build Instructions

This needs to be done in a peak Ubuntu instance (24+).

⭐ **Important:** from the shell perspective, we're working on the account's home (E.G. `/home/user`).
If you cloned this repository, then you'll need to run everything from the directory where you cloned this repo.

⚠️ **Warning:** all Solana related topics ahead are considering **devnet**, but all files in the repo
are pointing to **mainnnet**. Make sure you read this document in full before starting to copy/paste.

## Setup

Install development tools:

```shell
sudo apt-get install -y build-essential pkg-config libudev-dev llvm \
                        libclang-dev protobuf-compiler libssl-dev
```

Now install Node.js if not already done:

```shell
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
# Restart SSH session before continuing

nvm install stable --lts
# Restart SSH session again
```

Now install the Solana toolset

```shell
# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Restart SSH session before continuing 
rustc --version
#> rustc 1.87.0 (17067e9ac 2025-05-09)

# Install the Solana CLI
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
# Restart SSH session before continuing
solana --version
#> solana-cli 2.2.18 (src:8392f753; feat:3073396398, client:Agave)

# Install AVM
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm --version
#> avm 0.31.1

# Install anchor
avm install latest
#> Now using anchor version 0.31.1.
# Verify anchor:
anchor --version
#> anchor-cli 0.31.1

# Install Yarn:
npm install --global yarn
yarn --version
#> 1.22.22

# Install ts-node for using the TS helpers:
npm install --global ts-node
#> added 20 packages in 2s
```

## Make the admin / upgrade authority account

Make a default keypair to set the piggy bank contract authority, set it as default and then fund it:

```shell
solana-keygen new --no-bip39-passphrase
#> Wrote new keypair to /home/someone/.config/solana/id.json
#> =============================================================================
#> pubkey: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
#> =============================================================================
#> Save this seed phrase to recover your new keypair:
#> xxxx xxxxxxx xxxxxx xxxxxx xxxxxxx xxxxx xxxxx xxxx xxxxx xxxxxxxx xxxxx xxxx
#> =============================================================================

solana config set --url devnet
#> Config File: /home/user/.config/solana/cli/config.yml
#> RPC URL: https://api.devnet.solana.com
#> WebSocket URL: wss://api.devnet.solana.com/ (computed)
#> Keypair Path: /home/user/.config/solana/id.json
#> Commitment: confirmed

solana airdrop 5
#> Requesting airdrop of 5 SOL
#> Signature: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
#> 5 SOL
```
⭐ **Important**:

- The pubkey will be the program authority. MAKE SURE TO BACKUP THIS INFO!

## Install dependencies

```shell
npm install
```

## Generate a new account to host the program

You'll need to create an account to host the program.

```shell
mkdir -p target/deploy/
solana-keygen new --no-bip39-passphrase --outfile target/deploy/piggybank-keypair.json
#> Wrote new keypair to target/deploy/piggybank-keypair.json
#> =============================================================================
#> pubkey: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
#> =============================================================================
#> Save this seed phrase to recover your new keypair:
#> xxxx xxxxxxx xxxxxx xxxxxx xxxxxxx xxxxx xxxxx xxxx xxxxx xxxxxxxx xxxxx xxxx
#> =============================================================================
```

⭐ **Important**:

- The pubkey will be the program id. MAKE SURE TO BACKUP THIS INFO!

## Customize the project files

You need to edit some files:

- `programs/piggybank/src/lib.rs`:
  - At the top, set **the program id** (pubkey from above).
  - At the bottom, edit the `security_txt` macro.


- `programs/piggybank/Cargo.toml`:
  - Set a description that suits your needs.


- `Anchor.toml`:
  - On the `[programs.localnet]` section, set the `piggybank` value to **the program id**.
  - On the `[programs.devnet]` section, set the `piggybank` value to **the program id**.
  - On the `[programs.mainnet]` section, set the `piggybank` value to **the program id**.
  - On the `[provider]` section, set `cluster = "devnet"`

## Build the project

```shell
anchor build
#>   Compiling piggybank v0.0.1 (/home/user/piggybank/programs/piggybank)
#>    Finished `release` profile [optimized] target(s) in 1.66s
#>   Compiling piggybank v0.0.1 (/home/user/piggybank/programs/piggybank)
#>    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.97s
#>     Running unittests src/lib.rs (/home/user/piggybank/target/debug/deps/piggybank-ed23ef5af5b62355)
```

Deploy:

```shell
#> anchor deploy
#> Deploying cluster: https://api.devnet.solana.com
#> Upgrade authority: /home/piggybank/.config/solana/id.json
#> Deploying program "piggybank"...
#> Program path: /home/piggybank/target/deploy/piggybank.so...
#> Program Id: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
#> 
#> Signature: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
#> 
#> Deploy success
```

SAVE the next items:
- `.config/solana/id.json` file <-- Upgrade authority


- `target/deploy/piggybank-keypair.json` file <-- Contract private key


- Program Id from the output above (the pubkey of the piggybank keypair).


- `target/idl/piggybank.json` file <-- IDL for typescript stuff.

You won't need them anywhere else, but if you lose them, you'll lose access
to the piggy bank contract.

## Customize the TypeScript helpers

The TS files to initialize/add tokens/collections need to be customized.
Please edit them before running them.

## Initialize the valid SPL tokens registry and add tokens to it

```shell
# To initialize the registry (only once):
ts-node init_token_registry.ts

# To add a token to the registry:
ts-node add_token_to_registry.ts "token_mint_address"
```


## Initialize the valid collections registry and add a collection to it

```shell
# To initialize the registry (only once):
ts-node init_collection_registry.ts

# Warning: make sure to mint a collection using the tokenizer and 
# to add a collection to the registry:
ts-node add_collection_to_registry.ts "collection_mint_address"
```
