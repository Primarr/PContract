# Primar Contracts

Soroban smart contracts for **Primar** — agent-to-agent service payments on Stellar.

## Workspace (v2)

| Contract | Crate | Role |
|----------|-------|------|
| `registry` | service discovery | register / price / status with admin pause + events |
| `budget` | spending caps | session/task limits, over-budget checks |
| `settlement` | payment ledger | record settlements, protocol fee (BPS) |

## Build & test

```bash
make build
make test
```

Requires Rust with `wasm32-unknown-unknown`. Never commit private keys or deploy secrets.

## Version

Contract storage version: **2** (`initialize`, typed errors, pause controls).
