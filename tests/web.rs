#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use boltz_bolt12::{Invoice, Offer};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_decode_offer() {
    let offer = Offer::new(
        "lno1qgsqvgnwgcg35z6ee2h3yczraddm72xrfua9uve2rlrm9deu7xyfzrc2q36x2um5zcssyeyreggqmet8r4k6krvd3knppsx6c8v5g7tj8hcuq8lleta9ve5n",
    ).unwrap();
    assert_eq!(
        hex::encode(offer.id()),
        "1576a2882571f73cfee4a3856c78650f8f231bdaf43de88e1914b663a2de38ec"
    );
    assert_eq!(
        hex::encode(offer.signing_pubkey().unwrap()),
        "026483ca100de5671d6dab0d8d8da610c0dac1d94479723df1c01fffcafa566693"
    );
    assert_eq!(offer.amount(), None);
    assert_eq!(offer.description(), Some("test".to_string()));
}

#[wasm_bindgen_test]
fn test_decode_offer_error() {
    assert!(Offer::new("invalid").is_err());
}

#[wasm_bindgen_test]
fn test_decode_invoice() {
    let invoice = Invoice::new(
        "lni1qqgppx633kdjzjh5ehq5g2gx52fuvq3qqc3xu3s3rg94nj40zfsy866mhu5vxne6tcej5878k2mneuvgjy8s5pr5v4ehg93pqfjg8jssphjkw8td4vxcmrdxzrqd4sweg3uhy003cq0lljh62enfx5pqqc3xu3s3rg94nj40zfsy866mhu5vxne6tcej5878k2mneuvgjy84yq38zpvzzqnlzah7z7r6v4jhmlq5ypqhzkx58qs32ha5jpft9qnnpk5s50lpv6sfsqnys09pqr09vuwkm2cd3kx6vyxqmtqaj3rewg7lrsqlll9054nxjvp2dswdnvddqy0k8d3exfc6nuaa99myv4u4wktjljp0dxxmhs0falgpq2qagqd8w043aw8tvlv48ddsdz344qtf25a03nwsx6lpcj5ezskzwqpjdpwhn2uy9n56md6ft9f45c4c9ssyn74jwkj2nfmtf5qnrtde6c5qjjef6s0ycsd29587dexdvp40rmyc5gwqqqqqqqqqqqqqqq9qqqqqqqqqqqqqr5jt9hav2gqqqqqq5szxdccudz5zqtvc9wzw5qphzf52zfkhsshgwgr8uw48lm7zngxg9cg6q3vc5cug4gpzwy9syypxfq72zqx72ecadk4smrvd5cgvpkkpm9z8ju3a78qpll72lftxdylsgpk9x3smsygvz0tfxzfjypckktqpymzftdr26f3yq6fl87mstdku9kcn226efwrwgqy2fvrf85svp8mlfae2se0kmtvwtw69c286hg23",
    ).unwrap();
    assert_eq!(
        hex::encode(invoice.signing_pubkey()),
        "026483ca100de5671d6dab0d8d8da610c0dac1d94479723df1c01fffcafa566693"
    );
    assert_eq!(invoice.amount_msat(), 10_000);
    assert_eq!(
        hex::encode(invoice.payment_hash()),
        "2d982b84ea00371268a126d7842e872067e3aa7fefc29a0c82e11a04598a6388"
    );
    assert_eq!(invoice.description(), Some("test".to_string()));
}

#[wasm_bindgen_test]
fn test_decode_invoice_error() {
    assert!(Invoice::new("invalid").is_err());
}
