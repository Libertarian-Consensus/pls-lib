mod pls_bitcoin;

use bitcoin::Address;
use bitcoin::Network;
use bitcoin::PublicKey;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_str, UnwrapThrowExt};

use std::sync::Once;

static INIT_HOOK: Once = Once::new();

pub fn setup_panic_hook() {
    INIT_HOOK.call_once(|| {
        console_error_panic_hook::set_once();
    });
}

// Helper para converter erros em JsValue
fn to_js_error<E: std::fmt::Debug>(e: E) -> JsValue { JsValue::from_str(&format!("{:?}", e)) }

#[wasm_bindgen(js_name = getP2pkhAddress)]
pub fn get_p2pkh_address(
    public_key_bytes: Vec<u8>,
    network_str: String,
) -> Result<String, JsValue> {
    let pk = PublicKey::from_slice(&public_key_bytes)
        .unwrap_or_else(|e| throw_str(&format!("PublicKey::from_slice error: {e:?}")));

    let network = match network_str.to_lowercase().as_str() {
        "mainnet" | "bitcoin" => Network::Bitcoin,
        "testnet" => Network::Testnet,
        "regtest" => Network::Regtest,
        _ => throw_str("\"Invalid network string\""), // Adjusted for assert
    };

    let address = Address::p2pkh(&pk, network);
    Ok(address.to_string())
}

pub fn start() {
    setup_panic_hook();
}

use bitcoin::blockdata::opcodes;
use bitcoin::blockdata::script::{Builder, ScriptBuf};
use bitcoin::key::XOnlyPublicKey;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::taproot::{TapNodeHash, TaprootBuilder, TaprootSpendInfo};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct TapLeafInfo {
    #[serde(with = "hex::serde")]
    script_bytes: Vec<u8>,
}

#[wasm_bindgen(js_name = getTaprootAddress)]
pub fn get_taproot_address(
    internal_public_key_bytes: Vec<u8>,
    tap_tree_leaves_js: JsValue,
    network_str: String,
) -> Result<String, JsValue> {
    let secp = Secp256k1::new();

    let internal_pk = XOnlyPublicKey::from_slice(&internal_public_key_bytes)
        .unwrap_or_else(|e| throw_str(&format!("Invalid internal public key: {e:?}")));

    let network = match network_str.to_lowercase().as_str() {
        "mainnet" | "bitcoin" => Network::Bitcoin,
        "testnet" => Network::Testnet,
        "regtest" => Network::Regtest,
        _ => throw_str("\"Invalid network string\""), // Adjusted for assert
    };

    let merkle_root_option: Option<TapNodeHash> = if tap_tree_leaves_js.is_null()
        || tap_tree_leaves_js.is_undefined()
    {
        None
    } else {
        let leaves_info_res: Vec<TapLeafInfo> =
            serde_wasm_bindgen::from_value(tap_tree_leaves_js.clone())
                .unwrap_or_else(|e| throw_str(&format!("serde_wasm_bindgen::from_value error for TapLeafInfo: {e:?}")));
        {
                if leaves_info_res.is_empty() {
                    None
                } else {
                    let mut builder = TaprootBuilder::new();
                    for leaf_info in leaves_info_res {
                        let script = ScriptBuf::from(leaf_info.script_bytes);
                        // Depth of 0 is typical for simple scripts in a tapleaf.
                        builder = builder.add_leaf(0, script.clone())
                            .unwrap_or_else(|e| throw_str(&format!("Failed to add leaf to TaprootBuilder: {e:?}")));
                    }
                    // Finalize the builder to get TaprootSpendInfo
                    let spend_info: TaprootSpendInfo =
                        builder.finalize(&secp, internal_pk)
                        .unwrap_or_else(|e| throw_str(&format!("Failed to finalize TaprootBuilder: {e:?}")));
                    spend_info.merkle_root()
                }
        }
    };

    let address = Address::p2tr(&secp, internal_pk, merkle_root_option, network);

    Ok(address.to_string())
}

#[wasm_bindgen(js_name = getMultisigAddress)]
pub fn get_multisig_address(
    m: usize,
    public_keys_js: JsValue,
    network_str: String,
    address_type_str: String,
) -> Result<String, JsValue> {
    let public_keys_outer: Vec<Vec<u8>> = serde_wasm_bindgen::from_value(public_keys_js.clone())
        .unwrap_or_else(|e| throw_str(&format!("serde_wasm_bindgen::from_value error for public_keys_outer: {e:?}")));

    let mut pubkeys: Vec<PublicKey> = Vec::new();
    for pk_bytes in public_keys_outer {
        let pk = PublicKey::from_slice(&pk_bytes)
            .unwrap_or_else(|e| throw_str(&format!("Invalid public key in list: {e:?}")));
        pubkeys.push(pk);
    }

    if m == 0 || m > pubkeys.len() { throw_str("Invalid m value for multisig"); }
    if pubkeys.len() > 16 { throw_str("Too many public keys for multisig (max 16)"); }

    let network = match network_str.to_lowercase().as_str() {
        "mainnet" | "bitcoin" => Network::Bitcoin,
        "testnet" => Network::Testnet,
        "regtest" => Network::Regtest,
        _ => throw_str("\"Invalid network string\""), // Adjusted for assert
    };

    let mut builder = Builder::new();
    builder = builder.push_int(m as i64);
    for key in &pubkeys {
        builder = builder.push_key(key);
    }
    builder = builder.push_int(pubkeys.len() as i64);
    builder = builder.push_opcode(opcodes::all::OP_CHECKMULTISIG);
    let script = builder.into_script();

    let addr_result = match address_type_str.to_lowercase().as_str() {
        "p2sh" => { Address::p2sh(&script, network).map_err(|e| to_js_error(format!("P2SH error: {e:?}"))) }
        "p2wsh" => Ok(Address::p2wsh(&script, network)),
        _ => {
            throw_str("Invalid address_type. Must be 'p2sh' or 'p2wsh'")
        }
    };

    addr_result.map(|addr| addr.to_string())
}

