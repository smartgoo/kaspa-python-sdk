# API Reference

This section provides detailed API documentation for all classes and functions in the Kaspa Python SDK.

## Modules Overview

### Core Types

| Module | Description |
|--------|-------------|
| [Address](address.md) | Address creation, validation, and manipulation |
| [Keys](keys.md) | Mnemonics, private/public keys, HD derivation |
| [Transactions](transactions.md) | Transaction building, signing, and submission |
| [RPC](rpc.md) | Node communication via WebSocket RPC |
| [Utilities](utilities.md) | Network types, scripts, conversions |

## Quick Reference

### Address Operations

```python
from kaspa import Address, pay_to_address_script, address_from_script_public_key

address = Address("kaspa:qz...")
is_valid = Address.validate("kaspa:qz...")
script = pay_to_address_script(address)
```

### Key Management

```python
from kaspa import Mnemonic, XPrv, PrivateKeyGenerator, PublicKeyGenerator

mnemonic = Mnemonic.random()
xprv = XPrv(mnemonic.to_seed())
key_gen = PrivateKeyGenerator(xprv.to_string(), False, 0)
```

### Transactions

```python
from kaspa import Generator, PaymentOutput, sign_transaction

generator = Generator(network_id, entries, change_address, outputs=[...])
for pending_tx in generator:
    pending_tx.sign([private_key])
```

### RPC Client

```python
from kaspa import RpcClient, Resolver

client = RpcClient(resolver=Resolver(), network_id="mainnet")
await client.connect()
info = await client.get_info()
```

## Type Annotations

The SDK includes comprehensive type stubs (`.pyi` files) for IDE support. All public APIs are fully typed.

## Error Handling

Most SDK functions raise `Exception` with descriptive messages on failure. Always wrap SDK calls in try/except blocks for production code:

```python
try:
    address = Address("invalid")
except Exception as e:
    print(f"Error: {e}")
```

## Async APIs

RPC methods are async and must be called with `await`:

```python
import asyncio

async def main():
    client = RpcClient(resolver=Resolver(), network_id="mainnet")
    await client.connect()
    result = await client.get_info()
    await client.disconnect()

asyncio.run(main())
```

