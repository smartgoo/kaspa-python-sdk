mod address;
mod consensus;
mod crypto;
mod network;
mod rpc;
mod types;
mod wallet;

use pyo3::prelude::*;

#[pymodule]
fn kaspa(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<address::PyAddress>()?;

    m.add_class::<consensus::client::transaction::PyTransaction>()?;
    m.add_class::<consensus::client::input::PyTransactionInput>()?;
    m.add_class::<consensus::client::outpoint::PyTransactionOutpoint>()?;
    m.add_class::<consensus::client::output::PyTransactionOutput>()?;
    m.add_class::<consensus::client::utxo::PyUtxoEntry>()?;
    m.add_class::<consensus::client::utxo::PyUtxoEntries>()?;
    m.add_class::<consensus::client::utxo::PyUtxoEntryReference>()?;

    m.add_function(wrap_pyfunction!(
        consensus::client::utils::address_from_script_public_key,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        consensus::client::utils::pay_to_address_script,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        consensus::client::utils::pay_to_script_hash_script,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        consensus::client::utils::pay_to_script_hash_signature_script,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        consensus::client::utils::is_script_pay_to_pubkey,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        consensus::client::utils::is_script_pay_to_pubkey_ecdsa,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        consensus::client::utils::is_script_pay_to_script_hash,
        m
    )?)?;

    m.add_class::<consensus::core::hashing::PySighashType>()?;
    m.add_class::<consensus::core::script_public_key::PyScriptPublicKey>()?;
    m.add_class::<consensus::core::tx::PyTransactionId>()?;

    m.add_class::<wallet::bip32::language::PyLanguage>()?;
    m.add_class::<wallet::bip32::phrase::PyMnemonic>()?;
    m.add_class::<wallet::core::account::kind::PyAccountKind>()?;
    m.add_function(wrap_pyfunction!(
        wallet::core::derivation::create_multisig_address_py,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        wallet::core::tx::signer::py_sign_transaction,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        wallet::core::tx::signer::create_input_signature,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        wallet::core::tx::signer::sign_script_hash,
        m
    )?)?;

    m.add_function(wrap_pyfunction!(wallet::core::utils::kaspa_to_sompi, m)?)?;
    m.add_function(wrap_pyfunction!(wallet::core::utils::sompi_to_kaspa, m)?)?;
    m.add_function(wrap_pyfunction!(
        wallet::core::utils::sompi_to_kaspa_string_with_suffix,
        m
    )?)?;

    m.add_class::<crypto::txscript::builder::PyScriptBuilder>()?;
    m.add_class::<crypto::txscript::opcodes::PyOpcodes>()?;
    m.add_class::<crypto::hashes::PyHash>()?;

    m.add_class::<wallet::core::tx::generator::generator::PyGenerator>()?;
    m.add_class::<wallet::core::tx::generator::pending::PendingTransaction>()?;
    m.add_class::<wallet::core::tx::generator::summary::PyGeneratorSummary>()?;

    m.add_function(wrap_pyfunction!(
        wallet::core::tx::mass::maximum_standard_transaction_mass,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        wallet::core::tx::mass::calculate_unsigned_transaction_fee,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        wallet::core::tx::mass::calculate_unsigned_transaction_mass,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        wallet::core::tx::mass::calculate_storage_mass,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        wallet::core::tx::mass::update_unsigned_transaction_mass,
        m
    )?)?;

    m.add_class::<wallet::core::tx::payment::PyPaymentOutput>()?;

    m.add_function(wrap_pyfunction!(
        wallet::core::tx::utils::create_transaction_py,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        wallet::core::tx::utils::create_transactions_py,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        wallet::core::tx::utils::estimate_transactions_py,
        m
    )?)?;

    m.add_class::<network::PyNetworkId>()?;
    m.add_class::<network::PyNetworkType>()?;

    m.add_class::<rpc::wrpc::resolver::PyResolver>()?;
    m.add_class::<rpc::wrpc::client::PyRpcClient>()?;

    m.add_function(wrap_pyfunction!(wallet::core::message::py_sign_message, m)?)?;
    m.add_function(wrap_pyfunction!(
        wallet::core::message::py_verify_message,
        m
    )?)?;

    m.add_class::<wallet::keys::derivation::PyDerivationPath>()?;
    m.add_class::<wallet::keys::keypair::PyKeypair>()?;
    m.add_class::<wallet::keys::privatekey::PyPrivateKey>()?;
    m.add_class::<wallet::keys::privkeygen::PyPrivateKeyGenerator>()?;
    m.add_class::<wallet::keys::publickey::PyPublicKey>()?;
    m.add_class::<wallet::keys::pubkeygen::PyPublicKeyGenerator>()?;
    m.add_class::<wallet::keys::publickey::PyXOnlyPublicKey>()?;
    m.add_class::<wallet::keys::xprv::PyXPrv>()?;
    m.add_class::<wallet::keys::xpub::PyXPub>()?;

    Ok(())
}
