use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;
use anyhow::{Error, Result};

use crate::{FINISH_RECOVERY_FUNCTION, RECOVERY_CONTINUATION_PARAM, util::{Abbrev, FlankedFunction}};

//
// Request
//

#[derive(Debug, Clone)]
pub struct FinishRecoveryRequest {
    id: ARID,
    key: PublicKeyBase,
    continuation: Envelope,
}

impl FinishRecoveryRequest {
    pub fn from_fields(id: ARID, key: PublicKeyBase, continuation: Envelope) -> Self {
        Self {
            id,
            key,
            continuation,
        }
    }

    pub fn new(key: impl AsRef<PublicKeyBase>, continuation: Envelope) -> Self {
        Self::from_fields(
            ARID::new(),
            key.as_ref().clone(),
            continuation,
        )
    }

    pub fn from_body(id: ARID, key: PublicKeyBase, body: Envelope) -> Result<Self> {
        let continuation = body.object_for_parameter(RECOVERY_CONTINUATION_PARAM)?;
        Ok(Self::from_fields(id, key, continuation))
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }

    pub fn key(&self) -> &PublicKeyBase {
        &self.key
    }

    pub fn continuation(&self) -> &Envelope {
        &self.continuation
    }
}

impl From<FinishRecoveryRequest> for Envelope {
    fn from(value: FinishRecoveryRequest) -> Self {
        Envelope::new_function(FINISH_RECOVERY_FUNCTION)
            .add_parameter(RECOVERY_CONTINUATION_PARAM, &value.continuation)
            .into_transaction_request(value.id(), &value.key)
    }
}

impl TryFrom<Envelope> for FinishRecoveryRequest {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        let (id, key, body, _) = envelope.parse_transaction_request(Some(&FINISH_RECOVERY_FUNCTION))?;
        Self::from_body(id, key, body)
    }
}

impl PartialEq for FinishRecoveryRequest {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.key == other.key
            && self
                .continuation
                .is_identical_to(&other.continuation)
    }
}

impl Eq for FinishRecoveryRequest {}

impl std::fmt::Display for FinishRecoveryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} continuation {} for key {}",
            self.id().abbrev(),
            "finishRecovery".flanked_function(),
            self.continuation().abbrev(),
            self.key().abbrev()
        ))
    }
}

#[cfg(test)]
mod tests {
    use bc_components::PrivateKeyBase;
    use indoc::indoc;

    use super::*;

    fn id() -> ARID {
        ARID::from_data_ref(hex_literal::hex!(
            "8712dfac3d0ebfa910736b2a9ee39d4b68f64222a77bcc0074f3f5f1c9216d30"
        ))
        .unwrap()
    }

    #[test]
    fn test_request() {
        let private_key = PrivateKeyBase::new();
        let key = private_key.public_key();

        let continuation = Envelope::new("Continuation");

        let request = FinishRecoveryRequest::from_fields(id(), key, continuation);
        let request_envelope = request.to_envelope();
        assert_eq!(
            request_envelope.format(),
            indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"finishRecovery"» [
                ❰"recoveryContinuation"❱: "Continuation"
            ]
            'senderPublicKey': PublicKeyBase
        ]
        "#}
            .trim()
        );
        let decoded = request_envelope.try_into().unwrap();
        assert_eq!(request, decoded);
    }
}
