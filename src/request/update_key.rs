use bc_components::PublicKeyBase;
use bc_envelope::prelude::*;
use anyhow::{Error, Result};

use crate::{NEW_KEY_PARAM, UPDATE_KEY_FUNCTION, util::{Abbrev, FlankedFunction}};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateKey(PublicKeyBase);

impl UpdateKey {
    pub fn new(new_key: PublicKeyBase) -> Self {
        Self(new_key)
    }

    pub fn new_key(&self) -> &PublicKeyBase {
        &self.0
    }
}

impl From<UpdateKey> for Expression {
    fn from(value: UpdateKey) -> Self {
        Expression::new(UPDATE_KEY_FUNCTION)
            .with_parameter(NEW_KEY_PARAM, value.0)
    }
}

impl TryFrom<Expression> for UpdateKey {
    type Error = Error;

    fn try_from(expression: Expression) -> Result<Self> {
        let new_key: PublicKeyBase = expression.extract_object_for_parameter(NEW_KEY_PARAM)?;
        Ok(Self::new(new_key))
    }
}

impl std::fmt::Display for UpdateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} new {}",
            "updateKey".flanked_function(),
            self.new_key().abbrev()
        ))
    }
}

#[cfg(test)]
mod tests {
    use bc_components::PrivateKeyBase;
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_request() {
        let new_key = PrivateKeyBase::new().public_key();

        let request = UpdateKey::new(new_key);
        let expression: Expression = request.clone().into();
        let request_envelope = expression.to_envelope();
        // println!("{}", request_envelope.format());
        assert_eq!(request_envelope.format(), indoc! {r#"
        «"updateKey"» [
            ❰"newKey"❱: PublicKeyBase
        ]
        "#}.trim());
        let decoded_expression = Expression::try_from(request_envelope).unwrap();
        let decoded = UpdateKey::try_from(decoded_expression).unwrap();
        assert_eq!(request, decoded);
    }
}