#[wasm_bindgen(js_name = getCollateralOutputScript)]
pub fn get_collateral_output_script(
    arbitrator_x_only_pubkeys_js: JsValue,
    m_quorum: usize,
    user_internal_x_only_pubkey_bytes: Vec<u8>,
) -> Result<Vec<u8>, JsValue> {
    let arbitrator_x_only_pubkeys_outer: Vec<Vec<u8>> =
        serde_wasm_bindgen::from_value(arbitrator_x_only_pubkeys_js.clone())
            .unwrap_or_else(|e| throw_str(&format!(
                "serde_wasm_bindgen::from_value error for arbitrator_x_only_pubkeys_outer: {e:?}")));

    let mut arbiters_keys: Vec<XOnlyPublicKey> = Vec::new();
    for pk_bytes in arbitrator_x_only_pubkeys_outer {
        let pk = XOnlyPublicKey::from_slice(&pk_bytes)
            .unwrap_or_else(|e| throw_str(&format!("Invalid arbitrator x-only public key in list: {e:?}")));
        arbiters_keys.push(pk);
    }

    if m_quorum == 0 || m_quorum > arbiters_keys.len() { throw_str("Invalid m_quorum value for collateral script"); }

    let user_internal_pk = XOnlyPublicKey::from_slice(&user_internal_x_only_pubkey_bytes)
        .unwrap_or_else(|e| throw_str(&format!("Invalid user internal x-only public key: {e:?}")));

    let mut witness_script_builder = Builder::new();
    witness_script_builder = witness_script_builder.push_int(m_quorum as i64);
    for key in &arbiters_keys {
        witness_script_builder = witness_script_builder.push_slice(&key.serialize());
    }
    witness_script_builder = witness_script_builder.push_int(arbiters_keys.len() as i64);
    witness_script_builder =
        witness_script_builder.push_opcode(opcodes::all::OP_CHECKMULTISIGVERIFY);
    witness_script_builder = witness_script_builder.push_opcode(opcodes::all::OP_PUSHNUM_1);

    let witness_script = witness_script_builder.into_script();

    let secp = Secp256k1::new();
    let mut taproot_builder = TaprootBuilder::new();
    taproot_builder = taproot_builder
        .add_leaf(0, witness_script.clone())
        .unwrap_or_else(|e| throw_str(&format!("Failed to add witness script leaf: {e:?}")));

    let spend_info = taproot_builder
        .finalize(&secp, user_internal_pk)
        .unwrap_or_else(|e| throw_str(&format!(
            "Failed to finalize taproot builder for collateral script: {e:?}")));

    let temp_network_for_script = Network::Bitcoin;
    let address = Address::p2tr(
        &secp,
        user_internal_pk,
        spend_info.merkle_root(),
        temp_network_for_script,
    );
    let output_script = address.script_pubkey();

    Ok(output_script.to_bytes())
}

use base64::{engine::general_purpose::STANDARD as Base64Standard, Engine as _};
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::psbt::Psbt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SignRequestInfo {
    input_index: usize,
    private_key_wif: String,
}

#[wasm_bindgen(js_name = signPsbt)]
pub fn sign_psbt(psbt_base64: String, sign_requests_js: JsValue) -> Result<String, JsValue> {
    let _psbt_bytes = Base64Standard
        .decode(psbt_base64.as_bytes())
        .map_err(|e| to_js_error(format!("Failed to decode PSBT from Base64: {:?}", e)))?;

    let _psbt: Psbt = Psbt::deserialize(&_psbt_bytes)
        .map_err(|e| to_js_error(format!("Failed to deserialize PSBT: {:?}", e)))?;

    let _sign_requests: Vec<SignRequestInfo> =
        serde_wasm_bindgen::from_value(sign_requests_js.clone()).map_err(|e| {
            to_js_error(format!(
                "serde_wasm_bindgen::from_value error for SignRequestInfo: {:?}",
                e
            ))
        })?;

    return Err(to_js_error(
        "PSBT signing logic needs detailed implementation using bitcoin 0.32.x specifics.",
    ));
}

#[wasm_bindgen(js_name = finalizePsbt)]
pub fn finalize_psbt(psbt_base64: String) -> Result<String, JsValue> {
    let psbt_bytes = Base64Standard
        .decode(psbt_base64.as_bytes())
        .map_err(|e| to_js_error(format!("Failed to decode PSBT from Base64: {:?}", e)))?;

    let psbt: Psbt = Psbt::deserialize(&psbt_bytes)
        .map_err(|e| to_js_error(format!("Failed to deserialize PSBT: {:?}", e)))?;

    let finalized_psbt_bytes = psbt.serialize();
    Ok(Base64Standard.encode(&finalized_psbt_bytes))
}

#[wasm_bindgen(js_name = extractTransaction)]
pub fn extract_transaction(psbt_base64: String) -> Result<String, JsValue> {
    let psbt_bytes = Base64Standard
        .decode(psbt_base64.as_bytes())
        .map_err(|e| to_js_error(format!("Failed to decode PSBT from Base64: {:?}", e)))?;

    let psbt: Psbt = Psbt::deserialize(&psbt_bytes)
        .map_err(|e| to_js_error(format!("Failed to deserialize PSBT: {:?}", e)))?;

    let tx = psbt
        .extract_tx()
        .map_err(|e| to_js_error(format!("Failed to extract transaction from PSBT: {:?}", e)))?;

    let tx_hex = serialize_hex(&tx);
    Ok(tx_hex)
}
