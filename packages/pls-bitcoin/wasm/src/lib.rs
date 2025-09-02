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

#[cfg(test)]
mod tests {
    use super::Base64Standard;
    use super::setup_panic_hook;

    use super::*;
    use bitcoin::opcodes::all as op;
    use bitcoin::script::Builder as ScriptBuilder;
    use bitcoin::Amount;
    use bitcoin::OutPoint;
    use bitcoin::Sequence;
    use bitcoin::Transaction;
    use bitcoin::TxIn;
    use bitcoin::TxOut;
    use bitcoin::Witness;
    use hex;
    use std::sync::Once;
    use wasm_bindgen_test::*; // Import Base64Standard into the test module scope

    static INIT_HOOK: Once = Once::new();


    fn get_jsvalue_error_string(js_value: JsValue) -> String {
        js_value
            .as_string()
            .unwrap_or_else(|| "Not a string error".to_string())
    }

    #[wasm_bindgen_test]
    fn test_to_js_error() {
        setup_panic_hook();
        let error_message = "Test error message";
        let js_error = to_js_error(error_message);
        assert_eq!(get_jsvalue_error_string(js_error), "\"Test error message\"");
    }

    #[wasm_bindgen_test]
    fn test_get_p2pkh_address_valid_mainnet() {
        setup_panic_hook();
        let pk_hex = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let pk_bytes = hex::decode(pk_hex).unwrap();
        let result = get_p2pkh_address(pk_bytes, "mainnet".to_string());
        assert!(result.is_ok(), "Result was: {:?}", result.err());
        assert_eq!(result.unwrap(), "1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH");
    }

    #[wasm_bindgen_test]
    fn test_get_p2pkh_address_valid_testnet() {
        setup_panic_hook();
        let pk_hex = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let pk_bytes = hex::decode(pk_hex).unwrap();
        let result = get_p2pkh_address(pk_bytes, "testnet".to_string());
        assert!(result.is_ok(), "Result was: {:?}", result.err());
        assert_eq!(result.unwrap(), "mrCDrCybB6J1vRfbwM5hemdJz73FwDBC8r");
    }

    #[wasm_bindgen_test]
    fn test_get_p2pkh_address_valid_regtest() {
        setup_panic_hook();
        let pk_hex = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let pk_bytes = hex::decode(pk_hex).unwrap();
        let result = get_p2pkh_address(pk_bytes, "regtest".to_string());
        assert!(result.is_ok(), "Result was: {:?}", result.err());
        assert_eq!(result.unwrap(), "mrCDrCybB6J1vRfbwM5hemdJz73FwDBC8r");
    }

    #[wasm_bindgen_test]
    fn test_get_p2pkh_address_invalid_pubkey() {
        setup_panic_hook();
        let pk_bytes = vec![0, 1, 2, 3];
        let result = get_p2pkh_address(pk_bytes, "mainnet".to_string());
        assert!(result.is_err());
        let error_msg = get_jsvalue_error_string(result.err().unwrap());
        assert!(error_msg.contains("PublicKey::from_slice error:"));
    }

    #[wasm_bindgen_test]
    fn test_get_p2pkh_address_invalid_network() {
        setup_panic_hook();
        let pk_hex = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let pk_bytes = hex::decode(pk_hex).unwrap();
        let result = get_p2pkh_address(pk_bytes, "invalidnet".to_string());
        assert!(result.is_err());
        assert_eq!(
            get_jsvalue_error_string(result.err().unwrap()),
            "\"Invalid network string\""
        );
    }

    #[wasm_bindgen_test]
    fn test_start_function_callable() {
        setup_panic_hook();
        start();
        assert!(true);
    }

    fn get_valid_xonly_pk_bytes() -> Vec<u8> {
        hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").unwrap()
    }

