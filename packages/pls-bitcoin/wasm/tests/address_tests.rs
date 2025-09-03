use pls_bitcoin_wasm::*;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn get_jsvalue_error_string(js_value: wasm_bindgen::JsValue) -> String {
    js_value
        .as_string()
        .unwrap_or_else(|| "Not a string error".to_string())
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


