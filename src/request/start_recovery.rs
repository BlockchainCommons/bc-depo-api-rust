use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;
use anyhow::{Error, Result};

use crate::{RECOVERY_METHOD_PARAM, START_RECOVERY_FUNCTION, util::{Abbrev, FlankedFunction}};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartRecoveryRequest {
    id: ARID,
    key: PublicKeyBase,
    recovery: String,
}

impl StartRecoveryRequest {
    pub fn from_fields(id: ARID, key: PublicKeyBase, recovery: String) -> Self {
        Self {
            id,
            key,
            recovery,
        }
    }

    pub fn new(
        key: impl AsRef<PublicKeyBase>,
        recovery: impl AsRef<str>,
    ) -> Self {
        Self::from_fields(
            ARID::new(),
            key.as_ref().clone(),
            recovery.as_ref().to_string(),
        )
    }

    pub fn from_body(id: ARID, key: PublicKeyBase, body: Envelope) -> Result<Self> {
        let recovery: String = body.extract_object_for_parameter(RECOVERY_METHOD_PARAM)?;
        Ok(Self::from_fields(id, key, recovery))
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
}

impl From<StartRecoveryRequest> for Envelope {
    fn from(value: StartRecoveryRequest) -> Self {
        let id = value.id().clone();
        Envelope::new_function(START_RECOVERY_FUNCTION)
            .add_parameter(RECOVERY_METHOD_PARAM, value.recovery)
            .into_transaction_request(id, &value.key)
    }
}

impl TryFrom<&Envelope> for StartRecoveryRequest {
    type Error = Error;

    fn try_from(envelope: &Envelope) -> Result<Self> {
        let (id, key, body, _) = envelope.parse_transaction_request(Some(&START_RECOVERY_FUNCTION))?;
        Self::from_body(id, key, body)
    }
}

impl std::fmt::Display for StartRecoveryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} {} for key {}",
            self.id().abbrev(),
            "startRecovery".flanked_function(),
            self.recovery().abbrev(),
            self.key().abbrev()
        ))
    }
}

//
// Response
//

#[derive(Debug, Clone)]
pub struct StartRecoveryResponse {
    id: ARID,
    continuation: Envelope,
}

impl StartRecoveryResponse {
    pub fn new(id: ARID, continuation: Envelope) -> Self {
        Self { id, continuation }
    }

    pub fn id(&self) -> &ARID {
        self.id.as_ref()
    }

    pub fn continuation(&self) -> Envelope {
        self.continuation.clone()
    }
}

impl From<StartRecoveryResponse> for Envelope {
    fn from(value: StartRecoveryResponse) -> Self {
        value.continuation.into_success_response(value.id)
    }
}

impl TryFrom<Envelope> for StartRecoveryResponse {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        let (continuation, id) = envelope.parse_success_response(None)?;
        Ok(Self::new(id, continuation))
    }
}

impl PartialEq for StartRecoveryResponse {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self
                .continuation
                .is_identical_to(&other.continuation)
    }
}

impl Eq for StartRecoveryResponse {}

impl std::fmt::Display for StartRecoveryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} OK: continuation {}",
            self.id().abbrev(),
            "startRecovery".flanked_function(),
            self.continuation().abbrev()
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
        let recovery = "recovery".to_string();
        let new_key = PrivateKeyBase::new().public_key();

        let request = StartRecoveryRequest::from_fields(id(), new_key, recovery);
        let request_envelope = request.to_envelope();
        assert_eq!(
            request_envelope.format(),
            indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"startRecovery"» [
                ❰"recoveryMethod"❱: "recovery"
            ]
            'senderPublicKey': PublicKeyBase
        ]
        "#}
            .trim()
        );
        let decoded = StartRecoveryRequest::try_from(&request_envelope).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        let continuation = "continuation";
        let response = StartRecoveryResponse::new(id(), continuation.to_envelope());
        let response_envelope = response.to_envelope();
        assert_eq!(
            response_envelope.format(),
            indoc! {r#"
        response(ARID(8712dfac)) [
            'result': "continuation"
        ]
        "#}
            .trim()
        );
        let decoded = StartRecoveryResponse::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);
    }
}
