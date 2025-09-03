use pls_bitcoin_wasm::*;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn get_jsvalue_error_string(js_value: wasm_bindgen::JsValue) -> String {
    js_value
        .as_string()
        .unwrap_or_else(|| "Not a string error".to_string())
}

fn get_valid_xonly_pk_bytes() -> Vec<u8> {
    hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").unwrap()
}

#[wasm_bindgen_test]
fn test_get_taproot_address_key_path_mainnet() {
    setup_panic_hook();
    let internal_pk_bytes = get_valid_xonly_pk_bytes();
    let result = get_taproot_address(internal_pk_bytes.clone(), JsValue::NULL, "mainnet".to_string());
    assert!(result.is_ok(), "Taproot mainnet key-path failed. Actual: {:?}", result.err().map(get_jsvalue_error_string));
    assert_eq!(result.unwrap(), "bc1pmfr3p9j00pfxjh0zmgp99y8zftmd3s5pmedqhyptwy6lm87hf5sspknck9");
}

#[wasm_bindgen_test]
fn test_get_taproot_address_with_one_leaf() {
    setup_panic_hook();
    let internal_pk_bytes = get_valid_xonly_pk_bytes();
    let script_bytes = bitcoin::script::Builder::new().push_opcode(bitcoin::opcodes::all::OP_CHECKSIG).into_bytes();
    let leaves = vec![TapLeafInfo { script_bytes }];
    let leaves_js = serde_wasm_bindgen::to_value(&leaves).unwrap();
    let result = get_taproot_address(internal_pk_bytes.clone(), leaves_js, "mainnet".to_string());
    assert!(result.is_ok(), "get_taproot_address with one leaf failed: {:?}", result.err().map(get_jsvalue_error_string));
    let addr = result.unwrap();
    assert_eq!(addr, "bc1py4x88fznv4y8er4r3cdtt74jvt8wmm9e9ntvlk3e8yxe2jtzhjrqdtqep9");
}

#[wasm_bindgen_test]
fn test_get_taproot_address_invalid_internal_pk() {
    setup_panic_hook();
    let internal_pk_bytes = vec![1, 2, 3];
    let result = get_taproot_address(internal_pk_bytes, JsValue::NULL, "mainnet".to_string());
    assert!(result.is_err());
    assert!(get_jsvalue_error_string(result.err().unwrap()).contains("Invalid internal public key"));
}


