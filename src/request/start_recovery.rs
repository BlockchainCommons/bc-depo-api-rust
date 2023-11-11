use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;

use crate::{START_RECOVERY_FUNCTION, RECOVERY_METHOD_PARAM, NEW_KEY_PARAM};

use super::{request_body, request_envelope, parse_request, response_envelope, parse_response};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartRecoveryRequest {
    id: ARID,
    key: PublicKeyBase,
    recovery: String,
    new_key: PublicKeyBase,
}

impl StartRecoveryRequest {
    pub fn new(
        id: ARID,
        key: PublicKeyBase,
        recovery: String,
        new_key: PublicKeyBase,
    ) -> Self {
        Self {
            id,
            key,
            recovery,
            new_key,
        }
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }

    pub fn key(&self) -> &PublicKeyBase {
        &self.key
    }

    pub fn recovery(&self) -> &str {
        self.recovery.as_ref()
    }

    pub fn new_key(&self) -> &PublicKeyBase {
        &self.new_key
    }
}

impl EnvelopeEncodable for StartRecoveryRequest {
    fn envelope(self) -> Envelope {
        let body = request_body(START_RECOVERY_FUNCTION, self.key)
            .add_parameter(RECOVERY_METHOD_PARAM, self.recovery)
            .add_parameter(NEW_KEY_PARAM, self.new_key);
        request_envelope(self.id, body)
    }
}

impl From<StartRecoveryRequest> for Envelope {
    fn from(value: StartRecoveryRequest) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for StartRecoveryRequest {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, key, body) = parse_request(START_RECOVERY_FUNCTION, envelope)?;
        let recovery: String = body.extract_object_for_parameter(RECOVERY_METHOD_PARAM)?;
        let new_key: PublicKeyBase = body.extract_object_for_parameter(NEW_KEY_PARAM)?;
        Ok(Self::new(id, key, recovery, new_key))
    }
}

impl TryFrom<Envelope> for StartRecoveryRequest {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for StartRecoveryRequest {}

//
// Response
//


#[derive(Debug, Clone)]
pub struct StartRecoveryResponse {
    id: ARID,
    continuation: Envelope,
}

impl StartRecoveryResponse {
    pub fn new(
        id: ARID,
        continuation: Envelope,
    ) -> Self {
        Self {
            id,
            continuation,
        }
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }

    pub fn continuation(&self) -> &Envelope {
        &self.continuation
    }
}

impl EnvelopeEncodable for StartRecoveryResponse {
    fn envelope(self) -> Envelope {
        response_envelope(self.id, Some(self.continuation))
    }
}

impl From<StartRecoveryResponse> for Envelope {
    fn from(value: StartRecoveryResponse) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for StartRecoveryResponse {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, continuation) = parse_response(envelope)?;
        Ok(Self::new(id, continuation))
    }
}

impl TryFrom<Envelope> for StartRecoveryResponse {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for StartRecoveryResponse {}

impl PartialEq for StartRecoveryResponse {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.continuation.is_identical_to(other.continuation.clone())
    }
}

impl Eq for StartRecoveryResponse {}

#[cfg(test)]
mod tests {
    use bc_components::PrivateKeyBase;
    use indoc::indoc;

    use super::*;

    fn id() -> ARID {
        ARID::from_data_ref(hex_literal::hex!("8712dfac3d0ebfa910736b2a9ee39d4b68f64222a77bcc0074f3f5f1c9216d30")).unwrap()
    }

    #[test]
    fn test_request() {
        let private_key = PrivateKeyBase::new();
        let key = private_key.public_keys();

        let recovery = "recovery".to_string();
        let new_key = PrivateKeyBase::new().public_keys();

        let request = StartRecoveryRequest::new(id(), key, recovery, new_key);
        let request_envelope = request.clone().envelope();
        assert_eq!(request_envelope.format(),
        indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"startRecovery"» [
                ❰"key"❱: PublicKeyBase
                ❰"newKey"❱: PublicKeyBase
                ❰"recoveryMethod"❱: "recovery"
            ]
        ]
        "#}.trim()
        );
        let decoded = StartRecoveryRequest::try_from(request_envelope).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let continuation = "continuation";
        let response = StartRecoveryResponse::new(id(), continuation.envelope());
        let response_envelope = response.clone().envelope();
        assert_eq!(response_envelope.format(),
        indoc! {r#"
        response(ARID(8712dfac)) [
            'result': "continuation"
        ]
        "#}.trim()
        );
        let decoded = StartRecoveryResponse::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);
    }
}
