# Transactions API

This page documents transaction-related classes and functions.

## Transaction

A Kaspa transaction.

::: kaspa.Transaction
    options:
      members:
        - __init__
        - is_coinbase
        - finalize
        - id
        - inputs
        - outputs
        - version
        - lock_time
        - gas
        - subnetwork_id
        - payload
        - mass
        - addresses
        - serialize_to_dict

### Examples

```python
from kaspa import Transaction, TransactionInput, TransactionOutput

tx = Transaction(
    version=0,
    inputs=inputs,
    outputs=outputs,
    lock_time=0,
    subnetwork_id="0" * 40,
    gas=0,
    payload="",
    mass=0
)

print(tx.id)
print(tx.is_coinbase())
```

---

## TransactionInput

A transaction input.

::: kaspa.TransactionInput
    options:
      members:
        - __init__
        - previous_outpoint
        - signature_script
        - sequence
        - sig_op_count
        - utxo

---

## TransactionOutput

A transaction output.

::: kaspa.TransactionOutput
    options:
      members:
        - __init__
        - value
        - script_public_key

---

## TransactionOutpoint

Reference to a previous transaction output.

::: kaspa.TransactionOutpoint
    options:
      members:
        - __init__
        - get_id
        - transaction_id
        - index

---

## UtxoEntry

An unspent transaction output.

::: kaspa.UtxoEntry
    options:
      members:
        - address
        - outpoint
        - amount
        - script_public_key
        - block_daa_score
        - is_coinbase

---

## UtxoEntryReference

A reference to a UTXO entry.

::: kaspa.UtxoEntryReference
    options:
      members:
        - entry
        - outpoint
        - address
        - amount
        - is_coinbase
        - block_daa_score
        - script_public_key

---

## UtxoEntries

A collection of UTXO entries.

::: kaspa.UtxoEntries
    options:
      members:
        - items
        - sort
        - amount

---

## Generator

Transaction generator for building and signing transactions.

::: kaspa.Generator
    options:
      members:
        - __init__
        - estimate
        - summary
        - __iter__
        - __next__

### Examples

```python
from kaspa import Generator, PaymentOutput, NetworkId

generator = Generator(
    network_id=NetworkId("mainnet"),
    entries=utxos,
    change_address=my_address,
    outputs=[PaymentOutput(recipient, amount)],
    priority_fee=1000,
)

# Estimate before sending
summary = generator.estimate()
print(f"Fee: {summary.fees}")

# Generate and sign
for pending in generator:
    pending.sign([private_key])
    tx_id = await pending.submit(client)
```

---

## PendingTransaction

A transaction ready for signing and submission.

::: kaspa.PendingTransaction
    options:
      members:
        - id
        - payment_amount
        - change_amount
        - fee_amount
        - mass
        - minimum_signatures
        - aggregate_input_amount
        - aggregate_output_amount
        - transaction_type
        - addresses
        - get_utxo_entries
        - create_input_signature
        - fill_input
        - sign_input
        - sign
        - submit
        - transaction

### Examples

```python
for pending in generator:
    # Properties
    print(f"ID: {pending.id}")
    print(f"Fee: {pending.fee_amount}")
    
    # Sign all inputs
    pending.sign([private_key])
    
    # Or sign individually
    # pending.sign_input(0, private_key)
    
    # Submit
    tx_id = await pending.submit(rpc_client)
```

---

## GeneratorSummary

Summary of generated transactions.

::: kaspa.GeneratorSummary
    options:
      members:
        - network_type
        - utxos
        - fees
        - transactions
        - final_amount
        - final_transaction_id

---

## PaymentOutput

A payment destination.

::: kaspa.PaymentOutput
    options:
      members:
        - __init__

```python
from kaspa import PaymentOutput, Address

payment = PaymentOutput(
    address=Address("kaspa:..."),
    amount=100_000_000  # 1 KAS in sompi
)
```

---

## SighashType

Signature hash types.

::: kaspa.SighashType

| Value | Description |
|-------|-------------|
| `All` | Sign all inputs and outputs |
| `None` | Sign all inputs, no outputs |
| `Single` | Sign all inputs, one output |
| `AllAnyOneCanPay` | Sign one input, all outputs |
| `NoneAnyOneCanPay` | Sign one input, no outputs |
| `SingleAnyOneCanPay` | Sign one input, one output |

---

## Signing Functions

### sign_transaction

Sign a transaction with private keys.

::: kaspa.sign_transaction

```python
from kaspa import sign_transaction

signed_tx = sign_transaction(tx, [private_key], verify_sig=True)
```

### create_input_signature

Create a signature for a specific input.

::: kaspa.create_input_signature

### sign_script_hash

Sign a script hash.

::: kaspa.sign_script_hash

---

## Transaction Building Functions

### create_transaction

Create a single transaction.

::: kaspa.create_transaction

### create_transactions

Create multiple transactions with automatic UTXO management.

::: kaspa.create_transactions

### estimate_transactions

Estimate transaction fees without creating.

::: kaspa.estimate_transactions

---

## Mass and Fee Functions

### maximum_standard_transaction_mass

Get the maximum allowed transaction mass.

::: kaspa.maximum_standard_transaction_mass

### calculate_transaction_fee

Calculate the fee for a transaction.

::: kaspa.calculate_transaction_fee

### calculate_transaction_mass

Calculate the mass of a transaction.

::: kaspa.calculate_transaction_mass

### calculate_storage_mass

Calculate the storage mass component.

::: kaspa.calculate_storage_mass

### update_transaction_mass

Update a transaction's mass field.

::: kaspa.update_transaction_mass

