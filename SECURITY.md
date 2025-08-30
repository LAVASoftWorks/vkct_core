
# Volkachain Tokenizer Core Security Policy

## Scope

This policy covers the Solana programs in this repo:

- Program: `piggybank` — Program ID: `LvPibsi1V7z71Nmqt3pnafDBgaoeMhiwNVDYMdYc2tG`
  Networks: mainnet-beta, testnet, devnet.

Out of scope: marketing sites, docs, analytics, third-party wallets, RPC providers.

## Reporting a Vulnerability

Please **do not** open public issues. Use one of:

- **GitHub Security Advisories**: https://github.com/LAVASoftWorks/vkct_core/security/advisories/new


- **Contact channels**: https://volkachain.tech/contact/

Include:
- Impact summary and severity estimate
- Reproduction steps (IDL hash, Anchor/Solana versions, txids/logs)
- Affected commit/tag/Program ID and network
- Any suggested mitigations

We support encrypted reports (PGP) and respect safe harbor for good-faith research.

## Coordinated Disclosure & Timelines

- **Ack** within 48h → **Triage** within 5 business days.


- **Fix** targeted within 21 days for high/critical issues; may include:
    - Pausing sensitive functions (e.g., withdrawals)
    - Emergency upgrade with audited patch
    - Public advisory after patch + grace period


- Reporter credit by handle or name (opt-in).

## Bounty

If your finding prevents loss of funds or bypasses critical auth/upgrade controls,
we may award a bounty depending on severity and exploitability.
Contact us for more information.

## Emergency On-Chain Procedures
 
- **Circuit breaker / Pause:** Maintained by authority
  [LvAvGChA...GAMpaZA3](https://solscan.io/account/LvAvGChADF42u5ujjhk1HEd4Nxx8AfBq4QRGAMpaZA3).  
  Used only *to prevent active exploitation* or protect user funds.


- **Upgrade authority:** Held by
  [LvAvGChA...GAMpaZA3](https://solscan.io/account/LvAvGChADF42u5ujjhk1HEd4Nxx8AfBq4QRGAMpaZA3).  
  Key rotation is logged in [CHANGELOG](CHANGELOG.md) and releases.


- **User comms:** Advisories in GitHub Releases + Website blog entry / X post within 24h of fix.

## Exclusions
 
- DoS, spam, rate-limits, social engineering, physical attacks, third-party service bugs,
  issues requiring MITM on user’s device, and already-public vulnerabilities.

## Versions & Dependencies

We publish affected **tags/SHAs** and the **IDL hash** per advisory. Critical deps (e.g.,
`mpl-token-metadata`) are reviewed when upstream advisories are published.

## Past Advisories

- (none yet)

## Safe Harbor

We will not pursue legal action against security research conducted in good faith
that adheres to this policy and avoids privacy violations, service disruption, and
data exfiltration beyond the minimum necessary to prove a finding.
