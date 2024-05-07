use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;
use anyhow::{Error, Result};

use crate::{UPDATE_RECOVERY_FUNCTION, RECOVERY_METHOD_PARAM, util::{Abbrev, FlankedFunction}};

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
    pub fn from_fields(id: ARID, key: PublicKeyBase, recovery: Option<String>) -> Self {
        Self {
            id,
            key,
            recovery,
        }
    }

    pub fn new(
        key: impl AsRef<PublicKeyBase>,
        recovery: Option<&str>,
    ) -> Self {
        Self::from_fields(
            ARID::new(),
            key.as_ref().clone(),
            recovery.map(|s| s.to_string()),
        )
    }

    pub fn from_body(id: ARID, key: PublicKeyBase, body: Envelope) -> Result<Self> {
        let recovery_envelope = body.object_for_parameter(RECOVERY_METHOD_PARAM)?;
        let recovery: Option<String> = if recovery_envelope.is_null() {
            None
        } else {
            Some(recovery_envelope.extract_subject()?)
        };
        Ok(Self::from_fields(id, key, recovery))
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

impl From<UpdateRecoveryRequest> for Envelope {
    fn from(value: UpdateRecoveryRequest) -> Self {
        let method = if let Some(recovery) = value.recovery.clone() {
            recovery.to_envelope()
        } else {
            Envelope::null()
        };
        Envelope::new_function(UPDATE_RECOVERY_FUNCTION)
            .add_parameter(RECOVERY_METHOD_PARAM, method)
            .into_transaction_request(value.id(), &value.key)
    }
}

impl TryFrom<&Envelope> for UpdateRecoveryRequest {
    type Error = Error;

    fn try_from(envelope: &Envelope) -> Result<Self> {
        let (id, key, body, _) = envelope.parse_transaction_request(Some(&UPDATE_RECOVERY_FUNCTION))?;
        Self::from_body(id, key, body)
    }
}

impl std::fmt::Display for UpdateRecoveryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} key {} to {}",
            self.id().abbrev(),
            "updateRecovery".flanked_function(),
            self.key().abbrev(),
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
        ARID::from_data_ref(hex_literal::hex!("8712dfac3d0ebfa910736b2a9ee39d4b68f64222a77bcc0074f3f5f1c9216d30")).unwrap()
    }

    #[test]
    fn test_request() {
        let private_key = PrivateKeyBase::new();
        let key = private_key.public_key();

        let recovery = "recovery".to_string();

        let request = UpdateRecoveryRequest::from_fields(id(), key.clone(), Some(recovery));
        let request_envelope = request.to_envelope();
        assert_eq!(request_envelope.format(),
        indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"updateRecovery"» [
                ❰"recoveryMethod"❱: "recovery"
            ]
            'senderPublicKey': PublicKeyBase
        ]
        "#}.trim()
        );
        let decoded = UpdateRecoveryRequest::try_from(&request_envelope).unwrap();
        assert_eq!(request, decoded);

        let request = UpdateRecoveryRequest::from_fields(id(), key, None);
        let request_envelope = request.to_envelope();
        assert_eq!(request_envelope.format(),
        indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"updateRecovery"» [
                ❰"recoveryMethod"❱: null
            ]
            'senderPublicKey': PublicKeyBase
        ]
        "#}.trim()
        );
        let decoded = UpdateRecoveryRequest::try_from(&request_envelope).unwrap();
        assert_eq!(request, decoded);
    }
}
