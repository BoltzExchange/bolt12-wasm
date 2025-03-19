use lightning::bitcoin::secp256k1::PublicKey;
use lightning::blinded_path::message::BlindedMessagePath;
use lightning::blinded_path::payment::BlindedPaymentPath;
use lightning::blinded_path::{BlindedHop, Direction, IntroductionNode};
use lightning::util::ser::Writeable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inspectable)]
pub struct Hop {
    hop: BlindedHop,
}

impl Hop {
    pub fn from(hop: BlindedHop) -> Self {
        Self { hop }
    }
}

#[wasm_bindgen]
impl Hop {
    #[wasm_bindgen(getter)]
    pub fn pubkey(&self) -> Vec<u8> {
        self.hop.blinded_node_id.encode()
    }

    #[wasm_bindgen(getter)]
    pub fn encrypted_payload(&self) -> Vec<u8> {
        self.hop.encrypted_payload.clone()
    }
}

#[wasm_bindgen(inspectable)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DirectedShortChannelId {
    // The direction of the short_channel_id.
    // 0 for the lesser node id compared lexicographically in ascending order, 1 for the greater node id.
    pub direction: u8,
    pub short_channel_id: u64,
}

impl DirectedShortChannelId {
    pub fn from(direction: u8, short_channel_id: u64) -> Self {
        Self {
            direction,
            short_channel_id,
        }
    }
}

#[wasm_bindgen(inspectable)]
pub struct Path {
    introduction_node: Option<PublicKey>,
    introduction_node_short_channel_id: Option<DirectedShortChannelId>,
    blinding_point: PublicKey,
    hops: Vec<BlindedHop>,
}

impl Path {
    pub fn from_message_path(path: &BlindedMessagePath) -> Self {
        Self {
            introduction_node: match path.introduction_node() {
                IntroductionNode::NodeId(pubkey) => Some(*pubkey),
                IntroductionNode::DirectedShortChannelId(_, _) => None,
            },
            introduction_node_short_channel_id: match path.introduction_node() {
                IntroductionNode::NodeId(_) => None,
                IntroductionNode::DirectedShortChannelId(direction, short_channel_id) => {
                    match *direction {
                        Direction::NodeOne => {
                            Some(DirectedShortChannelId::from(0, *short_channel_id))
                        }
                        Direction::NodeTwo => {
                            Some(DirectedShortChannelId::from(1, *short_channel_id))
                        }
                    }
                }
            },
            blinding_point: path.blinding_point(),
            hops: path.blinded_hops().to_vec(),
        }
    }

    pub fn from_payment_path(path: &BlindedPaymentPath) -> Self {
        Self {
            introduction_node: match path.introduction_node() {
                IntroductionNode::NodeId(pubkey) => Some(*pubkey),
                IntroductionNode::DirectedShortChannelId(_, _) => None,
            },
            introduction_node_short_channel_id: match path.introduction_node() {
                IntroductionNode::NodeId(_) => None,
                IntroductionNode::DirectedShortChannelId(direction, short_channel_id) => {
                    match *direction {
                        Direction::NodeOne => {
                            Some(DirectedShortChannelId::from(0, *short_channel_id))
                        }
                        Direction::NodeTwo => {
                            Some(DirectedShortChannelId::from(1, *short_channel_id))
                        }
                    }
                }
            },
            blinding_point: path.blinding_point(),
            hops: path.blinded_hops().to_vec(),
        }
    }
}

#[wasm_bindgen]
impl Path {
    #[wasm_bindgen(getter)]
    pub fn introduction_node(&self) -> Option<Vec<u8>> {
        self.introduction_node.map(|pubkey| pubkey.encode())
    }

    #[wasm_bindgen(getter)]
    pub fn introduction_node_short_channel_id(&self) -> Option<DirectedShortChannelId> {
        self.introduction_node_short_channel_id
    }

    #[wasm_bindgen(getter)]
    pub fn blinding_point(&self) -> Vec<u8> {
        self.blinding_point.encode()
    }

    #[wasm_bindgen(getter)]
    pub fn hops(&self) -> Vec<Hop> {
        self.hops.iter().map(|hop| Hop::from(hop.clone())).collect()
    }
}
