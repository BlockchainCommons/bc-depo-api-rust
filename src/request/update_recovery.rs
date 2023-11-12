use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;

use crate::{UPDATE_RECOVERY_FUNCTION, RECOVERY_METHOD_PARAM};

use super::{request_body, request_envelope, parse_request, response_envelope, parse_response};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateRecoveryRequest {
    id: ARID,
    key: PublicKeyBase,
    recovery: Option<String>,
}

impl UpdateRecoveryRequest {
    pub fn new(
        key: impl AsRef<PublicKeyBase>,
        recovery: Option<&str>,
    ) -> Self {
        Self::new_opt(
            ARID::new(),
            key.as_ref().clone(),
            recovery.map(|s| s.to_string()),
        )
    }

    pub fn new_opt(id: ARID, key: PublicKeyBase, recovery: Option<String>) -> Self {
        Self {
            id,
            key,
            recovery,
        }
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }

    pub fn key(&self) -> &PublicKeyBase {
        &self.key
    }

    pub fn recovery(&self) -> Option<&String> {
        self.recovery.as_ref()
    }
}

impl EnvelopeEncodable for UpdateRecoveryRequest {
    fn envelope(self) -> Envelope {
        let value = if let Some(recovery) = self.recovery {
            recovery.envelope()
        } else {
            Envelope::null()
        };
        let body = request_body(UPDATE_RECOVERY_FUNCTION, self.key)
            .add_parameter(RECOVERY_METHOD_PARAM, value);
        request_envelope(self.id, body)
    }
}

impl From<UpdateRecoveryRequest> for Envelope {
    fn from(value: UpdateRecoveryRequest) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for UpdateRecoveryRequest {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, key, body) = parse_request(UPDATE_RECOVERY_FUNCTION, envelope)?;
        let recovery_envelope = body.object_for_parameter(RECOVERY_METHOD_PARAM)?;
        let recovery: Option<String> = if recovery_envelope.is_null() {
            None
        } else {
            Some(recovery_envelope.extract_subject()?)
        };
        Ok(Self::new_opt(id, key, recovery))
    }
}

impl TryFrom<Envelope> for UpdateRecoveryRequest {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for UpdateRecoveryRequest {}

//
// Response
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateRecoveryResponse {
    id: ARID,
}

impl UpdateRecoveryResponse {
    pub fn new(id: ARID) -> Self {
        Self {
            id,
        }
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }
}

impl EnvelopeEncodable for UpdateRecoveryResponse {
    fn envelope(self) -> Envelope {
        response_envelope(self.id, None)
    }
}

impl From<UpdateRecoveryResponse> for Envelope {
    fn from(value: UpdateRecoveryResponse) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for UpdateRecoveryResponse {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, _result) = parse_response(envelope)?;
        Ok(Self::new(id))
    }
}

impl TryFrom<Envelope> for UpdateRecoveryResponse {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for UpdateRecoveryResponse {}

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

        let request = UpdateRecoveryRequest::new_opt(id(), key.clone(), Some(recovery));
        let request_envelope = request.clone().envelope();
        assert_eq!(request_envelope.format(),
        indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"updateRecovery"» [
                ❰"key"❱: PublicKeyBase
                ❰"recoveryMethod"❱: "recovery"
            ]
        ]
        "#}.trim()
        );
        let decoded = UpdateRecoveryRequest::try_from(request_envelope).unwrap();
        assert_eq!(request, decoded);

        let request = UpdateRecoveryRequest::new_opt(id(), key, None);
        let request_envelope = request.clone().envelope();
        assert_eq!(request_envelope.format(),
        indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"updateRecovery"» [
                ❰"key"❱: PublicKeyBase
                ❰"recoveryMethod"❱: null
            ]
        ]
        "#}.trim()
        );
        let decoded = UpdateRecoveryRequest::try_from(request_envelope).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let response = UpdateRecoveryResponse::new(id());
        let response_envelope = response.clone().envelope();
        assert_eq!(response_envelope.format(),
        indoc! {r#"
        response(ARID(8712dfac)) [
            'result': 'OK'
        ]
        "#}.trim()
        );
        let decoded = UpdateRecoveryResponse::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);
    }
}
