#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use bolt12;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_decode_offer() {
    let offer = bolt12::Offer::new(
        "lno1pqqhkzsyv9ekgeskyyprc3k608wa39ykt8nsw76mpfr2m4d869fl9wfmh8v26rs8a2gzk2s",
    )
    .unwrap();
    assert_eq!(offer.amount_msat, Some(123));
    assert_eq!(offer.issuer(), None);
    assert_eq!(
        offer.signing_pubkey(),
        Some(vec![
            2, 60, 70, 218, 121, 221, 216, 148, 150, 89, 231, 7, 123, 91, 10, 70, 173, 213, 167,
            209, 83, 242, 185, 59, 185, 216, 173, 14, 7, 234, 144, 43, 42
        ])
    );
}

#[wasm_bindgen_test]
fn test_decode_offer_blinded_routes() {
    let offer = bolt12::Offer::new("lno1zrxq8pjw7qjlm68mtp7e3yvxee4y5xrgjhhyf2fxhlphpckrvevh50u0qtuzuc7vrxw3ptvmvzs7lqmyp94rhmfdg6gpef65rs2xxl2mzr2ksqszz0xxdd9y6653lwugyjqg6nwjerz4wcdskllmkp9fl4kp4d5z0hfqqvlxj7wm32qk0pavw6cr3kfn6unhzy9r25vlp4nmmy0gfck4ur7qk0xz43lhmnlx59qfxmdh6ue6y65s7rw4qdtdltt5gzratf3hnr668laqtcrk5tyzfz8a8ckq4dlpd40alv646qqsf6pgykhxlayyswqmjffpgehjjs").unwrap();
    assert_eq!(offer.amount_msat, None);
    assert_eq!(offer.issuer(), None);
    assert_eq!(offer.signing_pubkey(), None);
}
