pub mod receipt;
pub use receipt::Receipt;

pub mod request;
pub use request::*;

use bc_envelope::prelude::*;

// Functions

pub const DELETE_ACCOUNT_FUNCTION_NAME: &str = "deleteAccount";
pub const DELETE_ACCOUNT_FUNCTION: Function = Function::new_static_named(DELETE_ACCOUNT_FUNCTION_NAME);

pub const DELETE_SHARES_FUNCTION_NAME: &str = "deleteShares";
pub const DELETE_SHARES_FUNCTION: Function = Function::new_static_named(DELETE_SHARES_FUNCTION_NAME);

pub const FINISH_RECOVERY_FUNCTION_NAME: &str = "finishRecovery";
pub const FINISH_RECOVERY_FUNCTION: Function = Function::new_static_named(FINISH_RECOVERY_FUNCTION_NAME);

pub const GET_RECOVERY_FUNCTION_NAME: &str = "getRecovery";
pub const GET_RECOVERY_FUNCTION: Function = Function::new_static_named(GET_RECOVERY_FUNCTION_NAME);

pub const GET_SHARES_FUNCTION_NAME: &str = "getShares";
pub const GET_SHARES_FUNCTION: Function = Function::new_static_named(GET_SHARES_FUNCTION_NAME);

pub const START_RECOVERY_FUNCTION_NAME: &str = "startRecovery";
pub const START_RECOVERY_FUNCTION: Function = Function::new_static_named(START_RECOVERY_FUNCTION_NAME);

pub const STORE_SHARE_FUNCTION_NAME: &str = "storeShare";
pub const STORE_SHARE_FUNCTION: Function = Function::new_static_named(STORE_SHARE_FUNCTION_NAME);

pub const UPDATE_XID_DOCUMENT_FUNCTION_NAME: &str = "updateXIDDocument";
pub const UPDATE_XID_DOCUMENT_FUNCTION: Function = Function::new_static_named(UPDATE_XID_DOCUMENT_FUNCTION_NAME);

pub const UPDATE_RECOVERY_FUNCTION_NAME: &str = "updateRecovery";
pub const UPDATE_RECOVERY_FUNCTION: Function = Function::new_static_named(UPDATE_RECOVERY_FUNCTION_NAME);

// Parameters

pub const DATA_PARAM_NAME: &str = "data";
pub const DATA_PARAM: Parameter = Parameter::new_static_named(DATA_PARAM_NAME);

pub const NEW_XID_DOCUMENT_PARAM_NAME: &str = "newXIDDocument";
pub const NEW_XID_DOCUMENT_PARAM: Parameter = Parameter::new_static_named(NEW_XID_DOCUMENT_PARAM_NAME);

pub const RECEIPT_PARAM_NAME: &str = "receipt";
pub const RECEIPT_PARAM: Parameter = Parameter::new_static_named(RECEIPT_PARAM_NAME);

pub const RECOVERY_CONTINUATION_PARAM_NAME: &str = "recoveryContinuation";

pub const RECOVERY_METHOD_PARAM_NAME: &str = "recoveryMethod";
pub const RECOVERY_METHOD_PARAM: Parameter = Parameter::new_static_named(RECOVERY_METHOD_PARAM_NAME);
