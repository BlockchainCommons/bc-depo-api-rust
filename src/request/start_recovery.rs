use bc_envelope::prelude::*;
use anyhow::{ Error, Result };

use crate::{ RECOVERY_METHOD_PARAM, START_RECOVERY_FUNCTION, util::{ Abbrev, FlankedFunction } };

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartRecovery(String);

impl StartRecovery {
    pub fn new(recovery: String) -> Self {
        Self(recovery)
    }

    pub fn recovery(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<StartRecovery> for Expression {
    fn from(value: StartRecovery) -> Self {
        Expression::new(START_RECOVERY_FUNCTION).with_parameter(
            RECOVERY_METHOD_PARAM,
            value.0.clone()
        )
    }
}

impl TryFrom<Expression> for StartRecovery {
    type Error = Error;

    fn try_from(expression: Expression) -> Result<Self> {
        Ok(Self::new(expression.extract_object_for_parameter(RECOVERY_METHOD_PARAM)?))
    }
}

impl std::fmt::Display for StartRecovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!("{} {}", "startRecovery".flanked_function(), self.recovery().abbrev())
        )
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
        let request = StartRecovery::new(recovery);
        let expression: Expression = request.clone().into();
        let request_envelope = expression.to_envelope();
        // println!("{}", request_envelope.format());
        #[rustfmt::skip]
        assert_eq!(request_envelope.format(), indoc! {r#"
            «"startRecovery"» [
                ❰"recoveryMethod"❱: "recovery"
            ]
        "#}.trim());
        let decoded_expression = Expression::try_from(request_envelope).unwrap();
        let decoded = StartRecovery::try_from(decoded_expression).unwrap();
        assert_eq!(request, decoded);
    }
}
