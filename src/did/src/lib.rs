use candid::{CandidType, Decode, Encode};
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct State {
    pub name: String,
    pub value: u64,
}

impl Storable for State {
    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        Encode!(self).unwrap().into()
    }

    fn into_bytes(self) -> Vec<u8> {
        Encode!(&self).unwrap()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}
