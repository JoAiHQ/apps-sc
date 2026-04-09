# 🔐 JoAi Smart Contracts

The on-chain backbone of JoAi apps. Each contract powers a real user-facing feature — from issuing discount coupons to collecting verifiable signatures — all anchored on the blockchain.

---

## 📦 Contracts

| Contract | What it does |
|----------|-------------|
| `coupon` | 🎟️ Merchants issue discount coupons; customers redeem them by code. |
| `signature` | ✍️ Multi-party document signing. Collect verifiable on-chain signatures from multiple people. |
| `challenge` | 🏆 Create and resolve on-chain challenges and competitions. |
| `shop` | 🛍️ On-chain shop registry. Register a shop and manage product listings. |
| `loyalty` | ⭐ Reward customers with on-chain loyalty points redeemable in your shop. |

---

## 🗂️ Repository Structure

```
<contract>/
  src/              # Rust contract source
  output/           # Compiled artifacts (.wasm, .abi.json)
  tests/            # Integration tests (where applicable)
  sc-config.toml    # Build config
  meta/             # Build helper (auto-generated)
  wasm/             # Wasm adapter (auto-generated, do not edit)
```

---

## 🔨 Building

Requires the smart contract build tools:

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

## 🚀 Deploying

```bash
mxpy contract deploy \
  --bytecode coupon/output/coupon.wasm \
  --pem /path/to/deployer.pem \
  --proxy https://devnet-gateway.multiversx.com \
  --chain D \
  --gas-limit 100000000 \
  --send
```

For upgrades, replace `deploy` with `upgrade <contract-address>`.

---

## ➕ Adding a New Contract

1. Create `<app>/` with the standard structure (copy `coupon/` as a starting point)
2. Add `"<app>"` to workspace members in root `Cargo.toml`
3. Exclude `"<app>/meta"` and `"<app>/wasm"` from the workspace
4. Define the warp in `joai--warps/warps/<app>/` with a `brand.ts` and warp action JSONs
5. Register the deployed contract address in the brand's `contracts` config

---

## 🔗 Related Repos

- [`joai--warps`](https://github.com/JoAiHQ/joai--warps) — warp definitions and catalog (the off-chain interface to these contracts)
- [`joai--pwa`](https://github.com/JoAiHQ/joai--pwa) — the apps that render warp UIs for end users
- [`joai--api`](https://github.com/JoAiHQ/joai--api) — backend handling site resolution and agent auth
