use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Invalid envelope structure
    #[error("invalid envelope structure: {message}")]
    InvalidEnvelope { message: String },

    /// Missing required parameter in envelope
    #[error("missing required parameter: {parameter}")]
    MissingParameter { parameter: String },

    /// Invalid parameter value in envelope
    #[error("invalid parameter value for '{parameter}': {message}")]
    InvalidParameter { parameter: String, message: String },

    /// Envelope type mismatch
    #[error("envelope type mismatch: expected '{expected}', found '{found}'")]
    TypeMismatch { expected: String, found: String },

    /// Invalid receipt format
    #[error("invalid receipt format: {message}")]
    InvalidReceipt { message: String },

    /// Invalid digest data
    #[error("invalid digest data: {message}")]
    InvalidDigest { message: String },

    /// Envelope processing error
    #[error("envelope processing failed")]
    EnvelopeProcessing(#[from] bc_envelope::Error),

    /// DCBOR processing error
    #[error("DCBOR processing failed")]
    DcborProcessing(#[from] dcbor::Error),

    /// GSTP processing error
    #[error("GSTP processing failed")]
    GstpProcessing(#[from] gstp::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
