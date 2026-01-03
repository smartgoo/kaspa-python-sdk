# RPC API

This page documents the RPC client classes and methods.

## RpcClient

WebSocket RPC client for Kaspa nodes.

### Constructor

```python
RpcClient(
    resolver: Optional[Resolver] = None,
    url: Optional[str] = None,
    encoding: Optional[str] = "borsh",
    network_id: Optional[Union[str, NetworkId]] = "mainnet"
)
```

**Parameters:**

- `resolver` - Resolver for automatic node discovery
- `url` - Direct WebSocket URL (alternative to resolver)
- `encoding` - Message encoding: `"borsh"` (default) or `"json"`
- `network_id` - Network to connect to

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `url` | `str` | Current connection URL |
| `resolver` | `Optional[Resolver]` | The resolver instance |
| `is_connected` | `bool` | Connection status |
| `encoding` | `str` | Message encoding |
| `node_id` | `str` | Connected node ID |

### Connection Methods

#### connect

```python
async def connect(
    block_async_connect: Optional[bool] = None,
    strategy: Optional[str] = None,
    url: Optional[str] = None,
    timeout_duration: Optional[int] = None,
    retry_interval: Optional[int] = None
) -> None
```

Connect to a Kaspa node.

#### disconnect

```python
async def disconnect() -> None
```

Disconnect from the node.

#### start

```python
async def start() -> None
```

Start the RPC client.

---

### Network Information Methods

#### get_info

```python
async def get_info() -> dict
```

Get general node information.

#### get_block_count

```python
async def get_block_count() -> dict
```

Get block and header counts.

**Returns:** `{"blockCount": int, "headerCount": int}`

#### get_block_dag_info

```python
async def get_block_dag_info() -> dict
```

Get block DAG information.

#### get_coin_supply

```python
async def get_coin_supply() -> dict
```

Get circulating and max coin supply.

#### get_current_network

```python
async def get_current_network() -> dict
```

Get the current network name.

#### get_sync_status

```python
async def get_sync_status() -> dict
```

Get synchronization status.

**Returns:** `{"isSynced": bool}`

#### get_server_info

```python
async def get_server_info() -> dict
```

Get server version and capabilities.

#### get_system_info

```python
async def get_system_info() -> dict
```

Get system resource information.

---

### Balance and UTXO Methods

#### get_balance_by_address

```python
async def get_balance_by_address(request: dict) -> dict
```

Get balance for a single address.

**Request:** `{"address": str}`

**Returns:** `{"balance": int}` (in sompi)

#### get_balances_by_addresses

```python
async def get_balances_by_addresses(request: dict) -> dict
```

Get balances for multiple addresses.

**Request:** `{"addresses": list[str]}`

#### get_utxos_by_addresses

```python
async def get_utxos_by_addresses(request: dict) -> dict
```

Get UTXOs for addresses.

**Request:** `{"addresses": list[str]}`

---

### Block Methods

#### get_block

```python
async def get_block(request: dict) -> dict
```

Get a specific block.

**Request:**
```python
{
    "hash": str,              # Block hash
    "includeTransactions": bool
}
```

#### get_blocks

```python
async def get_blocks(request: dict) -> dict
```

Get multiple blocks.

**Request:**
```python
{
    "lowHash": Optional[str],
    "includeBlocks": bool,
    "includeTransactions": bool
}
```

#### get_block_template

```python
async def get_block_template(request: dict) -> dict
```

Get a block template for mining.

**Request:**
```python
{
    "payAddress": str,        # Coinbase payment address
    "extraData": list[int]    # Extra data bytes
}
```

#### get_headers

```python
async def get_headers(request: dict) -> dict
```

Get block headers.

**Request:**
```python
{
    "startHash": str,
    "limit": int,
    "isAscending": bool
}
```

---

### Transaction Methods

#### submit_transaction

```python
async def submit_transaction(request: dict) -> dict
```

Submit a transaction.

**Request:**
```python
{
    "transaction": dict,      # Serialized transaction
    "allowOrphan": bool
}
```

**Returns:** `{"transactionId": str}`

#### submit_transaction_replacement

```python
async def submit_transaction_replacement(request: dict) -> dict
```

Submit a replacement transaction.

**Request:** `{"transaction": dict}`

---

### Mempool Methods

#### get_mempool_entries

```python
async def get_mempool_entries(request: dict) -> dict
```

Get mempool entries.

**Request:**
```python
{
    "includeOrphanPool": bool,
    "filterTransactionPool": bool
}
```

#### get_mempool_entries_by_addresses

```python
async def get_mempool_entries_by_addresses(request: dict) -> dict
```

Get mempool entries for specific addresses.

**Request:**
```python
{
    "addresses": list[str],
    "includeOrphanPool": bool,
    "filterTransactionPool": bool
}
```

#### get_mempool_entry

```python
async def get_mempool_entry(request: dict) -> dict
```

Get a specific mempool entry.

**Request:**
```python
{
    "transactionId": str,
    "includeOrphanPool": bool,
    "filterTransactionPool": bool
}
```

---

### Fee Methods

#### get_fee_estimate

```python
async def get_fee_estimate() -> dict
```

Get fee estimation.

#### get_fee_estimate_experimental

```python
async def get_fee_estimate_experimental(request: dict) -> dict
```

Get detailed fee estimation.

**Request:** `{"verbose": bool}`

---

### Chain Methods

#### get_virtual_chain_from_block

