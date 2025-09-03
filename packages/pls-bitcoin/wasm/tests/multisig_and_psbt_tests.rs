use pls_bitcoin_wasm::*;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn get_jsvalue_error_string(js_value: wasm_bindgen::JsValue) -> String {
    js_value
        .as_string()
        .unwrap_or_else(|| "Not a string error".to_string())
}

fn get_valid_pubkey_vec_bytes() -> Vec<Vec<u8>> {
    vec![
        hex::decode("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").unwrap(),
        hex::decode("02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5").unwrap(),
        hex::decode("02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9").unwrap(),
    ]
}

#[wasm_bindgen_test]
fn test_get_multisig_address_2_of_3_p2sh_mainnet() {
    setup_panic_hook();
    let pubkeys_bytes = get_valid_pubkey_vec_bytes();
    let pubkeys_js = serde_wasm_bindgen::to_value(&pubkeys_bytes).unwrap();
    let result = get_multisig_address(2, pubkeys_js, "mainnet".to_string(), "p2sh".to_string());
    assert!(result.is_ok(), "Result was: {:?}", result.err().map(get_jsvalue_error_string));
    assert_eq!(result.unwrap(), "33hG2q39jRi2NqicRJB4ggY1J8EJm97Szz");
}

#[wasm_bindgen_test]
fn test_get_multisig_address_invalid_address_type() {
    setup_panic_hook();
    let pubkeys_js = serde_wasm_bindgen::to_value(&get_valid_pubkey_vec_bytes()).unwrap();
    let result = get_multisig_address(2, pubkeys_js, "mainnet".to_string(), "invalidtype".to_string());
    assert!(result.is_err());
    assert_eq!(get_jsvalue_error_string(result.err().unwrap()), "\"Invalid address_type. Must be 'p2sh' or 'p2wsh'\"");
}

#[wasm_bindgen_test]
fn test_sign_psbt_is_stub() {
    setup_panic_hook();
    let tx = bitcoin::Transaction {
        version: bitcoin::transaction::Version::ONE,
        lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
        input: vec![],
        output: vec![],
    };
    let psbt = bitcoin::psbt::Psbt::from_unsigned_tx(tx).unwrap();
    let psbt_base64 = base64::engine::general_purpose::STANDARD.encode(psbt.serialize());
    let sign_requests: Vec<serde_json::Value> = vec![];
    let sign_requests_js = serde_wasm_bindgen::to_value(&sign_requests).unwrap();
    let result = sign_psbt(psbt_base64, sign_requests_js);
    assert!(result.is_err());
}