    #[wasm_bindgen_test]
    fn test_get_taproot_address_key_path_mainnet() {
        setup_panic_hook();
        let internal_pk_bytes = get_valid_xonly_pk_bytes();
        let result = get_taproot_address(
            internal_pk_bytes.clone(),
            JsValue::NULL,
            "mainnet".to_string(),
        );
        assert!(
            result.is_ok(),
            "Taproot mainnet key-path failed. Actual: {:?}",
            result.err().map(get_jsvalue_error_string)
        );
        assert_eq!(
            result.unwrap(),
            "bc1pmfr3p9j00pfxjh0zmgp99y8zftmd3s5pmedqhyptwy6lm87hf5sspknck9"
        );
    }

    #[wasm_bindgen_test]
    fn test_get_taproot_address_key_path_testnet() {
        setup_panic_hook();
        let internal_pk_bytes = get_valid_xonly_pk_bytes();
        let result = get_taproot_address(
            internal_pk_bytes.clone(),
            JsValue::UNDEFINED,
            "testnet".to_string(),
        );
        assert!(
            result.is_ok(),
            "Taproot testnet key-path failed. Actual: {:?}",
            result.err().map(get_jsvalue_error_string)
        );
        assert_eq!(
            result.unwrap(),
            "tb1pmfr3p9j00pfxjh0zmgp99y8zftmd3s5pmedqhyptwy6lm87hf5ssk79hv2"
        );
    }

    #[wasm_bindgen_test]
    fn test_get_taproot_address_with_empty_leaves_array() {
        setup_panic_hook();
        let internal_pk_bytes = get_valid_xonly_pk_bytes();
        let empty_leaves: Vec<TapLeafInfo> = Vec::new();
        let leaves_js = serde_wasm_bindgen::to_value(&empty_leaves).unwrap();
        let result =
            get_taproot_address(internal_pk_bytes.clone(), leaves_js, "mainnet".to_string());
        assert!(
            result.is_ok(),
            "Taproot mainnet empty leaves failed. Actual: {:?}",
            result.err().map(get_jsvalue_error_string)
        );
        assert_eq!(
            result.unwrap(),
            "bc1pmfr3p9j00pfxjh0zmgp99y8zftmd3s5pmedqhyptwy6lm87hf5sspknck9"
        );
    }

    #[wasm_bindgen_test]
    fn test_get_taproot_address_with_one_leaf() {
        setup_panic_hook();
        let internal_pk_bytes = get_valid_xonly_pk_bytes();
        let script_bytes = ScriptBuilder::new()
            .push_opcode(op::OP_CHECKSIG)
            .into_bytes();
        let leaves = vec![TapLeafInfo { script_bytes }];
        let leaves_js = serde_wasm_bindgen::to_value(&leaves).unwrap();
        let result =
            get_taproot_address(internal_pk_bytes.clone(), leaves_js, "mainnet".to_string());
        assert!(
            result.is_ok(),
            "get_taproot_address with one leaf failed: {:?}",
            result.err().map(get_jsvalue_error_string)
        );
        let addr = result.unwrap();
        assert_eq!(
            addr,
            "bc1py4x88fznv4y8er4r3cdtt74jvt8wmm9e9ntvlk3e8yxe2jtzhjrqdtqep9"
        );
    }

    #[wasm_bindgen_test]
    fn test_get_taproot_address_invalid_internal_pk() {
        setup_panic_hook();
        let internal_pk_bytes = vec![1, 2, 3];
        let result = get_taproot_address(internal_pk_bytes, JsValue::NULL, "mainnet".to_string());
        assert!(result.is_err());
        assert!(
            get_jsvalue_error_string(result.err().unwrap()).contains("Invalid internal public key")
        );
    }

    #[wasm_bindgen_test]
    fn test_get_taproot_address_invalid_network() {
        setup_panic_hook();
        let internal_pk_bytes = get_valid_xonly_pk_bytes();
        let result =
            get_taproot_address(internal_pk_bytes, JsValue::NULL, "invalidnet".to_string());
        assert!(result.is_err());
        assert_eq!(
            get_jsvalue_error_string(result.err().unwrap()),
            "\"Invalid network string\""
        );
    }

