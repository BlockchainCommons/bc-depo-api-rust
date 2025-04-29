use bc_envelope::prelude::*;
use anyhow::{ Error, Result };

use crate::{ FINISH_RECOVERY_FUNCTION, util::FlankedFunction };

//
// Request
//

#[derive(Debug, Clone, PartialEq)]
pub struct FinishRecovery();

impl FinishRecovery {
    pub fn new() -> Self {
        Self()
    }
}

impl Default for FinishRecovery {
    fn default() -> Self {
        Self::new()
    }
}

impl From<FinishRecovery> for Expression {
    fn from(_: FinishRecovery) -> Self {
        Expression::new(FINISH_RECOVERY_FUNCTION)
    }
}

impl TryFrom<Expression> for FinishRecovery {
    type Error = Error;

    fn try_from(_: Expression) -> Result<Self> {
        Ok(Self::new())
    }
}

impl std::fmt::Display for FinishRecovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", "finishRecovery".flanked_function()))
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_request() {
        bc_envelope::register_tags();

        let request = FinishRecovery::new();
        let expression: Expression = request.clone().into();
        let request_envelope = expression.to_envelope();
        // println!("{}", request_envelope.format());
        #[rustfmt::skip]
        assert_eq!(request_envelope.format(), indoc! {r#"
            «"finishRecovery"»
        "#}.trim());
        let decoded_expression = Expression::try_from(request_envelope).unwrap();
        let decoded = FinishRecovery::try_from(decoded_expression).unwrap();
        assert_eq!(request, decoded);
    }
}
