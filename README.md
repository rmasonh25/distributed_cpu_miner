# Distributed CPU Miner

Part of the **MyBuji Miner** project — this is a lightweight, license-activated Rust-based CPU miner that supports configurable step sizes and orchestration for solo mining.

## 💡 Project Overview

This component is designed for:
- Solo CPU-based mining against your own node or private pool
- Configurable mining step (instead of default nonce += 1)
- Deployment across many machines via Docker
- Centralized orchestration and wallet binding
- Works on Raspberry Pi, Linux, or any Docker-supported device

## 🔧 CLI Usage

```bash
./distributed_cpu_miner \
  --range-start 0 \
  --range-end 10000000 \
  --step 10 \
  --wallet 1TestWallet \
  --license-key DEMO-KEY
