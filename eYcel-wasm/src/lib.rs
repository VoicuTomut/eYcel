pub mod cell_value;
pub mod column_analyzer;
pub mod decrypt;
pub mod encrypt;
pub mod errors;
pub mod formula_handler;
pub mod transformations;
pub mod xlsx_io;
pub mod yaml_handler;

use wasm_bindgen::prelude::*;

/// Initialize panic hook so WASM panics show real errors in the browser console.
fn init_panic_hook() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

/// Encrypt a spreadsheet file (xlsx, xls, or csv).
/// The filename is used to auto-detect the input format.
/// Output is always xlsx.
///
/// Returns a JS object with:
///   - encrypted_bytes: Uint8Array (xlsx)
///   - rules_yaml: String
#[wasm_bindgen]
pub fn encrypt(
    file_bytes: &[u8],
    filename: &str,
    scramble_numbers: bool,
    scramble_dates: bool,
) -> Result<JsValue, JsError> {
    init_panic_hook();

    let options = encrypt::EncryptOptions {
        scramble_numbers,
        scramble_dates,
    };
    let (encrypted_bytes, rules_yaml) =
        encrypt::encrypt_excel_with_options(file_bytes, filename, None, &options)
            .map_err(|e| JsError::new(&e.to_string()))?;

    let obj = js_sys::Object::new();
    let arr = js_sys::Uint8Array::from(encrypted_bytes.as_slice());
    js_sys::Reflect::set(&obj, &"encrypted_bytes".into(), &arr)
        .map_err(|_| JsError::new("Failed to set encrypted_bytes"))?;
    js_sys::Reflect::set(&obj, &"rules_yaml".into(), &JsValue::from_str(&rules_yaml))
        .map_err(|_| JsError::new("Failed to set rules_yaml"))?;

    Ok(obj.into())
}

/// Decrypt an encrypted file using rules YAML.
/// Accepts xlsx, xls, or csv. The filename parameter helps detect the format.
/// Returns decrypted xlsx as Uint8Array.
#[wasm_bindgen]
pub fn decrypt(file_bytes: &[u8], rules_yaml: &str, filename: &str) -> Result<js_sys::Uint8Array, JsError> {
    init_panic_hook();

    let decrypted = decrypt::decrypt_excel(file_bytes, rules_yaml, filename)
        .map_err(|e| JsError::new(&e.to_string()))?;

    Ok(js_sys::Uint8Array::from(decrypted.as_slice()))
}

/// Decrypt an encrypted file and return as CSV.
/// Returns CSV bytes as Uint8Array (first sheet only).
#[wasm_bindgen]
pub fn decrypt_as_csv(file_bytes: &[u8], rules_yaml: &str, filename: &str) -> Result<js_sys::Uint8Array, JsError> {
    init_panic_hook();

    let decrypted = decrypt::decrypt_excel_as_csv(file_bytes, rules_yaml, filename)
        .map_err(|e| JsError::new(&e.to_string()))?;

    Ok(js_sys::Uint8Array::from(decrypted.as_slice()))
}

/// Validate rules YAML. Returns a JS object with:
///   - valid: bool
///   - errors: string[]
///   - columns: object (column name → transform type)
#[wasm_bindgen]
pub fn validate_rules(rules_yaml: &str) -> Result<JsValue, JsError> {
    init_panic_hook();

    let obj = js_sys::Object::new();

    match yaml_handler::rules_from_yaml(rules_yaml) {
        Ok(rules) => {
            js_sys::Reflect::set(&obj, &"valid".into(), &JsValue::TRUE)
                .map_err(|_| JsError::new("reflect error"))?;
            js_sys::Reflect::set(
                &obj,
                &"errors".into(),
                &js_sys::Array::new().into(),
            )
            .map_err(|_| JsError::new("reflect error"))?;

            let cols = js_sys::Object::new();
            for (name, cfg) in &rules.columns {
                js_sys::Reflect::set(
                    &cols,
                    &JsValue::from_str(name),
                    &JsValue::from_str(&cfg.transform),
                )
                .map_err(|_| JsError::new("reflect error"))?;
            }
            js_sys::Reflect::set(&obj, &"columns".into(), &cols)
                .map_err(|_| JsError::new("reflect error"))?;
        }
        Err(err_msg) => {
            js_sys::Reflect::set(&obj, &"valid".into(), &JsValue::FALSE)
                .map_err(|_| JsError::new("reflect error"))?;
            let errors = js_sys::Array::new();
            errors.push(&JsValue::from_str(&err_msg));
            js_sys::Reflect::set(&obj, &"errors".into(), &errors.into())
                .map_err(|_| JsError::new("reflect error"))?;
        }
    }

    Ok(obj.into())
}