    #[wasm_bindgen_test]
    fn test_get_taproot_address_invalid_leaves_js_value() {
        setup_panic_hook();
        let internal_pk_bytes = get_valid_xonly_pk_bytes();
        let leaves_js = JsValue::from_str("not_an_array");
        let result = get_taproot_address(internal_pk_bytes, leaves_js, "mainnet".to_string());
        assert!(result.is_err());
        assert!(get_jsvalue_error_string(result.err().unwrap())
            .contains("serde_wasm_bindgen::from_value error for TapLeafInfo"));
    }

    fn get_valid_pubkey_vec_bytes() -> Vec<Vec<u8>> {
        vec![
            hex::decode("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
                .unwrap(),
            hex::decode("02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5")
                .unwrap(),
            hex::decode("02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9")
                .unwrap(),
        ]
    }

    #[wasm_bindgen_test]
    fn test_get_multisig_address_2_of_3_p2sh_mainnet() {
        setup_panic_hook();
        let pubkeys_bytes = get_valid_pubkey_vec_bytes();
        let pubkeys_js = serde_wasm_bindgen::to_value(&pubkeys_bytes).unwrap();
        let result = get_multisig_address(2, pubkeys_js, "mainnet".to_string(), "p2sh".to_string());
        assert!(
            result.is_ok(),
            "Result was: {:?}",
            result.err().map(get_jsvalue_error_string)
        );
        assert_eq!(result.unwrap(), "33hG2q39jRi2NqicRJB4ggY1J8EJm97Szz");
    }

    #[wasm_bindgen_test]
    fn test_get_multisig_address_1_of_1_p2wsh_testnet() {
        setup_panic_hook();
        let pubkeys_bytes = vec![get_valid_pubkey_vec_bytes()[0].clone()];
        let pubkeys_js = serde_wasm_bindgen::to_value(&pubkeys_bytes).unwrap();
        let result =
            get_multisig_address(1, pubkeys_js, "testnet".to_string(), "p2wsh".to_string());
        assert!(
            result.is_ok(),
            "Result was: {:?}",
            result.err().map(get_jsvalue_error_string)
        );
        assert_eq!(
            result.unwrap(),
            "tb1q9qs9xv7mjghkd69fgx62xttxmeww5q7eekjxu0nxtzf4yu4ekf8szffujl"
        );
    }

    #[wasm_bindgen_test]
    fn test_get_multisig_address_invalid_m_zero() {
        setup_panic_hook();
        let pubkeys_js = serde_wasm_bindgen::to_value(&get_valid_pubkey_vec_bytes()).unwrap();
        let result = get_multisig_address(0, pubkeys_js, "mainnet".to_string(), "p2sh".to_string());
        assert!(result.is_err());
        assert_eq!(
            get_jsvalue_error_string(result.err().unwrap()),
            "\"Invalid m value for multisig\""
        );
    }

    #[wasm_bindgen_test]
    fn test_get_multisig_address_invalid_m_too_large() {
        setup_panic_hook();
        let pubkeys_js = serde_wasm_bindgen::to_value(&get_valid_pubkey_vec_bytes()).unwrap();
        let result = get_multisig_address(4, pubkeys_js, "mainnet".to_string(), "p2sh".to_string());
        assert!(result.is_err());
        assert_eq!(
            get_jsvalue_error_string(result.err().unwrap()),
            "\"Invalid m value for multisig\""
        );
    }

    #[wasm_bindgen_test]
    fn test_get_multisig_address_too_many_pubkeys() {
        setup_panic_hook();
        let valid_pk_bytes =
            hex::decode("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
                .unwrap();
        let mut pk_bytes_list = Vec::new();
        for _i in 0..17 {
            // Create 17 valid (though identical) pubkeys
            pk_bytes_list.push(valid_pk_bytes.clone());
        }
        let pubkeys_js = serde_wasm_bindgen::to_value(&pk_bytes_list).unwrap();
        let result = get_multisig_address(1, pubkeys_js, "mainnet".to_string(), "p2sh".to_string());
        assert!(
            result.is_err(),
            "Should have failed due to too many pubkeys. Actual: {:?}",
            result.ok()
        );
        assert_eq!(
            get_jsvalue_error_string(result.err().unwrap()),
            "\"Too many public keys for multisig (max 16)\""
        );
    }

    #[wasm_bindgen_test]
    fn test_get_multisig_address_invalid_pubkey_in_list() {
        setup_panic_hook();
        let pubkeys_bytes = vec![vec![1, 2, 3]];
        let pubkeys_js = serde_wasm_bindgen::to_value(&pubkeys_bytes).unwrap();
        let result = get_multisig_address(1, pubkeys_js, "mainnet".to_string(), "p2sh".to_string());
        assert!(result.is_err());
        assert!(
            get_jsvalue_error_string(result.err().unwrap()).contains("Invalid public key in list")
        );
    }

    #[wasm_bindgen_test]
    fn test_get_multisig_address_invalid_network_str() {
        setup_panic_hook();
        let pubkeys_js = serde_wasm_bindgen::to_value(&get_valid_pubkey_vec_bytes()).unwrap();
        let result =
            get_multisig_address(2, pubkeys_js, "invalidnet".to_string(), "p2sh".to_string());
        assert!(result.is_err());
        assert_eq!(
            get_jsvalue_error_string(result.err().unwrap()),
            "\"Invalid network string\""
        );
    }

    #[wasm_bindgen_test]
    fn test_get_multisig_address_invalid_address_type() {
        setup_panic_hook();
        let pubkeys_js = serde_wasm_bindgen::to_value(&get_valid_pubkey_vec_bytes()).unwrap();
        let result = get_multisig_address(
            2,
            pubkeys_js,
            "mainnet".to_string(),
            "invalidtype".to_string(),
        );
        assert!(result.is_err());
        assert_eq!(
            get_jsvalue_error_string(result.err().unwrap()),
            "\"Invalid address_type. Must be 'p2sh' or 'p2wsh'\""
        );
    }

    #[wasm_bindgen_test]
    fn test_get_multisig_address_invalid_pubkeys_js_value() {
        setup_panic_hook();
        let pubkeys_js = JsValue::from_str("not_an_array");
        let result = get_multisig_address(1, pubkeys_js, "mainnet".to_string(), "p2sh".to_string());
        assert!(result.is_err());
        assert!(get_jsvalue_error_string(result.err().unwrap())
            .contains("serde_wasm_bindgen::from_value error for public_keys_outer"));
    }

    fn get_valid_xonly_pubkey_vec_bytes() -> Vec<Vec<u8>> {
        vec![
            hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
                .unwrap(),
            hex::decode("11c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709e")
                .unwrap(),
            hex::decode("22f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036")
                .unwrap(),
        ]
    }

    #[wasm_bindgen_test]
    fn test_get_collateral_output_script_valid() {
        setup_panic_hook();
        let arbitrator_pks_bytes = get_valid_xonly_pubkey_vec_bytes();
        let arbitrator_pks_js = serde_wasm_bindgen::to_value(&arbitrator_pks_bytes).unwrap();
        let user_pk_bytes = get_valid_xonly_pk_bytes();
        let result = get_collateral_output_script(arbitrator_pks_js, 2, user_pk_bytes);
        assert!(
            result.is_ok(),
            "Result was: {:?}",
            result.err().map(get_jsvalue_error_string)
        );
        assert!(!result.unwrap().is_empty());
    }

    #[wasm_bindgen_test]
    fn test_get_collateral_output_script_invalid_m_quorum_zero() {
        setup_panic_hook();
        let arbitrator_pks_js =
            serde_wasm_bindgen::to_value(&get_valid_xonly_pubkey_vec_bytes()).unwrap();
        let user_pk_bytes = get_valid_xonly_pk_bytes();
        let result = get_collateral_output_script(arbitrator_pks_js, 0, user_pk_bytes);
        assert!(result.is_err());
        assert_eq!(
            get_jsvalue_error_string(result.err().unwrap()),
            "\"Invalid m_quorum value for collateral script\""
        );
    }

    #[wasm_bindgen_test]
    fn test_get_collateral_output_script_invalid_m_quorum_too_large() {
        setup_panic_hook();
        let arbitrator_pks_js =
            serde_wasm_bindgen::to_value(&get_valid_xonly_pubkey_vec_bytes()).unwrap();
        let user_pk_bytes = get_valid_xonly_pk_bytes();
        let result = get_collateral_output_script(arbitrator_pks_js, 4, user_pk_bytes);
        assert!(result.is_err());
        assert_eq!(
            get_jsvalue_error_string(result.err().unwrap()),
            "\"Invalid m_quorum value for collateral script\""
        );
    }

    #[wasm_bindgen_test]
    fn test_get_collateral_output_script_invalid_arbitrator_pk() {
        setup_panic_hook();
        let arbitrator_pks_bytes = vec![vec![1, 2, 3]];
        let arbitrator_pks_js = serde_wasm_bindgen::to_value(&arbitrator_pks_bytes).unwrap();
        let user_pk_bytes = get_valid_xonly_pk_bytes();
        let result = get_collateral_output_script(arbitrator_pks_js, 1, user_pk_bytes);
        assert!(result.is_err());
        assert!(get_jsvalue_error_string(result.err().unwrap())
            .contains("Invalid arbitrator x-only public key in list"));
    }

    #[wasm_bindgen_test]
    fn test_get_collateral_output_script_invalid_user_pk() {
        setup_panic_hook();
        let arbitrator_pks_js =
            serde_wasm_bindgen::to_value(&get_valid_xonly_pubkey_vec_bytes()).unwrap();
        let user_pk_bytes = vec![1, 2, 3];
        let result = get_collateral_output_script(arbitrator_pks_js, 2, user_pk_bytes);
        assert!(result.is_err());
        assert!(get_jsvalue_error_string(result.err().unwrap())
            .contains("Invalid user internal x-only public key"));
    }

    #[wasm_bindgen_test]
    fn test_get_collateral_output_script_invalid_arbitrator_pks_js_value() {
        setup_panic_hook();
        let arbitrator_pks_js = JsValue::from_str("not_an_array");
        let user_pk_bytes = get_valid_xonly_pk_bytes();
        let result = get_collateral_output_script(arbitrator_pks_js, 1, user_pk_bytes);
        assert!(result.is_err());
        assert!(get_jsvalue_error_string(result.err().unwrap())
            .contains("serde_wasm_bindgen::from_value error for arbitrator_x_only_pubkeys_outer"));
    }

    #[wasm_bindgen_test]
    fn test_sign_psbt_is_stub() {
        setup_panic_hook();
        let tx = Transaction {
            version: bitcoin::transaction::Version::ONE,
            lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
            input: vec![],
            output: vec![],
        };
        let psbt = Psbt::from_unsigned_tx(tx).unwrap();
        let psbt_base64 = Base64Standard.encode(psbt.serialize());
        let sign_requests: Vec<SignRequestInfo> = vec![];
        let sign_requests_js = serde_wasm_bindgen::to_value(&sign_requests).unwrap();
        let result = sign_psbt(psbt_base64, sign_requests_js);
        assert!(result.is_err());
        assert_eq!(
            get_jsvalue_error_string(result.err().unwrap()),
            "\"PSBT signing logic needs detailed implementation using bitcoin 0.32.x specifics.\""
        );
    }

    #[wasm_bindgen_test]
    fn test_sign_psbt_invalid_base64() {
        setup_panic_hook();
        let result = sign_psbt("invalid_base64".to_string(), JsValue::NULL);
        assert!(result.is_err());
        assert!(get_jsvalue_error_string(result.err().unwrap())
            .contains("Failed to decode PSBT from Base64"));
    }

    #[wasm_bindgen_test]
    fn test_sign_psbt_invalid_psbt_bytes() {
        setup_panic_hook();
        let psbt_base64 = Base64Standard.encode(b"invalid psbt bytes");
        let result = sign_psbt(psbt_base64, JsValue::NULL);
        assert!(result.is_err());
        assert!(
            get_jsvalue_error_string(result.err().unwrap()).contains("Failed to deserialize PSBT")
        );
    }

    #[wasm_bindgen_test]
    fn test_sign_psbt_invalid_sign_requests_js_value() {
        setup_panic_hook();
        let tx = Transaction {
            version: bitcoin::transaction::Version::ONE,
            lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
            input: vec![],
            output: vec![],
        };
        let psbt = Psbt::from_unsigned_tx(tx).unwrap();
        let psbt_base64 = Base64Standard.encode(psbt.serialize());
        let sign_requests_js = JsValue::from_str("not_an_array");
        let result = sign_psbt(psbt_base64, sign_requests_js);
        assert!(result.is_err());
        assert!(get_jsvalue_error_string(result.err().unwrap())
            .contains("serde_wasm_bindgen::from_value error for SignRequestInfo"));
    }

    fn get_minimal_psbt_base64() -> String {
        let tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
            input: vec![],
            output: vec![],
        };
        let psbt = Psbt::from_unsigned_tx(tx).unwrap();
        Base64Standard.encode(psbt.serialize())
    }

    #[wasm_bindgen_test]
    fn test_finalize_psbt_valid() {
        setup_panic_hook();
        let psbt_base64 = get_minimal_psbt_base64();
        let result = finalize_psbt(psbt_base64.clone());
        assert!(result.is_ok(), "Result was: {:?}", result.err());
        assert_eq!(result.unwrap(), psbt_base64);
    }

    #[wasm_bindgen_test]
    fn test_finalize_psbt_invalid_base64() {
        setup_panic_hook();
        let result = finalize_psbt("invalid_base64".to_string());
        assert!(result.is_err());
        assert!(get_jsvalue_error_string(result.err().unwrap())
            .contains("Failed to decode PSBT from Base64"));
    }

    #[wasm_bindgen_test]
    fn test_finalize_psbt_invalid_psbt_bytes() {
        setup_panic_hook();
        let psbt_base64 = Base64Standard.encode(b"invalid psbt bytes");
        let result = finalize_psbt(psbt_base64);
        assert!(result.is_err());
        assert!(
            get_jsvalue_error_string(result.err().unwrap()).contains("Failed to deserialize PSBT")
        );
    }

    #[wasm_bindgen_test]
    fn test_extract_transaction_valid_minimal() {
        setup_panic_hook();
        let tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
            input: vec![],
            output: vec![],
        };
        let psbt = Psbt::from_unsigned_tx(tx.clone()).unwrap();
        let psbt_base64 = Base64Standard.encode(psbt.serialize());
        let result = extract_transaction(psbt_base64);
        assert!(result.is_ok(), "Result was: {:?}", result.err());
        assert_eq!(result.unwrap(), serialize_hex(&tx));
    }

