use bech32::FromBase32;
use lightning::offers::offer::Amount;
use lightning::util::ser::Writeable;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

const BECH32_BOLT12_INVOICE_HRP: &str = "lni";

#[wasm_bindgen(inspectable)]
pub struct Offer {
    offer: lightning::offers::offer::Offer,
}

#[wasm_bindgen]
impl Offer {
    #[wasm_bindgen(constructor)]
    pub fn new(offer: &str) -> Result<Offer, String> {
        let offer = match offer.parse::<lightning::offers::offer::Offer>() {
            Ok(res) => res,
            Err(err) => return Err(format!("{:?}", err)),
        };
        Ok(Offer { offer })
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> Vec<u8> {
        Vec::from(self.offer.id().0)
    }

    #[wasm_bindgen(getter)]
    pub fn signing_pubkey(&self) -> Option<Vec<u8>> {
        self.offer.signing_pubkey().map(|key| key.encode())
    }

    /// The minimum amount required for a successful payment of a single item
    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> Option<u64> {
        match self.offer.amount() {
            Some(amount) => match amount {
                Amount::Bitcoin { amount_msats } => Some(amount_msats),
                Amount::Currency { .. } => None,
            },
            None => None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> Option<String> {
        self.offer.description().map(|s| s.to_string())
    }
}

#[wasm_bindgen(inspectable)]
pub struct Invoice {
    invoice: lightning::offers::invoice::Bolt12Invoice,
}

#[wasm_bindgen]
impl Invoice {
    #[wasm_bindgen(constructor)]
    pub fn new(invoice: &str) -> Result<Invoice, String> {
        let (hrp, data) = match bech32::decode_without_checksum(invoice) {
            Ok(res) => res,
            Err(err) => return Err(format!("{:?}", err)),
        };
        if hrp != BECH32_BOLT12_INVOICE_HRP {
            return Err("invalid HRP".into());
        }

        let data = match Vec::<u8>::from_base32(&data) {
            Ok(res) => res,
            Err(err) => return Err(format!("{:?}", err)),
        };
        match lightning::offers::invoice::Bolt12Invoice::try_from(data) {
            Ok(invoice) => Ok(Invoice { invoice }),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn signing_pubkey(&self) -> Vec<u8> {
        self.invoice.signing_pubkey().encode()
    }

    #[wasm_bindgen(getter)]
    pub fn amount_msat(&self) -> u64 {
        self.invoice.amount_msats()
    }

    #[wasm_bindgen(getter)]
    pub fn payment_hash(&self) -> Vec<u8> {
        self.invoice.payment_hash().encode()
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> Option<String> {
        self.invoice.description().map(|s| s.to_string())
    }
}
