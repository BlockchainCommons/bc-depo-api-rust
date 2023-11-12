use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;

use crate::{NEW_KEY_PARAM, UPDATE_KEY_FUNCTION};

use super::{parse_request, parse_response, request_body, request_envelope, response_envelope};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateKeyRequest {
    id: ARID,
    key: PublicKeyBase,
    new_key: PublicKeyBase,
}

impl UpdateKeyRequest {
    pub fn new(key: impl AsRef<PublicKeyBase>, new_key: impl AsRef<PublicKeyBase>) -> Self {
        Self::new_opt(ARID::new(), key.as_ref().clone(), new_key.as_ref().clone())
    }

    pub fn new_opt(id: ARID, key: PublicKeyBase, new_key: PublicKeyBase) -> Self {
        Self { id, key, new_key }
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }

    pub fn key(&self) -> &PublicKeyBase {
        &self.key
    }

    pub fn new_key(&self) -> &PublicKeyBase {
        &self.new_key
    }
}

impl EnvelopeEncodable for UpdateKeyRequest {
    fn envelope(self) -> Envelope {
        let body =
            request_body(UPDATE_KEY_FUNCTION, self.key).add_parameter(NEW_KEY_PARAM, self.new_key);
        request_envelope(self.id, body)
    }
}

impl From<UpdateKeyRequest> for Envelope {
    fn from(value: UpdateKeyRequest) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for UpdateKeyRequest {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, key, body) = parse_request(UPDATE_KEY_FUNCTION, envelope)?;
        let new_key: PublicKeyBase = body.extract_object_for_parameter(NEW_KEY_PARAM)?;
        Ok(Self::new_opt(id, key, new_key))
    }
}

impl TryFrom<Envelope> for UpdateKeyRequest {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for UpdateKeyRequest {}

//
// Response
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateKeyResponse {
    id: ARID,
}

impl UpdateKeyResponse {
    pub fn new(id: ARID) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }
}

impl EnvelopeEncodable for UpdateKeyResponse {
    fn envelope(self) -> Envelope {
        response_envelope(self.id, None)
    }
}

impl From<UpdateKeyResponse> for Envelope {
    fn from(value: UpdateKeyResponse) -> Self {
        value.envelope()
    }
}

impl EnvelopeDecodable for UpdateKeyResponse {
    fn from_envelope(envelope: Envelope) -> anyhow::Result<Self> {
        let (id, _result) = parse_response(envelope)?;
        Ok(Self::new(id))
    }
}

impl TryFrom<Envelope> for UpdateKeyResponse {
    type Error = anyhow::Error;

    fn try_from(value: Envelope) -> anyhow::Result<Self> {
        Self::from_envelope(value)
    }
}

impl EnvelopeCodable for UpdateKeyResponse {}

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

        let new_key = PrivateKeyBase::new().public_keys();

        let request = UpdateKeyRequest::new_opt(id(), key, new_key);
        let request_envelope = request.clone().envelope();
        assert_eq!(
            request_envelope.format(),
            indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"updateKey"» [
                ❰"key"❱: PublicKeyBase
                ❰"newKey"❱: PublicKeyBase
            ]
        ]
        "#}
            .trim()
        );
        let decoded = UpdateKeyRequest::try_from(request_envelope).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let response = UpdateKeyResponse::new(id());
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
        let decoded = UpdateKeyResponse::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);
    }
}
