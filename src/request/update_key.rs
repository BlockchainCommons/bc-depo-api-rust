use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;
use anyhow::{Error, Result};

use crate::{NEW_KEY_PARAM, UPDATE_KEY_FUNCTION, util::{Abbrev, FlankedFunction}};

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
    pub fn from_fields(id: ARID, key: PublicKeyBase, new_key: PublicKeyBase) -> Self {
        Self { id, key, new_key }
    }

    pub fn new(key: impl AsRef<PublicKeyBase>, new_key: impl AsRef<PublicKeyBase>) -> Self {
        Self::from_fields(ARID::new(), key.as_ref().clone(), new_key.as_ref().clone())
    }

    pub fn from_body(id: ARID, key: PublicKeyBase, body: Envelope) -> Result<Self> {
        let new_key = body.extract_object_for_parameter(NEW_KEY_PARAM)?;
        Ok(Self::from_fields(id, key, new_key))
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

impl From<UpdateKeyRequest> for Envelope {
    fn from(value: UpdateKeyRequest) -> Self {
        let id = value.id().clone();
        Envelope::new_function(UPDATE_KEY_FUNCTION)
            .add_parameter(NEW_KEY_PARAM, value.new_key)
            .into_transaction_request(id, value.key)
    }
}

impl TryFrom<&Envelope> for UpdateKeyRequest {
    type Error = Error;

    fn try_from(envelope: &Envelope) -> Result<Self> {
        let (id, key, body, _) = envelope.parse_transaction_request(Some(&UPDATE_KEY_FUNCTION))?;
        Self::from_body(id, key, body)
    }
}

impl std::fmt::Display for UpdateKeyRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} old {} new {}",
            self.id().abbrev(),
            "updateKey".flanked_function(),
            self.key().abbrev(),
            self.new_key().abbrev()
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

        let new_key = PrivateKeyBase::new().public_key();

        let request = UpdateKeyRequest::from_fields(id(), key, new_key);
        let request_envelope = request.to_envelope();
        assert_eq!(
            request_envelope.format(),
            indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"updateKey"» [
                ❰"newKey"❱: PublicKeyBase
            ]
            'senderPublicKey': PublicKeyBase
        ]
        "#}
            .trim()
        );
        let decoded = UpdateKeyRequest::try_from(&request_envelope).unwrap();
        assert_eq!(request, decoded);
    }
}
