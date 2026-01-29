from kaspa import ScriptPublicKey, TransactionOutput

def transaction_output_roundtrip():
    spk = ScriptPublicKey(0, "2079b0fb70f7b5be66f6b448d765f6385132a599c3c516f60dd6069d1d5f29d217ac")
    original = TransactionOutput(1000000, spk)

    d = original.to_dict()
    print('Original: ', d)

    restored = TransactionOutput.from_dict(d)
    print('Restored: ', restored.to_dict())

    print(original == restored)

if __name__ == "__main__":
    transaction_output_roundtrip()