```python
async def get_virtual_chain_from_block(request: dict) -> dict
```

Get virtual chain from a block.

**Request:**
```python
{
    "startHash": str,
    "includeAcceptedTransactionIds": bool
}
```

#### get_sink

```python
async def get_sink() -> dict
```

Get the current DAG sink.

#### get_sink_blue_score

```python
async def get_sink_blue_score() -> dict
```

Get the sink's blue score.

#### get_current_block_color

```python
async def get_current_block_color(request: dict) -> dict
```

Get block color (blue/red).

**Request:** `{"hash": str}`

#### get_daa_score_timestamp_estimate

```python
async def get_daa_score_timestamp_estimate(request: dict) -> dict
```

Estimate timestamps for DAA scores.

**Request:** `{"daaScores": list[int]}`

---

### Peer Methods

#### get_connected_peer_info

```python
async def get_connected_peer_info() -> dict
```

Get connected peer information.

#### get_peer_addresses

```python
async def get_peer_addresses() -> dict
```

Get known peer addresses.

#### get_connections

```python
async def get_connections(request: dict) -> dict
```

Get connection details.

**Request:** `{"includeProfileData": bool}`

#### add_peer

```python
async def add_peer(request: dict) -> dict
```

Add a peer.

**Request:**
```python
{
    "peerAddress": str,       # "ip:port"
    "isPermanent": bool
}
```

#### ban

```python
async def ban(request: dict) -> dict
```

Ban an IP address.

**Request:** `{"ip": str}`

#### unban

```python
async def unban(request: dict) -> dict
```

Unban an IP address.

**Request:** `{"ip": str}`

---

### Metrics Methods

#### get_metrics

```python
async def get_metrics(request: dict) -> dict
```

Get node metrics.

**Request:**
```python
{
    "processMetrics": bool,
    "connectionMetrics": bool,
    "bandwidthMetrics": bool,
    "consensusMetrics": bool,
    "storageMetrics": bool,
    "customMetrics": bool
}
```

#### estimate_network_hashes_per_second

```python
async def estimate_network_hashes_per_second(request: dict) -> dict
```

Estimate network hash rate.

**Request:**
```python
{
    "windowSize": int,
    "startHash": Optional[str]
}
```

---

### Other Methods

#### ping

```python
async def ping() -> dict
```

Ping the node.

#### shutdown

```python
async def shutdown() -> dict
```

Request node shutdown.

#### get_subnetwork

```python
async def get_subnetwork(request: dict) -> dict
```

Get subnetwork information.

**Request:** `{"subnetworkId": str}`

#### resolve_finality_conflict

```python
async def resolve_finality_conflict(request: dict) -> dict
```

Resolve a finality conflict.

**Request:** `{"finalityBlockHash": str}`

#### submit_block

```python
async def submit_block(request: dict) -> dict
```

Submit a mined block.

**Request:**
```python
{
    "block": dict,
    "allowNonDaaBlocks": bool
}
```

---

### Event Subscription Methods

#### add_event_listener

```python
def add_event_listener(
    event: str,
    callback: Callable[..., Any],
    *args: Any,
    **kwargs: Optional[Any]
) -> None
```

Add an event listener.

#### remove_event_listener

```python
def remove_event_listener(
    event: str,
    callback: Callable[..., Any] = None
) -> None
```

Remove an event listener.

#### remove_all_event_listeners

```python
def remove_all_event_listeners() -> None
```

Remove all event listeners.

---

### Subscription Methods

| Method | Event |
|--------|-------|
| `subscribe_utxos_changed(addresses)` | UTXO changes |
| `unsubscribe_utxos_changed(addresses)` | |
| `subscribe_block_added()` | New blocks |
| `unsubscribe_block_added()` | |
| `subscribe_virtual_chain_changed(include_accepted_transaction_ids)` | Chain changes |
| `unsubscribe_virtual_chain_changed(...)` | |
| `subscribe_virtual_daa_score_changed()` | DAA score changes |
| `unsubscribe_virtual_daa_score_changed()` | |
| `subscribe_sink_blue_score_changed()` | Blue score changes |
| `unsubscribe_sink_blue_score_changed()` | |
| `subscribe_finality_conflict()` | Finality conflicts |
| `unsubscribe_finality_conflict()` | |
| `subscribe_finality_conflict_resolved()` | Conflict resolution |
| `unsubscribe_finality_conflict_resolved()` | |
| `subscribe_new_block_template()` | New templates |
| `unsubscribe_new_block_template()` | |
| `subscribe_pruning_point_utxo_set_override()` | Pruning events |
| `unsubscribe_pruning_point_utxo_set_override()` | |

---

## Resolver

Node resolver for automatic discovery.

### Constructor

```python
Resolver(
    urls: Optional[list[str]] = None,
    tls: Optional[int] = None
)
```

### Methods

#### urls

```python
def urls() -> list[str]
```

Get resolver URLs.

#### get_node

```python
async def get_node(encoding: str, network_id: Union[str, NetworkId]) -> dict
```

Get node information.

#### get_url

```python
async def get_url(encoding: str, network_id: Union[str, NetworkId]) -> str
```

Get a node URL.

### Examples

```python
from kaspa import Resolver

# Default resolver
resolver = Resolver()

# Custom URLs
resolver = Resolver(urls=["https://resolver.kaspa.org"])

# Get available URLs
print(resolver.urls())

# Get a node URL
url = await resolver.get_url("borsh", "mainnet")
```

