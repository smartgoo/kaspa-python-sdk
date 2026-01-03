# Utilities API

This page documents utility classes and functions.

## Network Types

### NetworkId

Network identifier with optional suffix.

::: kaspa.NetworkId
    options:
      members:
        - __init__
        - with_suffix
        - network_type
        - is_mainnet
        - suffix
        - default_p2p_port
        - to_prefixed
        - to_string
        - address_prefix

### Examples

```python
from kaspa import NetworkId, NetworkType

# From string
network = NetworkId("mainnet")
network = NetworkId("testnet-10")

# From NetworkType
network = NetworkId(NetworkType.Mainnet)

# With suffix
network = NetworkId.with_suffix(NetworkType.Testnet, 10)

# Properties
print(network.network_type)      # NetworkType.Mainnet
print(network.is_mainnet())      # True
print(network.suffix)            # None or int
print(network.default_p2p_port)  # 16111
print(network.address_prefix())  # "kaspa"
```

---

### NetworkType

Network type enumeration.

::: kaspa.NetworkType

| Value | Description |
|-------|-------------|
| `Mainnet` | Production network |
| `Testnet` | Test network |
| `Devnet` | Development network |
| `Simnet` | Simulation network |

```python
from kaspa import NetworkType

network = NetworkType.Mainnet
network = NetworkType.Testnet
```

---

## Script Building

### ScriptBuilder

Builder for transaction scripts.

::: kaspa.ScriptBuilder
    options:
      members:
        - __init__
        - from_script
        - add_op
        - add_ops
        - add_data
        - add_i64
        - add_lock_time
        - add_sequence
        - canonical_data_size
        - to_string
        - drain
        - create_pay_to_script_hash_script
        - encode_pay_to_script_hash_signature_script

### Examples

```python
from kaspa import ScriptBuilder, Opcodes

# Create new script
builder = ScriptBuilder()

# Add opcodes
builder.add_op(Opcodes.Op2)
builder.add_op(Opcodes.OpCheckMultiSig)

# Add data
builder.add_data("a1b2c3...")

# Chain operations
script = (
    ScriptBuilder()
    .add_op(Opcodes.OpDup)
    .add_op(Opcodes.OpBlake2b)
    .add_data(pubkey_hash)
    .add_op(Opcodes.OpEqualVerify)
    .add_op(Opcodes.OpCheckSig)
)

# Get script as string
script_hex = builder.drain()

# Create P2SH script
p2sh_script = builder.create_pay_to_script_hash_script()

# From existing script
builder = ScriptBuilder.from_script("76a914...")
```

---

### Opcodes

Script opcodes enumeration.

::: kaspa.Opcodes

#### Common Opcodes

| Opcode | Value | Description |
|--------|-------|-------------|
| `OpFalse` | 0x00 | Push empty array |
| `OpTrue` / `Op1` | 0x51 | Push 1 |
| `Op2` - `Op16` | 0x52-0x60 | Push 2-16 |
| `OpDup` | 0x76 | Duplicate top stack item |
| `OpEqual` | 0x87 | Check equality |
| `OpEqualVerify` | 0x88 | Equal + verify |
| `OpCheckSig` | 0xac | Verify Schnorr signature |
| `OpCheckSigVerify` | 0xad | CheckSig + verify |
| `OpCheckMultiSig` | 0xae | Verify multisig |
| `OpCheckSigECDSA` | 0xab | Verify ECDSA signature |
| `OpBlake2b` | 0xaa | Blake2b hash |
| `OpSHA256` | 0xa8 | SHA256 hash |
| `OpReturn` | 0x6a | Mark unspendable |
| `OpIf` | 0x63 | Conditional |
| `OpElse` | 0x67 | Else branch |
| `OpEndIf` | 0x68 | End conditional |
| `OpCheckLockTimeVerify` | 0xb0 | Time lock |
| `OpCheckSequenceVerify` | 0xb1 | Relative time lock |

```python
from kaspa import Opcodes

op = Opcodes.OpCheckSig
print(op.value)  # 172 (0xac)
```

---

## Hashing

### Hash

A 32-byte hash value.

::: kaspa.Hash
    options:
      members:
        - __init__
        - to_string

```python
from kaspa import Hash

h = Hash("a1b2c3d4...")
print(h.to_string())
```

---

## Unit Conversion

### kaspa_to_sompi

Convert KAS to sompi.

::: kaspa.kaspa_to_sompi

```python
from kaspa import kaspa_to_sompi

sompi = kaspa_to_sompi(1.5)  # 150,000,000
sompi = kaspa_to_sompi(0.00000001)  # 1
```

### sompi_to_kaspa

Convert sompi to KAS.

::: kaspa.sompi_to_kaspa

```python
from kaspa import sompi_to_kaspa

kas = sompi_to_kaspa(150000000)  # 1.5
kas = sompi_to_kaspa(1)  # 0.00000001
```

### sompi_to_kaspa_string_with_suffix

Format sompi as a string with network suffix.

::: kaspa.sompi_to_kaspa_string_with_suffix

```python
from kaspa import sompi_to_kaspa_string_with_suffix, NetworkType

formatted = sompi_to_kaspa_string_with_suffix(150000000, NetworkType.Mainnet)
# "1.5 KAS"

formatted = sompi_to_kaspa_string_with_suffix(150000000, "testnet")
# "1.5 TKAS"
```

---

## Constants

### Unit Conversion

| Constant | Value |
|----------|-------|
| 1 KAS | 100,000,000 sompi |
| 1 sompi | 0.00000001 KAS |

### Network Ports

| Network | P2P Port |
|---------|----------|
| Mainnet | 16111 |
| Testnet | 16211 |

---

## Type Aliases

The SDK uses these type patterns:

```python
# Script data can be provided as:
ScriptData = Union[str, bytes, list[int]]

# Network can be specified as:
NetworkParam = Union[str, NetworkType, NetworkId]

# Sighash can be specified as:
SighashParam = Union[str, SighashType]
```

