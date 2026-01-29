"""
Examples demonstrating to_dict/from_dict conversion for consensus client types.

Each function shows:
1. Creating an object from a dictionary using from_dict()
2. Converting it back to a dictionary using to_dict()
3. Verifying the round-trip produces equal objects
"""

from kaspa import (
    Transaction,
    TransactionInput,
    TransactionOutput,
    TransactionOutpoint,
    UtxoEntry,
    UtxoEntryReference,
)


def transaction_outpoint_example():
    """TransactionOutpoint from_dict/to_dict example."""
    print("=" * 60)
    print("TransactionOutpoint")
    print("=" * 60)

    input_dict = {
        "transactionId": "a" * 64,
        "index": 5,
    }
    print("Input dict:", input_dict)

    outpoint = TransactionOutpoint.from_dict(input_dict)
    output_dict = outpoint.to_dict()
    print("Output dict:", output_dict)

    restored = TransactionOutpoint.from_dict(output_dict)
    print("Round-trip equal:", outpoint == restored)
    print()


def transaction_output_example():
    """TransactionOutput from_dict/to_dict example."""
    print("=" * 60)
    print("TransactionOutput")
    print("=" * 60)

    input_dict = {
        "value": 1000000,
        "scriptPublicKey": {
            "version": 0,
            "script": "2079b0fb70f7b5be66f6b448d765f6385132a599c3c516f60dd6069d1d5f29d217ac",
        },
    }
    print("Input dict:", input_dict)

    output = TransactionOutput.from_dict(input_dict)
    output_dict = output.to_dict()
    print("Output dict:", output_dict)

    restored = TransactionOutput.from_dict(output_dict)
    print("Round-trip equal:", output == restored)
    print()


def transaction_input_example():
    """TransactionInput from_dict/to_dict example."""
    print("=" * 60)
    print("TransactionInput")
    print("=" * 60)

    input_dict = {
        "previousOutpoint": {
            "transactionId": "b" * 64,
            "index": 0,
        },
        "signatureScript": "deadbeef",
        "sequence": 0xFFFFFFFF,
        "sigOpCount": 1,
        "utxo": None,
    }
    print("Input dict:", input_dict)

    tx_input = TransactionInput.from_dict(input_dict)
    output_dict = tx_input.to_dict()
    print("Output dict:", output_dict)

    restored = TransactionInput.from_dict(output_dict)
    print("Round-trip equal:", tx_input == restored)
    print()


def transaction_example():
    """Transaction from_dict/to_dict example."""
    print("=" * 60)
    print("Transaction")
    print("=" * 60)

    input_dict = {
        "id": "c" * 64,
        "version": 0,
        "inputs": [
            {
                "previousOutpoint": {
                    "transactionId": "d" * 64,
                    "index": 0,
                },
                "signatureScript": "",
                "sequence": 0,
                "sigOpCount": 1,
                "utxo": None,
            }
        ],
        "outputs": [
            {
                "value": 500000,
                "scriptPublicKey": {
                    "version": 0,
                    "script": "2079b0fb70f7b5be66f6b448d765f6385132a599c3c516f60dd6069d1d5f29d217ac",
                },
            }
        ],
        "lockTime": 0,
        "subnetworkId": "0" * 40,
        "gas": 0,
        "payload": "",
        "mass": 0,
    }
    print("Input dict:", input_dict)

    tx = Transaction.from_dict(input_dict)
    output_dict = tx.to_dict()
    print("Output dict:", output_dict)

    restored = Transaction.from_dict(output_dict)
    print("Round-trip equal:", tx == restored)
    print()


def utxo_entry_example():
    """UtxoEntry from_dict/to_dict example."""
    print("=" * 60)
    print("UtxoEntry")
    print("=" * 60)

    input_dict = {
        "address": "kaspa:qr0lr4ml9fn3chekrqmjdkergxl93l4wrk3dankcgvjq776s9wn9jkdskewva",
        "outpoint": {
            "transactionId": "e" * 64,
            "index": 0,
        },
        "amount": 1000000,
        "scriptPublicKey": {
            "version": 0,
            "script": "20852be1b87fca94453a35027c550a3ccdbebb5913106029f3a8bf18152bf93bffac",
        },
        "blockDaaScore": 12345,
        "isCoinbase": False,
    }
    print("Input dict:", input_dict)

    entry = UtxoEntry.from_dict(input_dict)
    output_dict = entry.to_dict()
    print("Output dict:", output_dict)

    restored = UtxoEntry.from_dict(output_dict)
    print("Round-trip equal:", entry == restored)
    print()


def utxo_entry_reference_example():
    """UtxoEntryReference from_dict/to_dict example."""
    print("=" * 60)
    print("UtxoEntryReference")
    print("=" * 60)

    input_dict = {
        "address": "kaspa:qr0lr4ml9fn3chekrqmjdkergxl93l4wrk3dankcgvjq776s9wn9jkdskewva",
        "outpoint": {
            "transactionId": "f" * 64,
            "index": 2,
        },
        "utxoEntry": {
            "amount": 2500000,
            "scriptPublicKey": {
                "version": 0,
                "script": "20852be1b87fca94453a35027c550a3ccdbebb5913106029f3a8bf18152bf93bffac",
            },
            "blockDaaScore": 67890,
            "isCoinbase": False,
        },
    }
    print("Input dict:", input_dict)

    entry_ref = UtxoEntryReference.from_dict(input_dict)
    output_dict = entry_ref.to_dict()
    print("Output dict:", output_dict)

    restored = UtxoEntryReference.from_dict(output_dict)
    print("Round-trip equal:", entry_ref == restored)
    print()


if __name__ == "__main__":
    transaction_outpoint_example()
    transaction_output_example()
    transaction_input_example()
    transaction_example()
    utxo_entry_example()
    utxo_entry_reference_example()

    print("All examples completed successfully!")

