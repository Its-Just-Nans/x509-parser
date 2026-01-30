//! this file is used to test the time crate with wasm
//!
//! ```sh
//! cargo add --dev wasm-bindgen-test
//! wasm-pack test --node
//! ```
//!

#![cfg(target_arch = "wasm32")]

use x509_parser::pem::{parse_x509_pem, Pem};
use x509_parser::{parse_x509_certificate, x509::X509Version};

static IGCA_PEM: &[u8] = include_bytes!("../assets/IGC_A.pem");


#[wasm_bindgen_test::wasm_bindgen_test]
fn test_x509_parse_pem() {
    let pem: Vec<Pem> = Pem::iter_from_buffer(IGCA_PEM)
        .collect::<Result<_, _>>()
        .ok()
        .unwrap();
    let x509 = pem[0].parse_x509().unwrap();
    assert_eq!(x509.validity.is_valid(), true);
}