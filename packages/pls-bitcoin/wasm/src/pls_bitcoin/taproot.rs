use bitcoin::key::TweakedPublicKey;
use bitcoin::script::Builder;
use bitcoin::script::ScriptBuf;
use bitcoin::secp256k1::Parity;
use bitcoin::taproot::{LeafVersion, TapLeafHash, TaprootBuilder, TaprootSpendInfo};
use bitcoin::Network;
use bitcoin::PublicKey;
use bitcoin::XOnlyPublicKey;
// use std::str::FromStr; // Unused
use serde_json::from_str;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

// Structure to represent a script with weight for the Huffman tree
#[derive(serde::Deserialize, Debug, Clone)]
struct ScriptWithWeight {
    script_hex: String,
    weight: usize,
}

fn build_simple_taproot_spend_info_from_scripts(
    internal_public_key: XOnlyPublicKey,
    scripts_hex: Vec<String>,
) -> Result<TaprootSpendInfo, String> {
    let secp = bitcoin::secp256k1::Secp256k1::new();
    if scripts_hex.is_empty() {
        return Ok(TaprootBuilder::new()
            .finalize(&secp, internal_public_key)
            .map_err(|e| format!("Failed to finalize taproot builder (key-path): {:?}", e))?);
    }

    let mut builder = TaprootBuilder::new();
    for script_hex_str in scripts_hex {
        let script_bytes = hex::decode(&script_hex_str)
            .map_err(|e| format!("Invalid script hex {}: {}", script_hex_str, e))?;
        let script = ScriptBuf::from(script_bytes);
        builder = builder
            .add_leaf(1, script)
            .map_err(|e| format!("Failed to add leaf to taproot builder: {:?}", e))?;
    }

    builder
        .finalize(&secp, internal_public_key)
        .map_err(|e| format!("Failed to finalize taproot builder: {:?}", e))
}

#[wasm_bindgen]
pub fn get_taproot_address(
    internal_public_key_hex: &str,
    scripts_json: &str,
    network_str: &str,
) -> Result<String, JsValue> {
    let internal_pk_bytes = hex::decode(internal_public_key_hex)
        .map_err(|e| JsValue::from_str(&format!("Invalid internal public key hex: {}", e)))?;
    let internal_pk = PublicKey::from_slice(&internal_pk_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse internal public key: {}", e)))?;

    // Corrected: PublicKey.inner (secp256k1::PublicKey) has x_only_public_key
    let (internal_x_only_pk, _parity) = internal_pk.inner.x_only_public_key();

    let network = match network_str {
        "bitcoin" => Network::Bitcoin,
        "testnet" => Network::Testnet,
        "regtest" => Network::Regtest,
        _ => return Err(JsValue::from_str("Invalid network string")),
    };

    let scripts_hex_vec: Vec<String> = from_str(scripts_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid scripts_json: {}", e)))?;

    let spend_info =
        build_simple_taproot_spend_info_from_scripts(internal_x_only_pk, scripts_hex_vec)
            .map_err(|e_str| JsValue::from_str(&e_str))?;

    let secp_for_address = bitcoin::secp256k1::Secp256k1::new();
    let address = bitcoin::Address::p2tr(
        &secp_for_address,
        internal_x_only_pk,
        spend_info.merkle_root(),
        network,
    );

    Ok(address.to_string())
}
