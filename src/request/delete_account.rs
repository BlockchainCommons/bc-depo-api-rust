use bc_components::{PublicKeyBase, ARID};
use bc_envelope::prelude::*;
use anyhow::{Error, Result};

use crate::{DELETE_ACCOUNT_FUNCTION, util::{Abbrev, FlankedFunction}};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteAccountRequest {
    id: ARID,
    key: PublicKeyBase,
}

impl DeleteAccountRequest {
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

impl From<DeleteAccountRequest> for Envelope {
    fn from(value: DeleteAccountRequest) -> Self {
        Envelope::new_function(DELETE_ACCOUNT_FUNCTION)
            .into_transaction_request(value.id, value.key)
    }
}

impl TryFrom<Envelope> for DeleteAccountRequest {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        let (id, key, body, _) = envelope.parse_transaction_request(Some(&DELETE_ACCOUNT_FUNCTION))?;
        Self::from_body(id, key, body)
    }
}

impl std::fmt::Display for DeleteAccountRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {} key {}",
            self.id().abbrev(),
            "deleteAccount".flanked_function(),
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

        let request = DeleteAccountRequest::from_fields(id(), key);
        let request_envelope = request.clone().to_envelope();
        assert_eq!(
            request_envelope.format(),
            indoc! {r#"
        request(ARID(8712dfac)) [
            'body': «"deleteAccount"»
            'senderPublicKey': PublicKeyBase
        ]
        "#}
            .trim()
        );
        let decoded = request_envelope.try_into().unwrap();
        assert_eq!(request, decoded);
    }
}
