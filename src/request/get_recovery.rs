use bc_envelope::prelude::*;
use gstp::prelude::*;

use crate::{
    Error, GET_RECOVERY_FUNCTION, Result,
    util::{Abbrev, FlankedFunction},
};

//
// Request
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetRecovery();

impl GetRecovery {
    pub fn new() -> Self {
        Self()
    }
}

impl Default for GetRecovery {
    fn default() -> Self {
        Self::new()
    }
}

impl From<GetRecovery> for Expression {
    fn from(_: GetRecovery) -> Self {
        Expression::new(GET_RECOVERY_FUNCTION)
    }
}

impl TryFrom<Expression> for GetRecovery {
    type Error = Error;

    fn try_from(_: Expression) -> Result<Self> {
        Ok(Self::new())
    }
}

impl std::fmt::Display for GetRecovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", "getRecovery".flanked_function()))
    }
}

//
// Response
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetRecoveryResult(Option<String>);

impl GetRecoveryResult {
    pub fn new(recovery: Option<String>) -> Self {
        Self(recovery)
    }

    pub fn recovery(&self) -> Option<&str> {
        self.0.as_deref()
    }
}

impl From<GetRecoveryResult> for Envelope {
    fn from(value: GetRecoveryResult) -> Self {
        value.recovery().map_or_else(Envelope::null, Envelope::new)
    }
}

impl TryFrom<Envelope> for GetRecoveryResult {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        let recovery = if envelope.is_null() {
            None
        } else {
            Some(envelope.extract_subject().map_err(|e| {
                Error::InvalidEnvelope {
                    message: format!(
                        "failed to extract recovery subject: {}",
                        e
                    ),
                }
            })?)
        };
        Ok(Self::new(recovery))
    }
}

impl TryFrom<SealedResponse> for GetRecoveryResult {
    type Error = Error;

    fn try_from(response: SealedResponse) -> Result<Self> {
        response.result()?.clone().try_into()
    }
}

impl std::fmt::Display for GetRecoveryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} OK: {}",
            "getRecovery".flanked_function(),
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

        let request = GetRecovery::new();
        let expression: Expression = request.clone().into();
        let request_envelope = expression.to_envelope();
        #[rustfmt::skip]
        assert_eq!(request_envelope.format(), indoc! {r#"
            «"getRecovery"»
        "#}.trim());
        let decoded_expression =
            Expression::try_from(request_envelope).unwrap();
        let decoded = GetRecovery::try_from(decoded_expression).unwrap();
        assert_eq!(request, decoded);
    }

    #[test]
    fn test_response() {
        bc_envelope::register_tags();

        let response = GetRecoveryResult::new(Some("Recovery Method".into()));
        let response_envelope = response.to_envelope();
        assert_eq!(
            response_envelope.format(),
            (indoc! {
                r#"
        "Recovery Method"
        "#
            })
            .trim()
        );
        let decoded = GetRecoveryResult::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);

        let response = GetRecoveryResult::new(None);
        let response_envelope = response.to_envelope();
        assert_eq!(
            response_envelope.format(),
            (indoc! {
                r#"
        null
        "#
            })
            .trim()
        );
        let decoded = GetRecoveryResult::try_from(response_envelope).unwrap();
        assert_eq!(response, decoded);
    }
}
