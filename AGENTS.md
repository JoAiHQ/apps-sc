# joai--apps-sc

MultiversX smart contracts powering JoAi site apps (WarpApps).

## Structure

Each app is a self-contained Rust workspace member:

```
<app>/          - contract source (src/, sc-config.toml, multiversx.json)
<app>/meta/     - sc-meta build helper
<app>/wasm/     - wasm adapter (auto-generated, do not edit)
<app>/output/   - compiled artifacts (.wasm, .mxsc.json, .abi.json)
```

## Build

```bash
# Build a single contract
cd <app> && sc-meta all build

# Build with wasm32-unknown-unknown (required for current devnet)
cd <app> && RUSTFLAGS="-C link-arg=-s" cargo build \
  --manifest-path wasm/Cargo.toml \
  --target wasm32-unknown-unknown \
  --release \
  --target-dir meta/target
cp meta/target/wasm32-unknown-unknown/release/<app>_wasm.wasm output/<app>.wasm
```

## Deploy (devnet)

```bash
mxpy contract deploy \
  --bytecode <app>/output/<app>.wasm \
  --pem /path/to/deployer.pem \
  --proxy https://devnet-gateway.multiversx.com \
  --chain D \
  --gas-limit 100000000 \
  --send
```

## Contracts

| Contract | Devnet address | Owner |
|----------|---------------|-------|
| `challenge` | `erd1qqqqqqqqqqqqqpgqkwkadfc58fz38lhs0dxcusz2qsrunkgwncrsjfpgdt` | JoAi server signer |
| `loyalty` | `erd1qqqqqqqqqqqqqpgq9gagzdqrjlpp2nqllmcdzuvpjna3llh5tresc3qye8` | JoAi server signer |

## Adding a new contract

1. Create `<app>/` with the standard structure
2. Add `"<app>"`, `"<app>/meta"`, `"<app>/wasm"` to root `Cargo.toml` workspace members
3. Add brand config in `joai--warps/warps/<app>/brand.ts`
4. Add site config in `warps--pwa/app/app/brands.ts`
