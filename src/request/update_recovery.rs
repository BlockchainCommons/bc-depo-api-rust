use anyhow::{Error, Result};
use bc_envelope::prelude::*;

use crate::{
    RECOVERY_METHOD_PARAM, UPDATE_RECOVERY_FUNCTION,
    util::{Abbrev, FlankedFunction},
};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateRecovery(Option<String>);

impl UpdateRecovery {
    pub fn new(recovery: Option<String>) -> Self { Self(recovery) }

    pub fn recovery(&self) -> Option<&String> { self.0.as_ref() }
}

impl From<UpdateRecovery> for Expression {
    fn from(value: UpdateRecovery) -> Self {
        let method = if let Some(recovery) = value.0.clone() {
            recovery.to_envelope()
        } else {
            Envelope::null()
        };
        Expression::new(UPDATE_RECOVERY_FUNCTION)
            .with_parameter(RECOVERY_METHOD_PARAM, method)
    }
}

impl TryFrom<Expression> for UpdateRecovery {
    type Error = Error;

    fn try_from(expression: Expression) -> Result<Self> {
        let recovery_object =
            expression.object_for_parameter(RECOVERY_METHOD_PARAM)?;
        let recovery = if recovery_object.is_null() {
            None
        } else {
            Some(recovery_object.extract_subject()?)
        };
        Ok(Self::new(recovery))
    }
}

impl std::fmt::Display for UpdateRecovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} to {}",
            "updateRecovery".flanked_function(),
            self.recovery().abbrev()
        ))
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_request() {
        bc_envelope::register_tags();

        let recovery = "recovery".to_string();

        let request = UpdateRecovery::new(Some(recovery));
        let expression: Expression = request.clone().into();
        let request_envelope = expression.to_envelope();
        // println!("{}", request_envelope.format());
        #[rustfmt::skip]
        assert_eq!(request_envelope.format(), indoc! {r#"
            «"updateRecovery"» [
                ❰"recoveryMethod"❱: "recovery"
            ]
        "#}.trim());
        let decoded_expression =
            Expression::try_from(request_envelope).unwrap();
        let decoded = UpdateRecovery::try_from(decoded_expression).unwrap();
        assert_eq!(request, decoded);
    }
}
