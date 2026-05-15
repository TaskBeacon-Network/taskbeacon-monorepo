# TaskBeacon Monorepo

Decentralized task management built on Stellar/Soroban.

## Structure

```
taskbeacon-monorepo/
├── apps/
│   ├── web/          # Next.js frontend (TypeScript)
│   └── indexer/      # Node.js/Express event indexer (TypeScript)
├── contracts/
│   └── taskbeacon/   # Soroban smart contract (Rust)
├── Cargo.toml        # Rust workspace root
├── package.json      # npm workspaces root
└── turbo.json        # Turborepo pipeline
```

## Tech Stack

| Layer     | Technology                  |
|-----------|-----------------------------|
| Frontend  | Next.js 14, React 18, TypeScript |
| Backend   | Node.js, Express, TypeScript |
| Contract  | Rust, Soroban SDK 21, Stellar |
| Monorepo  | Turborepo + npm workspaces (JS), Cargo workspaces (Rust) |

## Getting Started

### Prerequisites
- Node.js 20+
- Rust + `wasm32-unknown-unknown` target
- Stellar CLI (`stellar`)

### Install JS dependencies
```bash
npm install
```

### Run frontend (dev)
```bash
npm run dev -w web
```

### Run indexer (dev)
```bash
npm run dev -w indexer
```

### Build Soroban contract
```bash
cargo build --release --target wasm32-unknown-unknown -p taskbeacon
```

### Run all (Turborepo)
```bash
npm run dev
```
