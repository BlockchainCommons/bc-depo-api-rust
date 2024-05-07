use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;
use anyhow::{Error, Result};

use crate::{GET_RECOVERY_FUNCTION, util::{Abbrev, FlankedFunction}};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetRecoveryRequest {
    id: ARID,
    key: PublicKeyBase,
}

impl GetRecoveryRequest {
    pub fn from_fields(id: ARID, key: PublicKeyBase) -> Self {
        Self { id, key }
    }

    pub fn new(key: impl AsRef<PublicKeyBase>) -> Self {
        Self::from_fields(ARID::new(), key.as_ref().clone())
    }

    pub fn from_body(id: ARID, key: PublicKeyBase, _body: Envelope) -> Result<Self> {
        Ok(Self::from_fields(id, key))
    }

    pub fn id(&self) -> &ARID {
        &self.id
    }

    pub fn key(&self) -> &PublicKeyBase {
        &self.key
    }
}

impl From<GetRecoveryRequest> for Envelope {
    fn from(value: GetRecoveryRequest) -> Self {
        Envelope::new_function(GET_RECOVERY_FUNCTION)
            .into_transaction_request(value.id, value.key)
    }
}

impl TryFrom<Envelope> for GetRecoveryRequest {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        let (id, key, body, _) = envelope.parse_transaction_request(Some(&GET_RECOVERY_FUNCTION))?;
        Self::from_body(id, key, body)
    }
}

impl std::fmt::Display for GetRecoveryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} key {}",
            self.id().abbrev(),
            "getRecovery".flanked_function(),
            self.key().abbrev()
        ))
    }
}

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

impl From<GetRecoveryResponse> for Envelope {
    fn from(value: GetRecoveryResponse) -> Self {
        let result: Envelope = if let Some(recovery) = value.recovery {
            recovery.to_envelope()
        } else {
            Envelope::null()
        };
        result.into_success_response(value.id)
    }
}

impl TryFrom<Envelope> for GetRecoveryResponse {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        let (result, id) = envelope.parse_success_response(None)?;
        let recovery = if result.is_null() {
            None
        } else {
            Some(result.extract_subject()?)
        };
        Ok(Self::new(id, recovery))
    }
}

impl std::fmt::Display for GetRecoveryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} OK: {}",
            self.id().abbrev(),
            "getRecovery".flanked_function(),
            self.recovery().abbrev()
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

        let request = GetRecoveryRequest::from_fields(id(), key);
        let request_envelope = request.to_envelope();
        assert_eq!(
            request_envelope.format(),
            indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"getRecovery"»
            'senderPublicKey': PublicKeyBase
        ]
        "#}
            .trim()
        );
        let decoded = request_envelope.try_into().unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let response = GetRecoveryResponse::new(id(), Some("Recovery Method".into()));
        let response_envelope = response.to_envelope();
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
        let response_envelope = response.to_envelope();
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
