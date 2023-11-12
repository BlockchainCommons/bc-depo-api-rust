use bc_components::{ARID, PublicKeyBase};
use bc_envelope::prelude::*;

use crate::KEY_PARAM;

pub mod delete_account;
pub use delete_account::{DeleteAccountRequest, DeleteAccountResponse};

pub mod get_recovery;
pub use get_recovery::{GetRecoveryRequest, GetRecoveryResponse};

pub mod delete_shares;
pub use delete_shares::{DeleteSharesRequest, DeleteSharesResponse};

pub mod finish_recovery;
pub use finish_recovery::{FinishRecoveryRequest, FinishRecoveryResponse};

pub mod get_shares;
pub use get_shares::{GetSharesRequest, GetSharesResponse};

pub mod start_recovery;
pub use start_recovery::{StartRecoveryRequest, StartRecoveryResponse};

pub mod store_share;
pub use store_share::{StoreShareRequest, StoreShareResponse};

pub mod update_key;
pub use update_key::{UpdateKeyRequest, UpdateKeyResponse};

pub mod update_recovery;
pub use update_recovery::{UpdateRecoveryRequest, UpdateRecoveryResponse};

fn request_body(function: Function, key: PublicKeyBase) -> Envelope {
    Envelope::new(function)
        .add_parameter(KEY_PARAM, key)
}

fn request_envelope(id: ARID, body: Envelope) -> Envelope {
    Envelope::new_request(id, body)
}

pub fn parse_request(function: Function, envelope: Envelope) -> anyhow::Result<(ARID, PublicKeyBase, Envelope)> {
    let id = envelope.request_id()?;
    let body = envelope.request_body()?;
    body.check_function(&function)?;
    let key: PublicKeyBase = body.extract_object_for_parameter(KEY_PARAM)?;
    Ok((id, key, body))
}

pub fn response_envelope(id: ARID, result: Option<Envelope>) -> Envelope {
    let result = result.unwrap_or(known_values::OK_VALUE.envelope());
    Envelope::new_response(id, result)
}

pub fn parse_response(envelope: Envelope) -> anyhow::Result<(ARID, Envelope)> {
    let id = envelope.response_id()?;
    let result = envelope.result()?;
    Ok((id, result))
}
