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
fn test_decode_offer_paths() {
    let offer = Offer::new(
        "lno1zrxq8pjw7qjlm68mtp7e3yvxee4y5xrgjhhyf2fxhlphpckrvevh50u0qtuzuc7vrxw3ptvmvzs7lqmyp94rhmfdg6gpef65rs2xxl2mzr2ksqszz0xxdd9y6653lwugyjqg6nwjerz4wcdskllmkp9fl4kp4d5z0hfqqvlxj7wm32qk0pavw6cr3kfn6unhzy9r25vlp4nmmy0gfck4ur7qk0xz43lhmnlx59qfxmdh6ue6y65s7rw4qdtdltt5gzratf3hnr668laqtcrk5tyzfz8a8ckq4dlpd40alv646qqsf6pgykhxlayyswqmjffpgehjjs"
    ).unwrap();
    let paths = offer.paths();
    assert_eq!(paths.len(), 1);

    let hops = paths[0].hops();
    let last_hop = hops.last().unwrap();
    assert_eq!(
        hex::encode(last_hop.pubkey()),
        "0356dfad744087d5a63798f5a3ffa05e076a2c82488fd3e2c0ab7e16d5fdfb355d"
    );
}

#[wasm_bindgen_test]
fn test_decode_offer_scid_paths() {
    let offer = Offer::new(
        "lno1pgx9getnwss8vetrw3hhyucs3yqqqqqqqqqqqqp2qgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqqyqqqqqqqqqqqqqqqqqqqqqqqqqqqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqqgzyg3zyg3zyg3z93pqthvwfzadd7jejes8q9lhc4rvjxd022zv5l44g6qah82ru5rdpnpj"
    ).unwrap();
    let paths = offer.paths();
    assert_eq!(paths.len(), 1);

    let hops = paths[0].hops();
    let last_hop = hops.last().unwrap();
    assert_eq!(
        hex::encode(last_hop.pubkey()),
        "020202020202020202020202020202020202020202020202020202020202020202"
    );
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
fn test_decode_invoice_paths() {
    let invoice = Invoice::new(
        "lni1qqg0u9j0r86j5k5ejghfv8km59j7jyxvqwryaup9lh50kkranzgcdnn2fgvx390wgj5jd07rwr3vxeje0glc7qhc9e3ucxvazzkekc9pa7pkgzt280kj635srjn4g8q5vd74kyx4dqpqyy7vv662f44fr7acsfyq34xa9jx92asmpdllhvz2nltvr2mgylwjqqe7d9uahz5pv7r6ca4s8rvn84e8wyg2x4ge7rt8hkg7sn3dtc8upv7v9trl0h87dg2qjdkm04en5f4fpuxa2q6km7khgsy86knr0x8450l6qhs8dgkgyjy0603vp2m7zm2lm7e4t5qpqn5zsfdwdl6gfquphyjjz3n099zjqgp7skppq0x6py8g70pn6692tqke75e6y0wy9ejlc873vc6n4djzcsljr4jneg8aqxdq8pjw7qjlm68mtp7e3yvxee4y5xrgjhhyf2fxhlphpckrvevh50u0qts9zu98ca0z62ahjsh0v85a9ukaqyuev9u20gwk3mnwvh8hunugqqszld4aksgef0rnduusxfzauvynr3l9wp7lztth5hd9qtwj5vzm9xlsq3gcvvahd6x0a2cw529vyl0ukdwdg25wu0rcqmurp5wc55f44jjny5v6tlrls052ss58r490q4ds50f3cmzj2qqf4773q6eg5gfptvq4vlkfq2n9qq7xl53kta005252fnsfu2c383hjjysutxe3qdqq8zjl0ud8m5cskyqvch5net7dpawa7sjr5x8qu26t529hnnelvf70tmgxnj4umkfcncn7aaphtej4anf7r83qkkfdfasy09g2ha0wlct6t30w9qll0ak2ad6qjjztxz82h2he04pd6ulynyjyd9wlt5et5v4vmkdh9ky24wr6pyh280jrgc5twg9l2nvgm4nr6xawhaef92dhnmu4d3e0cfxsh0lx7kvhsjtz57lgr57elw9w9ndtjtl6xtzarxw6u3w98cpef7mzrneenq0vuj6345skqm88a0xkfpthgwdfc9ux4z79l48nspvfuf728uu48dt5rdfrsgpevg8c7d56y8qqqqp7sqqqqpjqzgqqqqqqqqqqq05qqqqqqqqqqp7sqqq2gpr8zlx8dfsrq9gcp2pq3djq6vxmenenj4g2t9qwzjvrywflec7gkkz0mx7zd59na6zxvkw25qsrazhqxqsqqzczzq6km7khgsy86knr0x8450l6qhs8dgkgyjy0603vp2m7zm2lm7e4thcyppr70nsjasuspk3ne7p72z33rdkz87507jdl6qrww9d3ye9zae45cxtnuf7h2tkk0stdx6k9s29h5z4veh8ptaptjzw6lmtr3ge6ncts"
    ).unwrap();
    assert_eq!(
        hex::encode(invoice.signing_pubkey()),
        "0356dfad744087d5a63798f5a3ffa05e076a2c82488fd3e2c0ab7e16d5fdfb355d"
    );

    let paths = invoice.message_paths();
    assert_eq!(paths.len(), 1);

    let hops = paths[0].hops();
    let last_hop = hops.last().unwrap();
    assert_eq!(
        hex::encode(last_hop.pubkey()),
        "0356dfad744087d5a63798f5a3ffa05e076a2c82488fd3e2c0ab7e16d5fdfb355d"
    );

    let paths = invoice.payment_paths();
    assert_eq!(paths.len(), 1);

    let hops = paths[0].hops();
    let last_hop = hops.last().unwrap();
    assert_eq!(
        hex::encode(last_hop.pubkey()),
        "03c6fd2365f5efa2a8a4ce09e2b113c6f29121c59b310340038a5f7f1a7dd310b1"
    );
}

#[wasm_bindgen_test]
fn test_decode_invoice_error() {
    assert!(Invoice::new("invalid").is_err());
}
