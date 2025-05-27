use bitcoin::consensus::{deserialize as consensus_deserialize, serialize as consensus_serialize};
use bitcoin::psbt::{Input, Psbt};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::sighash::EcdsaSighashType;
use bitcoin::Network; // May not be needed if derived from PSBT
use bitcoin::PrivateKey;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sign_psbt(
    psbt_bytes: Vec<u8>,
    input_index: u32,
    private_key_wif: &str,
    sighash_type_num: Option<u32>,
) -> Result<Vec<u8>, JsValue> {
    let mut psbt: Psbt = match Psbt::deserialize(&psbt_bytes) {
        Ok(p) => p,
        Err(e) => {
            return Err(JsValue::from_str(&format!(
                "Failed to deserialize PSBT: {}",
                e
            )))
        }
    };

    let _secp = Secp256k1::new();
    let _private_key = match PrivateKey::from_wif(private_key_wif) {
        Ok(k) => k,
        Err(e) => {
            return Err(JsValue::from_str(&format!(
                "Invalid private key WIF: {}",
                e
            )))
        }
    };

    let _sighash_type = match sighash_type_num {
        Some(num) => EcdsaSighashType::from_consensus(num),
        None => EcdsaSighashType::All,
    };

    if input_index as usize >= psbt.inputs.len() {
        return Err(JsValue::from_str("Input index out of bounds"));
    }

    Err(JsValue::from_str("sign_psbt for specific input is not fully implemented and requires detailed sighash logic for bitcoin 0.32.6."))
}

#[wasm_bindgen]
pub fn finalize_psbt(psbt_bytes: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    let mut psbt: Psbt = match Psbt::deserialize(&psbt_bytes) {
        Ok(p) => p,
        Err(e) => {
            return Err(JsValue::from_str(&format!(
                "Failed to deserialize PSBT for finalization: {}",
                e
            )))
        }
    };

    let secp = Secp256k1::new();
    for input in psbt.inputs.iter_mut() {
        if input.final_script_sig.is_none() && input.final_script_witness.is_none() {
            return Err(JsValue::from_str(
                "Input is not finalized. Please provide the required signatures.",
            ));
        }
    }
    Ok(psbt.serialize())
}

#[wasm_bindgen]
pub fn extract_transaction_from_psbt(psbt_bytes: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    let psbt: Psbt = match Psbt::deserialize(&psbt_bytes) {
        Ok(p) => p,
        Err(e) => {
            return Err(JsValue::from_str(&format!(
                "Failed to deserialize PSBT for extraction: {}",
                e
            )))
        }
    };

    let tx = match psbt.extract_tx() {
        Ok(t) => t,
        Err(e) => {
            return Err(JsValue::from_str(&format!(
                "Failed to extract transaction from PSBT: {:?}",
                e
            )))
        }
    };

    Ok(consensus_serialize(&tx))
}
