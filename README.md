# joai--apps-sc

MultiversX smart contracts that power JoAi's on-chain apps — coupons, signatures, challenges, shops, and loyalty programs.

Each contract backs a JoAi site app: when a user visits their booking or coupon page at `joai.ai`, these contracts handle the on-chain side of the transaction.

---

## Contracts

| Contract | What it does |
|----------|-------------|
| `coupon` | Issue SFT-based discount coupons. Merchants create coupon collections on-chain; customers redeem them by code. |
| `signature` | Multi-party signature requests. Create a document signing request and collect on-chain signatures from multiple parties. |
| `challenge` | On-chain challenges and competitions. Create, join, and resolve challenge rounds. |
| `shop` | On-chain shop registry. Register a shop slug and manage product listings. |
| `loyalty` | Loyalty point tracking. Reward customers with on-chain points redeemable in your shop. |

---

## Repository Structure

```
<contract>/
  src/              # Rust contract source
  output/           # Compiled artifacts (.wasm, .abi.json, .mxsc.json)
  tests/            # Integration tests (where applicable)
  sc-config.toml    # sc-meta build config
  meta/             # sc-meta build helper (auto-generated)
  wasm/             # wasm adapter (auto-generated, do not edit)
```

---

## Building

Requires the [MultiversX smart contract tools](https://docs.multiversx.com/developers/developer-reference/sc-meta/):

```bash
cargo install multiversx-sc-meta
```

Build a single contract:

```bash
cd coupon
sc-meta all build
```

Artifacts land in `<contract>/output/`.

---

## Deploying to Devnet

```bash
mxpy contract deploy \
  --bytecode coupon/output/coupon.wasm \
  --pem /path/to/deployer.pem \
  --proxy https://devnet-gateway.multiversx.com \
  --chain D \
  --gas-limit 100000000 \
  --send
```

For upgrades:

```bash
mxpy contract upgrade <contract-address> \
  --bytecode coupon/output/coupon.wasm \
  --pem /path/to/deployer.pem \
  --proxy https://devnet-gateway.multiversx.com \
  --chain D \
  --gas-limit 100000000 \
  --send
```

---

## Adding a New Contract

1. Create `<app>/` with the standard structure (copy `coupon/` as a starting point)
2. Add `"<app>"` to the workspace members in root `Cargo.toml`
3. Exclude `"<app>/meta"` and `"<app>/wasm"` from the workspace
4. Define the warp in `joai--warps/warps/<app>/` with a `brand.ts` and warp action JSONs
5. Add the contract address to the brand's `contracts` config in `brand.ts`

---

## Related Repos

- [`joai--warps`](https://github.com/JoAiHQ/joai--warps) — warp definitions and catalog (the off-chain interface to these contracts)
- [`joai--pwa`](https://github.com/JoAiHQ/joai--pwa) — the apps that render warp UIs for these contracts
- [`joai--api`](https://github.com/JoAiHQ/joai--api) — backend that handles site resolution and agent auth
