# Kaspa Python SDK

Welcome to the Kaspa Python SDK documentation. This library provides a Python SDK for interacting with the Kaspa blockchain, built from Rust source code, leveraging native [rusty-kaspa](https://github.com/kaspanet/rusty-kaspa) under the hood as much as possible.

This project very closely mirrors [Kaspa's WASM SDK](https://kaspa.aspectron.org/docs/), while trying to respect Python conventions. Feature partiy with WASM SDK is a work in progress, not all features are available yet in Python.

## Features

This SDK provides features across the following categories:

- **Address Management** - Create, validate, and manipulate Kaspa addresses
- **Key Management** - HD wallet key derivation, mnemonic phrases, and key generation
- **Transactions** - Construct, sign, broadcast and built scripts
- **RPC Client** - Connect to Kaspa nodes via WebSocket RPC

Most features gaps with WASM SDK exist around Wallet functionality.

## A (Very) Basic Example

```python
import asyncio
from kaspa import Resolver, RpcClient

async def main():
    client = RpcClient(resolver=Resolver())
    await client.connect()
    print(await client.get_server_info())

if __name__ == "__main__":
    asyncio.run(main())
```

## Getting Started

1. [Installation](getting-started/installation.md) - Set up the SDK in your environment
2. [Quickstart](getting-started/quickstart.md) - Build your first Kaspa application

## Documentation

<div class="grid cards" markdown>

- **[Addresses](guides/addresses.md)**  
  Create and validate Kaspa addresses

- **[Mnemonics](guides/mnemonics.md)**  
  Generate and use seed phrases

- **[Key Derivation](guides/key-derivation.md)**  
  HD wallet key generation

- **[Transactions](guides/transactions.md)**  
  Build and sign transactions

- **[Message Signing](guides/message-signing.md)**  
  Sign and verify messages

- **[RPC Client](guides/rpc-client.md)**  
  Connect to Kaspa nodes

</div>

## API Reference

For complete API documentation, see the [API Reference](api/index.md).

## License

This project is licensed under the ISC License.

