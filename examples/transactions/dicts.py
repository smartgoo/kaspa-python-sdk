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
        "transactionId": "368caa222bd878b987bf75d07f0bc6a6dbb866be754c421a73e33edd59552b75",
        "index": 5,
    }
    print("INPUT DICT:", input_dict)

    outpoint = TransactionOutpoint.from_dict(input_dict)
    output_dict = outpoint.to_dict()
    print("OUTPUT DICT:", output_dict)

    restored = TransactionOutpoint.from_dict(output_dict)
    print("ROUND-TRIP EQUAL:", outpoint == restored)
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
    print("INPUT DICT:", input_dict)

    output = TransactionOutput.from_dict(input_dict)
    output_dict = output.to_dict()
    print("OUTPUT DICT:", output_dict)

    restored = TransactionOutput.from_dict(output_dict)
    print("ROUND-TRIP EQUAL:", output == restored)
    print()


def transaction_input_example():
    """TransactionInput from_dict/to_dict example."""
    print("=" * 60)
    print("TransactionInput")
    print("=" * 60)

    input_dict = {
        "previousOutpoint": {
            "transactionId": "368caa222bd878b987bf75d07f0bc6a6dbb866be754c421a73e33edd59552b75",
            "index": 0,
        },
        "signatureScript": "deadbeef",
        "sequence": 0xFFFFFFFF,
        "sigOpCount": 1,
        "utxo": None,
    }
    print("INPUT DICT:", input_dict)

    tx_input = TransactionInput.from_dict(input_dict)
    output_dict = tx_input.to_dict()
    print("OUTPUT DICT:", output_dict)

    restored = TransactionInput.from_dict(output_dict)
    print("ROUND-TRIP EQUAL:", tx_input == restored)
    print()


def transaction_example():
    """Transaction from_dict/to_dict example."""
    print("=" * 60)
    print("Transaction")
    print("=" * 60)

    input_dict = {
        "id": "368caa222bd878b987bf75d07f0bc6a6dbb866be754c421a73e33edd59552b75",
        "version": 0,
        "inputs": [
            {
                "previousOutpoint": {
                    "transactionId": "368caa222bd878b987bf75d07f0bc6a6dbb866be754c421a73e33edd59552b75",
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
    print("INPUT DICT:", input_dict)

    tx = Transaction.from_dict(input_dict)
    output_dict = tx.to_dict()
    print("OUTPUT DICT:", output_dict)

    restored = Transaction.from_dict(output_dict)
    print("ROUND-TRIP EQUAL:", tx == restored)
    print()


def utxo_entry_example():
    """UtxoEntry from_dict/to_dict example."""
    print("=" * 60)
    print("UtxoEntry")
    print("=" * 60)

    input_dict = {
        "address": "kaspatest:qzy5xn4ue047muj3sz7g7a2gk5k5achh22kyhg64k7rj4afw8lp5kurchfh2n",
        "outpoint": {
            "transactionId": "368caa222bd878b987bf75d07f0bc6a6dbb866be754c421a73e33edd59552b75",
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
    print("INPUT DICT:", input_dict)

    entry = UtxoEntry.from_dict(input_dict)
    output_dict = entry.to_dict()
    print("OUTPUT DICT:", output_dict)

    restored = UtxoEntry.from_dict(output_dict)
    print("ROUND-TRIP EQUAL:", entry == restored)
    print()


def utxo_entry_reference_example():
    """UtxoEntryReference from_dict/to_dict example."""
    print("=" * 60)
    print("UtxoEntryReference (flat format)")
    print("=" * 60)

    # Flat format (same as UtxoEntry)
    input_dict = {
        "address": "kaspatest:qzy5xn4ue047muj3sz7g7a2gk5k5achh22kyhg64k7rj4afw8lp5kurchfh2n",
        "outpoint": {
            "transactionId": "368caa222bd878b987bf75d07f0bc6a6dbb866be754c421a73e33edd59552b75",
            "index": 2,
        },
        "amount": 2500000,
        "scriptPublicKey": {
            "version": 0,
            "script": "20852be1b87fca94453a35027c550a3ccdbebb5913106029f3a8bf18152bf93bffac",
        },
        "blockDaaScore": 67890,
        "isCoinbase": False,
    }
    print("INPUT DICT:", input_dict)

    entry_ref = UtxoEntryReference.from_dict(input_dict)
    output_dict = entry_ref.to_dict()
    print("OUTPUT DICT:", output_dict)

    restored = UtxoEntryReference.from_dict(output_dict)
    print("ROUND-TRIP EQUAL:", entry_ref == restored)
    print()

    # Also demonstrate nested format support (compatible with utxos returned via RPC)
    print("=" * 60)
    print("UtxoEntryReference (nested format - (utxos structure returned via RPC)")
    print("=" * 60)

    nested_dict = {
        "address": "kaspatest:qzy5xn4ue047muj3sz7g7a2gk5k5achh22kyhg64k7rj4afw8lp5kurchfh2n",
        "outpoint": {
            "transactionId": "368caa222bd878b987bf75d07f0bc6a6dbb866be754c421a73e33edd59552b75",
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
    print("INPUT DICT (nested):", nested_dict)

    entry_ref_nested = UtxoEntryReference.from_dict(nested_dict)
    print("OUTPUT DICT (flat):", entry_ref_nested.to_dict())
    print("Both formats produce equal objects:", entry_ref == entry_ref_nested)
    print()


if __name__ == "__main__":
    transaction_outpoint_example()
    transaction_output_example()
    transaction_input_example()
    transaction_example()
    utxo_entry_example()
    utxo_entry_reference_example()
