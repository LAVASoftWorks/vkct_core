
# Volkachain Tokenizer Core

## About

This contract is used to validate fund withdrawals by owners of NFTs minted using the 
Volkachain Tokenizer framework.

If the signer of the withdrawal isn't the NFT owner, then the transaction will be rejected.

All NFTs minted with this framework feature a "piggy bank", which is a vault for SPL tokens that
are owned by the NFT, so if it is sold or transferred, the vault goes to the new owner.

## Why it is open source?

For two reasons:

- **Transparency.**  
  We want the users of our framework to be sure of what they're using, and the best we can do
  is being transparent. And to be transparent, we leave the program available for everyone.


- **Required for a verified build.**  
  [Solana Verified Builds](https://solana.com/es/developers/guides/advanced/verified-builds)
  require a program to be open source in order to appear as verified in the blockchain explorers.
  We fulfill the requirement and allow our software to have a trust check mark.

## License

This software is provided as-is and governed by the MIT license.
You can find it in the [LICENSE](LICENSE.md) file.

## How to build it yourself

Please check the [BUILD](BUILD.md) file for instructions.

## Security info

Please check the [SECURITY](SECURITY.md) file for information.

## About the developer

LAVA SoftWorks is a small company working on IT since 1997 and on the internet since 2000,
focusing on blockchain-driven solutions since 2014.

## Links

- Project website: https://volkachain.tech/tokenizer


- Developer website: https://www.lavasoftworks.com


- X account: https://x.com/LAVASoftWorks
