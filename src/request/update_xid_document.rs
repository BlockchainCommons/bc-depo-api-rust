use anyhow::{Error, Result};
use bc_components::XIDProvider;
use bc_envelope::prelude::*;
use bc_xid::XIDDocument;

use crate::{
    NEW_XID_DOCUMENT_PARAM, UPDATE_XID_DOCUMENT_FUNCTION, util::FlankedFunction,
};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateXIDDocument(XIDDocument);

impl UpdateXIDDocument {
    pub fn new(new_xid_document: XIDDocument) -> Self { Self(new_xid_document) }

    pub fn new_xid_document(&self) -> &XIDDocument { &self.0 }
}

impl From<UpdateXIDDocument> for Expression {
    fn from(value: UpdateXIDDocument) -> Self {
        Expression::new(UPDATE_XID_DOCUMENT_FUNCTION)
            .with_parameter(NEW_XID_DOCUMENT_PARAM, value.0)
    }
}

impl TryFrom<Expression> for UpdateXIDDocument {
    type Error = Error;

    fn try_from(expression: Expression) -> Result<Self> {
        let new_xid_document = XIDDocument::try_from(
            expression.object_for_parameter(NEW_XID_DOCUMENT_PARAM)?,
        )?;
        Ok(Self::new(new_xid_document))
    }
}

impl std::fmt::Display for UpdateXIDDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} new {}",
            "updateXIDDocument".flanked_function(),
            self.new_xid_document().xid()
        ))
    }
}

#[cfg(test)]
mod tests {
    use bc_components::{PrivateKeyBase, PublicKeysProvider};
    use bc_rand::make_fake_random_number_generator;
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_request() {
        bc_envelope::register_tags();

        let mut rng = make_fake_random_number_generator();
        let new_xid_document: XIDDocument =
            PrivateKeyBase::new_using(&mut rng).public_keys().into();

        let request = UpdateXIDDocument::new(new_xid_document);
        let expression: Expression = request.clone().into();
        let request_envelope = expression.to_envelope();
        // println!("{}", request_envelope.format());
        #[rustfmt::skip]
        assert_eq!(request_envelope.format(), indoc! {r#"
            «"updateXIDDocument"» [
                ❰"newXIDDocument"❱: XID(71274df1) [
                    'key': PublicKeys(eb9b1cae) [
                        'allow': 'All'
                    ]
                ]
            ]
        "#}.trim());
        let decoded_expression =
            Expression::try_from(request_envelope).unwrap();
        let decoded = UpdateXIDDocument::try_from(decoded_expression).unwrap();
        assert_eq!(request, decoded);
    }
}
