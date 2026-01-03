# Keys API

This page documents key management classes and functions.

## Mnemonic

BIP-39 mnemonic seed phrase.

::: kaspa.Mnemonic
    options:
      members:
        - __init__
        - validate
        - random
        - phrase
        - entropy
        - to_seed

### Examples

```python
from kaspa import Mnemonic, Language

# Generate random mnemonic
mnemonic = Mnemonic.random()
mnemonic = Mnemonic.random(word_count=24)

# Restore from phrase
mnemonic = Mnemonic("word1 word2 ...")

# Validate
is_valid = Mnemonic.validate("word1 word2 ...")

# Get phrase
print(mnemonic.phrase)

# Convert to seed
seed = mnemonic.to_seed()
seed_with_password = mnemonic.to_seed("passphrase")
```

---

## Language

Supported mnemonic languages.

::: kaspa.Language

```python
from kaspa import Language

lang = Language.English
```

---

## PrivateKey

A private key for signing.

::: kaspa.PrivateKey
    options:
      members:
        - __init__
        - to_string
        - to_public_key
        - to_address
        - to_address_ecdsa
        - to_keypair

### Examples

```python
from kaspa import PrivateKey, NetworkType

# Create from hex
pk = PrivateKey("a1b2c3d4...")

# Derive public key
public_key = pk.to_public_key()

# Derive address
address = pk.to_address(NetworkType.Mainnet)
ecdsa_address = pk.to_address_ecdsa(NetworkType.Mainnet)

# Get keypair
keypair = pk.to_keypair()
```

---

## PublicKey

A public key.

::: kaspa.PublicKey
    options:
      members:
        - __init__
        - to_string
        - to_address
        - to_address_ecdsa
        - to_x_only_public_key

### Examples

```python
from kaspa import PublicKey, NetworkType

# Create from hex
pub = PublicKey("02a1b2c3...")

# Derive address
address = pub.to_address(NetworkType.Mainnet)

# Get x-only key
xonly = pub.to_x_only_public_key()
```

---

## XOnlyPublicKey

An x-only public key (for Schnorr signatures).

::: kaspa.XOnlyPublicKey
    options:
      members:
        - __init__
        - to_string
        - to_address
        - to_address_ecdsa
        - from_address

---

## Keypair

A key pair containing both private and public keys.

::: kaspa.Keypair
    options:
      members:
        - __init__
        - random
        - from_private_key
        - private_key
        - public_key
        - xonly_public_key
        - to_address
        - to_address_ecdsa

### Examples

```python
from kaspa import Keypair, NetworkType

# Generate random keypair
kp = Keypair.random()

# From existing private key
kp = Keypair.from_private_key(private_key)

# Access keys
print(kp.private_key)
print(kp.public_key)
print(kp.xonly_public_key)

# Derive address
address = kp.to_address(NetworkType.Mainnet)
```

---

## XPrv

Extended private key (BIP-32).

::: kaspa.XPrv
    options:
      members:
        - __init__
        - from_xprv
        - derive_child
        - derive_path
        - to_xpub
        - to_private_key
        - to_string
        - into_string
        - xprv
        - private_key
        - depth
        - parent_fingerprint
        - child_number
        - chain_code

### Examples

```python
from kaspa import XPrv, Mnemonic

# From seed
mnemonic = Mnemonic.random()
xprv = XPrv(mnemonic.to_seed())

# From existing xprv string
xprv = XPrv.from_xprv("xprv...")

# Derive children
child = xprv.derive_child(0)
hardened = xprv.derive_child(0, hardened=True)

# Derive by path
account = xprv.derive_path("m/44'/111111'/0'")

# Get extended public key
xpub = xprv.to_xpub()

# Access properties
print(xprv.depth)
print(xprv.chain_code)
```

---

## XPub

Extended public key (BIP-32).

::: kaspa.XPub
    options:
      members:
        - __init__
        - derive_child
        - derive_path
        - to_public_key
        - to_str
        - xpub
        - depth
        - parent_fingerprint
        - child_number
        - chain_code

---

## DerivationPath

BIP-32 derivation path.

::: kaspa.DerivationPath
    options:
      members:
        - __init__
        - is_empty
        - length
        - parent
        - push
        - to_string

### Examples

```python
from kaspa import DerivationPath

path = DerivationPath("m/44'/111111'/0'/0/0")

print(path.length())      # 5
print(path.to_string())   # "m/44'/111111'/0'/0/0"

parent = path.parent()
path.push(1)              # Append non-hardened child
path.push(0, hardened=True)  # Append hardened child
```

---

## PrivateKeyGenerator

Generator for deriving private keys.

::: kaspa.PrivateKeyGenerator
    options:
      members:
        - __init__
        - receive_key
        - change_key

### Examples

```python
from kaspa import PrivateKeyGenerator, XPrv, NetworkType

xprv = XPrv(seed)
gen = PrivateKeyGenerator(xprv.to_string(), False, 0)

# Get receive keys
for i in range(5):
    pk = gen.receive_key(i)
    addr = pk.to_address(NetworkType.Mainnet)
    print(f"Receive {i}: {addr.to_string()}")

# Get change key
change_pk = gen.change_key(0)
```

---

## PublicKeyGenerator

Generator for deriving public keys and addresses (watch-only).

::: kaspa.PublicKeyGenerator
    options:
      members:
        - from_xpub
        - from_master_xprv
        - receive_pubkeys
        - receive_pubkey
        - receive_pubkeys_as_strings
        - receive_pubkey_as_string
        - receive_addresses
        - receive_address
        - receive_addresses_as_strings
        - receive_address_as_string
        - change_pubkeys
        - change_pubkey
        - change_pubkeys_as_strings
        - change_pubkey_as_string
        - change_addresses
        - change_address
        - change_addresses_as_strings
        - change_address_as_string
        - to_string

### Examples

```python
from kaspa import PublicKeyGenerator, NetworkType

# From xpub
gen = PublicKeyGenerator.from_xpub("kpub...")

# From master xprv
gen = PublicKeyGenerator.from_master_xprv(xprv_string, False, 0)

# Get addresses
addrs = gen.receive_addresses(NetworkType.Mainnet, 0, 10)

# Get single address
addr = gen.receive_address(NetworkType.Mainnet, 0)

# Get public keys
pubkeys = gen.receive_pubkeys(0, 5)
```

---

## AccountKind

Account type identifier.

::: kaspa.AccountKind
    options:
      members:
        - __init__
        - to_string

---

## Message Signing Functions

### sign_message

Sign an arbitrary message.

::: kaspa.sign_message

```python
from kaspa import sign_message, PrivateKey

sig = sign_message("Hello", private_key)
sig_deterministic = sign_message("Hello", private_key, no_aux_rand=True)
```

### verify_message

Verify a message signature.

::: kaspa.verify_message

```python
from kaspa import verify_message, PublicKey

is_valid = verify_message("Hello", signature, public_key)
```

