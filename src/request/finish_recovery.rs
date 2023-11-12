use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;

use crate::{FINISH_RECOVERY_FUNCTION, RECOVERY_CONTINUATION_PARAM};

use super::{parse_response, request_body, request_envelope, response_envelope};

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
    pub fn new(key: impl AsRef<PublicKeyBase>, continuation: impl AsRef<Envelope>) -> Self {
        Self::new_opt(
            ARID::new(),
            key.as_ref().clone(),
            continuation.as_ref().clone(),
        )
    }

    pub fn new_opt(id: ARID, key: PublicKeyBase, continuation: Envelope) -> Self {
        Self {
            id,
            key,
            continuation,
        }
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

impl EnvelopeEncodable for FinishRecoveryRequest {
    fn envelope(self) -> Envelope {
        let body = request_body(FINISH_RECOVERY_FUNCTION, self.key)
            .add_parameter(RECOVERY_CONTINUATION_PARAM, self.continuation);
        request_envelope(self.id, body)
    }
}

impl From<FinishRecoveryRequest> for Envelope {
    fn from(value: FinishRecoveryRequest) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for FinishRecoveryRequest {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, key, body) = super::parse_request(FINISH_RECOVERY_FUNCTION, envelope)?;
        let continuation = body.object_for_parameter(RECOVERY_CONTINUATION_PARAM)?;
        Ok(Self::new_opt(id, key, continuation))
    }
}

impl TryFrom<Envelope> for FinishRecoveryRequest {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for FinishRecoveryRequest {}

impl PartialEq for FinishRecoveryRequest {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.key == other.key
            && self
                .continuation
                .is_identical_to(other.continuation.clone())
    }
}

impl Eq for FinishRecoveryRequest {}

//
// Response
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinishRecoveryResponse {
    id: ARID,
}

impl FinishRecoveryResponse {
    pub fn new(id: ARID) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }
}

impl EnvelopeEncodable for FinishRecoveryResponse {
    fn envelope(self) -> Envelope {
        response_envelope(self.id, None)
    }
}

impl From<FinishRecoveryResponse> for Envelope {
    fn from(value: FinishRecoveryResponse) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for FinishRecoveryResponse {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, _result) = parse_response(envelope)?;
        Ok(Self::new(id))
    }
}

impl TryFrom<Envelope> for FinishRecoveryResponse {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for FinishRecoveryResponse {}

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
        let key = private_key.public_keys();

        let continuation = Envelope::new("Continuation");

        let request = FinishRecoveryRequest::new_opt(id(), key, continuation);
        let request_envelope = request.clone().envelope();
        assert_eq!(
            request_envelope.format(),
            indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"finishRecovery"» [
                ❰"key"❱: PublicKeyBase
                ❰"recoveryContinuation"❱: "Continuation"
            ]
        ]
        "#}
            .trim()
        );
        let decoded = FinishRecoveryRequest::try_from(request_envelope).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let response = FinishRecoveryResponse::new(id());
        let response_envelope = response.clone().envelope();
        assert_eq!(
            response_envelope.format(),
            indoc! {r#"
        response(ARID(8712dfac)) [
            'result': 'OK'
        ]
        "#}
            .trim()
        );
        let decoded = FinishRecoveryResponse::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);
    }
}
