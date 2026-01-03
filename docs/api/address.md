# Address API

This page documents the address-related classes and functions.

## Address

::: kaspa.Address
    options:
      members:
        - __init__
        - validate
        - to_string
        - version
        - prefix
        - payload
        - short

### Examples

```python
from kaspa import Address

# Create from string
address = Address("kaspa:qz0s9f5...")

# Validate an address
is_valid = Address.validate("kaspa:qz...")

# Get components
print(address.prefix)   # "kaspa"
print(address.version)  # "PubKey"

# Convert to string
addr_str = address.to_string()

# Change network prefix
address.prefix = "kaspatest"
```

---

## ScriptPublicKey

Represents a script public key (locking script).

::: kaspa.ScriptPublicKey
    options:
      members:
        - __init__
        - script

### Examples

```python
from kaspa import ScriptPublicKey

# Create from version and script bytes
spk = ScriptPublicKey(0, "20a1b2c3d4...")

# Access script
print(spk.script)
```

---

## Functions

### address_from_script_public_key

Convert a script public key to an address.

::: kaspa.address_from_script_public_key

```python
from kaspa import ScriptPublicKey, address_from_script_public_key, NetworkType

spk = ScriptPublicKey(0, "20...")
address = address_from_script_public_key(spk, NetworkType.Mainnet)
```

---

### pay_to_address_script

Get the locking script for an address.

::: kaspa.pay_to_address_script

```python
from kaspa import Address, pay_to_address_script

address = Address("kaspa:qz...")
script = pay_to_address_script(address)
```

---

### pay_to_script_hash_script

Create a P2SH locking script.

::: kaspa.pay_to_script_hash_script

---

### pay_to_script_hash_signature_script

Create a P2SH signature script.

::: kaspa.pay_to_script_hash_signature_script

---

### is_script_pay_to_pubkey

Check if a script is pay-to-pubkey (Schnorr).

::: kaspa.is_script_pay_to_pubkey

---

### is_script_pay_to_pubkey_ecdsa

Check if a script is pay-to-pubkey (ECDSA).

::: kaspa.is_script_pay_to_pubkey_ecdsa

---

### is_script_pay_to_script_hash

Check if a script is pay-to-script-hash.

::: kaspa.is_script_pay_to_script_hash

---

### create_multisig_address

Create a multi-signature address.

::: kaspa.create_multisig_address

```python
from kaspa import create_multisig_address, PublicKey, NetworkType

pubkeys = [PublicKey(k) for k in ["02key1...", "02key2...", "02key3..."]]

# 2-of-3 multisig
address = create_multisig_address(
    minimum_signatures=2,
    keys=pubkeys,
    network_type=NetworkType.Mainnet
)
```