    #[wasm_bindgen_test]
    fn test_extract_transaction_invalid_base64() {
        setup_panic_hook();
        let result = extract_transaction("invalid_base64".to_string());
        assert!(result.is_err());
        assert!(get_jsvalue_error_string(result.err().unwrap())
            .contains("Failed to decode PSBT from Base64"));
    }

    #[wasm_bindgen_test]
    fn test_extract_transaction_invalid_psbt_bytes() {
        setup_panic_hook();
        let psbt_base64 = Base64Standard.encode(b"invalid psbt bytes");
        let result = extract_transaction(psbt_base64);
        assert!(result.is_err());
        assert!(
            get_jsvalue_error_string(result.err().unwrap()).contains("Failed to deserialize PSBT")
        );
    }

    #[wasm_bindgen_test]
    fn test_extract_transaction_with_unfinalized_input() {
        setup_panic_hook();
        let tx_input = TxIn {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        };
        let tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
            input: vec![tx_input],
            output: vec![],
        };
        let mut psbt = Psbt::from_unsigned_tx(tx.clone()).unwrap();
        psbt.inputs[0].witness_utxo = Some(TxOut {
            value: Amount::from_sat(1000),
            script_pubkey: ScriptBuilder::new().into_script(),
        });

        let psbt_base64 = Base64Standard.encode(psbt.serialize());
        let result = extract_transaction(psbt_base64);
        assert!(
            result.is_ok(),
            "Result was: {:?}",
            result.err().map(get_jsvalue_error_string)
        );
        assert_eq!(result.unwrap(), serialize_hex(&tx));
    }
}
