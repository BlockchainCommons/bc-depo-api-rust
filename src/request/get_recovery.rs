use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;

use crate::GET_RECOVERY_FUNCTION;

use super::{parse_request, parse_response, request_body, request_envelope, response_envelope};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetRecoveryRequest {
    id: ARID,
    key: PublicKeyBase,
}

impl GetRecoveryRequest {
    pub fn new(key: impl AsRef<PublicKeyBase>) -> Self {
        Self::new_opt(ARID::new(), key.as_ref().clone())
    }

    pub fn new_opt(id: ARID, key: PublicKeyBase) -> Self {
        Self { id, key }
    }

    pub fn id(&self) -> &ARID {
        &self.id
    }

    pub fn key(&self) -> &PublicKeyBase {
        &self.key
    }
}

impl EnvelopeEncodable for GetRecoveryRequest {
    fn envelope(self) -> Envelope {
        request_envelope(self.id, request_body(GET_RECOVERY_FUNCTION, self.key))
    }
}

impl From<GetRecoveryRequest> for Envelope {
    fn from(value: GetRecoveryRequest) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for GetRecoveryRequest {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, key, _body) = parse_request(GET_RECOVERY_FUNCTION, envelope)?;
        Ok(Self::new_opt(id, key))
    }
}

impl TryFrom<Envelope> for GetRecoveryRequest {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for GetRecoveryRequest {}

//
// Response
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetRecoveryResponse {
    id: ARID,
    recovery: Option<String>,
}

impl GetRecoveryResponse {
    pub fn new(id: ARID, recovery: Option<String>) -> Self {
        Self { id, recovery }
    }

    pub fn id(&self) -> &ARID {
        &self.id
    }

    pub fn recovery(&self) -> Option<&str> {
        self.recovery.as_deref()
    }
}

impl EnvelopeEncodable for GetRecoveryResponse {
    fn envelope(self) -> Envelope {
        let result: Envelope = if let Some(recovery) = self.recovery {
            recovery.into()
        } else {
            Envelope::null()
        };
        response_envelope(self.id, Some(result))
    }
}

impl From<GetRecoveryResponse> for Envelope {
    fn from(value: GetRecoveryResponse) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for GetRecoveryResponse {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, result) = parse_response(envelope.clone())?;
        let recovery = if result.is_null() {
            None
        } else {
            Some(result.extract_subject()?)
        };
        Ok(Self::new(id, recovery))
    }
}

impl TryFrom<Envelope> for GetRecoveryResponse {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for GetRecoveryResponse {}

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

        let request = GetRecoveryRequest::new_opt(id(), key);
        let request_envelope = request.clone().envelope();
        assert_eq!(
            request_envelope.format(),
            indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"getRecovery"» [
                ❰"key"❱: PublicKeyBase
            ]
        ]
        "#}
            .trim()
        );
        let decoded = GetRecoveryRequest::try_from(request_envelope).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let response = GetRecoveryResponse::new(id(), Some("Recovery Method".into()));
        let response_envelope = response.clone().envelope();
        assert_eq!(
            response_envelope.format(),
            indoc! {r#"
        response(ARID(8712dfac)) [
            'result': "Recovery Method"
        ]
        "#}
            .trim()
        );
        let decoded = GetRecoveryResponse::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);

        let response = GetRecoveryResponse::new(id(), None);
        let response_envelope = response.clone().envelope();
        assert_eq!(
            response_envelope.format(),
            indoc! {r#"
        response(ARID(8712dfac)) [
            'result': null
        ]
        "#}
            .trim()
        );
        let decoded = GetRecoveryResponse::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);
    }
}
