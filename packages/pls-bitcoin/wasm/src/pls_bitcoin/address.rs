use bitcoin::key::PublicKey;
use bitcoin::script::Builder as ScriptBuilder;
use bitcoin::Address;
use bitcoin::Network;
// use std::str::FromStr; // Unused
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_p2pkh_address(public_key_hex: &str, network_str: &str) -> Result<String, JsValue> {
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| JsValue::from_str(&format!("Invalid public key hex: {}", e)))?;
    let public_key = PublicKey::from_slice(&public_key_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse public key: {}", e)))?;

    let network = match network_str {
        "bitcoin" => Network::Bitcoin,
        "testnet" => Network::Testnet,
        "regtest" => Network::Regtest,
        _ => return Err(JsValue::from_str("Invalid network string")),
    };

    let address = Address::p2pkh(&public_key, network);
    Ok(address.to_string())
}

#[wasm_bindgen]
pub fn get_multisig_address(
    m: u8,
    public_keys_hex: js_sys::Array,
    network_str: &str,
) -> Result<String, JsValue> {
    let mut pubkeys: Vec<PublicKey> = Vec::new();
    for pk_hex_js in public_keys_hex.iter() {
        let pk_hex = pk_hex_js
            .as_string()
            .ok_or_else(|| JsValue::from_str("Public key hex must be a string"))?;
        let pk_bytes = hex::decode(pk_hex).map_err(|e| {
            JsValue::from_str(&format!("Invalid public key hex for multisig: {}", e))
        })?;
        let public_key = PublicKey::from_slice(&pk_bytes).map_err(|e| {
            JsValue::from_str(&format!("Failed to parse public key for multisig: {}", e))
        })?;
        pubkeys.push(public_key);
    }

    if m == 0 || m as usize > pubkeys.len() {
        return Err(JsValue::from_str("Invalid m value for multisig"));
    }

    let network = match network_str {
        "bitcoin" => Network::Bitcoin,
        "testnet" => Network::Testnet,
        "regtest" => Network::Regtest,
        _ => return Err(JsValue::from_str("Invalid network string")),
    };

    let mut builder = ScriptBuilder::new();
    builder = builder.push_int(m as i64);
    for pk in pubkeys.iter() {
        // PublicKey::to_bytes() returns Vec<u8>. ScriptBuilder::push_slice expects &[u8].
        // In bitcoin 0.32.6, PublicKey::to_bytes() is not available directly.
        // We need to serialize the PublicKey.inner (which is secp256k1::PublicKey)
        // For compressed keys: pk.inner.serialize()
        // For uncompressed keys: pk.inner.serialize_uncompressed()
        // Assuming compressed keys as is standard.
        builder = builder.push_slice(&pk.inner.serialize());
    }
    builder = builder.push_int(pubkeys.len() as i64);
    builder = builder.push_opcode(bitcoin::opcodes::all::OP_CHECKMULTISIG);
    let script = builder.into_script();

    let address = Address::p2wsh(&script, network);

    Ok(address.to_string())
}
