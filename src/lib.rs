use crate::path::Path;
use bech32::primitives::decode::CheckedHrpstring;
use bech32::NoChecksum;

use lightning::bitcoin::constants::ChainHash;
use lightning::offers::offer::Amount;
use lightning::util::ser::Writeable;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

mod path;

const BECH32_BOLT12_INVOICE_HRP: &str = "lni";

#[wasm_bindgen(inspectable)]
pub struct Offer {
    offer: lightning::offers::offer::Offer,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub enum Network {
    Bitcoin,
    Testnet3,
    Testnet4,
    Signet,
    Regtest,
    Unkown,
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
        self.offer.issuer_signing_pubkey().map(|key| key.encode())
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

    #[wasm_bindgen(getter)]
    pub fn expiry(&self) -> Option<u64> {
        self.offer.absolute_expiry().map(|d| d.as_secs())
    }

    #[wasm_bindgen(getter)]
    pub fn issuer(&self) -> Option<String> {
        self.offer.issuer().map(|s| s.to_string())
    }

    #[wasm_bindgen(getter)]
    pub fn quantity(&self) -> Option<u64> {
        match self.offer.supported_quantity() {
            lightning::offers::offer::Quantity::Bounded(n) => Some(n.get()),
            lightning::offers::offer::Quantity::Unbounded => Some(0),
            lightning::offers::offer::Quantity::One => Some(1),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn chains(&self) -> Vec<Network> {
        self.offer
            .chains()
            .iter()
            .map(|chain| match *chain {
                ChainHash::BITCOIN => Network::Bitcoin,
                ChainHash::TESTNET3 => Network::Testnet3,
                ChainHash::TESTNET4 => Network::Testnet4,
                ChainHash::SIGNET => Network::Signet,
                ChainHash::REGTEST => Network::Regtest,
                _ => Network::Unkown,
            })
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn paths(&self) -> Vec<Path> {
        self.offer
            .paths()
            .iter()
            .map(Path::from_message_path)
            .collect()
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
        let p = match CheckedHrpstring::new::<NoChecksum>(invoice) {
            Ok(res) => res,
            Err(err) => return Err(format!("{:?}", err)),
        };
        if p.hrp().to_lowercase() != BECH32_BOLT12_INVOICE_HRP {
            return Err("invalid HRP".into());
        }
        let data = p.byte_iter().collect::<Vec<u8>>();
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

    #[wasm_bindgen(getter)]
    pub fn message_paths(&self) -> Vec<Path> {
        self.invoice
            .message_paths()
            .iter()
            .map(Path::from_message_path)
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn payment_paths(&self) -> Vec<Path> {
        self.invoice
            .payment_paths()
            .iter()
            .map(Path::from_payment_path)
            .collect()
    }
}
