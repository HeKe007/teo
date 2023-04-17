use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use serde::{Serialize};
use maplit::hashmap;
use key_path::KeyPath;
use std::borrow::Cow;
use crate::core::model::Model;

// New errors

#[derive(Debug)]
pub struct FatalError(Cow<'static, str>);

#[derive(Debug)]
pub struct ServerError(Cow<'static, str>);

#[derive(Debug)]
pub enum RuntimeError {
    ObjectIsNotSaved,
}

impl RuntimeError {
    pub fn message(&self) -> &'static str {
        match self {
            RuntimeError::ObjectIsNotSaved => "Object is not saved thus can't be deleted.",
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message())
    }
}

pub enum UserErrorType {
    ValidationError,
    UnexpectedInput,
    DestinationNotFound,
    IncorrectJSONFormat,
    MissingRequiredInput,
    ObjectNotFound,
    InvalidAuthToken,
    PermissionError,
    DeletionDenied,
    CustomInternalServerError,
    CustomValidationError,
    UniqueConstraintError,
    WrongIdentityModel,
    CustomErrorType(Cow<'static, str>),
}

#[derive(Debug)]
pub struct UserError {
    r#type: UserErrorType,
    message: Cow<'static, str>,
    errors: Option<HashMap<Cow<'static, str>, Cow<'static, str>>>,
}

#[derive(Debug)]
pub enum Error {
    FatalError(FatalError),
    RuntimeError(RuntimeError),
    UserError(UserError),
    ServerError(ServerError),
}

impl Error {
    pub fn fatal(message: &'static str) -> Self {
        Self::FatalError(FatalError(Cow::Borrowed(message)))
    }

    pub fn fatal_message(message: String) -> Self {
        Self::FatalError(FatalError(Cow::Owned(message)))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            FatalError(err) => f.write_str(err.0.as_ref()),
            RuntimeError(err) => Display::fmt(err, f),
            UserError(err) => f.write_str(err.message.as_ref()),
            ServerError(err) => f.write_str(err.0.as_ref()),
        }
    }
}

impl std::error::Error for Error { }




// Old errors


// impl ErrorType {
//     pub(crate) fn code(&self) -> u16 {
//         match self {
//             ErrorType::ValidationError => { 400 }
//             ErrorType::IncorrectJSONFormat => { 400 }
//             ErrorType::UnknownDatabaseWriteError => { 500 }
//             ErrorType::UnknownDatabaseDeleteError => { 500 }
//             ErrorType::UnknownDatabaseFindError => { 500 }
//             ErrorType::UnknownDatabaseFindUniqueError => { 500 }
//             ErrorType::DestinationNotFound => { 404 }
//             ErrorType::InternalServerError => { 500 }
//             ErrorType::ObjectNotFound => { 404 }
//             ErrorType::InvalidAuthToken => { 401 }
//             ErrorType::CustomInternalServerError => { 500 }
//             ErrorType::CustomValidationError => { 400 }
//             ErrorType::WrongIdentityModel => { 401 }
//             ErrorType::PropertySetterError => { 400 }
//             ErrorType::UnexpectedInputRootType => { 400 }
//             ErrorType::UnexpectedInputType => { 400 }
//             ErrorType::UnexpectedInputKey => { 400 }
//             ErrorType::MissingRequiredInput => { 400 }
//             ErrorType::UnexpectedObjectLength => { 400 }
//             ErrorType::InvalidKey => { 500 }
//             ErrorType::InvalidOperation => { 500 }
//             ErrorType::PermissionError => { 401 }
//             ErrorType::DeletionDenied => { 400 }
//             ErrorType::RecordDecodingError => { 500 }
//         }
//     }
// }

impl Error {

    pub fn message(&self) -> &str {
        match self {
            Error::FatalError(fatal_error) => fatal_error.0.as_ref(),
            Error::ServerError(server_error) => server_error.0.as_ref(),
            Error::RuntimeError(runtime_error) => runtime_error.message(),
            Error::UserError(user_error) => user_error.message.as_ref(),
        }
    }

    pub(crate) fn unique_value_duplicated(field: &'static str) -> Self {
        Error::UserError(UserError {
            r#type: UserErrorType::UniqueConstraintError,
            message: Cow::Borrowed("Value is not unique."),
            errors: Some(hashmap!{
                Cow::Borrowed(field) => Cow::Borrowed("value is not unique")
            }),
        })
    }

    pub(crate) fn internal_server_error(reason: impl Into<String>) -> Self {
        Error::ServerError(ServerError(Cow::Owned(reason.into())))
    }

    pub(crate) fn unknown_database_write_error() -> Self {
        Error::ServerError(ServerError(Cow::Borrowed("An unknown database write error occurred.")))
    }

    pub(crate) fn unknown_database_delete_error() -> Self {
        Error::ServerError(ServerError(Cow::Borrowed("An unknown database delete error occurred.")))
    }

    pub(crate) fn destination_not_found() -> Self {
        Error::UserError(UserError {
            r#type: UserErrorType::DestinationNotFound,
            message: Cow::Borrowed("The request destination is not found."),
            errors: None,
        })
    }

    pub(crate) fn object_not_found() -> Self {
        Error::UserError(UserError {
            r#type: UserErrorType::ObjectNotFound,
            message: Cow::Borrowed("The request object is not found."),
            errors: None,
        })
    }

    pub(crate) fn object_is_not_saved_thus_cant_be_deleted() -> Self {
        Error::RuntimeError(RuntimeError::ObjectIsNotSaved)
    }

    pub(crate) fn unknown_database_find_error() -> Self {
        Error::ServerError(ServerError(Cow::Borrowed("An unknown database find error occurred.")))
    }

    pub(crate) fn unknown_database_find_unique_error() -> Self {
        Error::ServerError(ServerError(Cow::Borrowed("An unknown database find unique error occurred.")))
    }

    pub(crate) fn record_decoding_error<'a>(model: &str, path: impl AsRef<KeyPath<'a>>, expected: impl AsRef<str>) -> Self {
        Error::ServerError(ServerError(Cow::Owned(format!("Record decoding error. Expect `{}' for value at path `{}' of model `{model}'.", expected.as_ref(), path.as_ref()))))
    }

    pub(crate) fn invalid_auth_token() -> Self {
        Error::UserError(UserError {
            r#type: UserErrorType::InvalidAuthToken,
            message: Cow::Borrowed("This auth token is invalid."),
            errors: None,
        })
    }

    pub fn custom_internal_server_error(message: impl Into<String>) -> Self {
        Error::ServerError(ServerError(Cow::Owned(message.into())))
    }

    pub fn custom_validation_error(message: impl Into<String>) -> Self {
        Error::UserError(UserError {
            r#type: UserErrorType::CustomValidationError,
            message: Cow::Owned(message.into()),
            errors: None,
        })
    }

    pub(crate) fn wrong_identity_model() -> Self {
        Error::UserError(UserError {
            r#type: UserErrorType::WrongIdentityModel,
            message: Cow::Borrowed("This identity is valid but is not of this model."),
            errors: None,
        })
    }

    // new error types which should be used across the project

    pub(crate) fn incorrect_json_format() -> Self {
        Error::UserError(UserError {
            r#type: UserErrorType::IncorrectJSONFormat,
            message: Cow::Borrowed("The request JSON body format is incorrect."),
            errors: None,
        })
    }

    pub(crate) fn unexpected_input_root_type<'a>(expected: impl AsRef<str>) -> Self {
        Error::UserError(UserError {
            r#type: UserErrorType::UnexpectedInput,
            message: Cow::Owned(format!("Unexpected root input type. Expect {}.", expected.as_ref())),
            errors: None,
        })
    }

    pub(crate) fn unexpected_input_type<'a>(expected: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error::UserError(UserError {
            r#type: UserErrorType::UnexpectedInput,
            message: Cow::Owned(format!("Unexpected input type. Expect {}.", expected.as_ref())),
            errors: Some(hashmap!{Cow::Owned(key_path.as_ref().to_string()) => Cow::Owned(format!("Expect {}.", expected.into()))}),
        })

    }

    pub(crate) fn unexpected_input_key<'a>(unexpected: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error::UserError(UserError {
            r#type: UserErrorType::UnexpectedInput,
            message: Cow::Owned(format!("Unexpected key '{}'.", unexpected.as_ref())),
            errors: Some(hashmap!{Cow::Owned(key_path.as_ref().to_string()) => Cow::Owned(format!("Expect {}.", unexpected.into()))}),
        })
    }

    pub(crate) fn unexpected_input_value<'a>(expected: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error::UserError(UserError {
            r#type: UserErrorType::UnexpectedInput,
            message: Cow::Owned(format!("Unexpected value found.")),
            errors: Some(hashmap!{Cow::Owned(key_path.as_ref().to_string()) => Cow::Owned(format!("Expect {}.", expected.into()))}),
        })
    }

    pub(crate) fn cannot_disconnect_previous_relation() -> Self {
        Error::UserError(UserError {
            r#type: UserErrorType::UnexpectedInput,
            message: Cow::Owned(format!("Required relation cannot disconnect previous connected value.")),
            errors: None,
        })
    }

    pub(crate) fn unexpected_input_value_with_reason<'a>(reason: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error {
            r#type: ErrorType::ValidationError,
            message: "Unexpected value found.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("{}", reason.into())}),
        }
    }

    pub(crate) fn missing_required_input<'a>(key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error {
            r#type: ErrorType::MissingRequiredInput,
            message: "Missing required input.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("value is required")})
        }
    }

    pub(crate) fn missing_required_input_with_type<'a>(expected: impl AsRef<str>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error {
            r#type: ErrorType::MissingRequiredInput,
            message: "Missing required input.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("{} value is required", expected.as_ref())})
        }
    }

    pub(crate) fn unexpected_object_length<'a>(expected: usize, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error {
            r#type: ErrorType::UnexpectedObjectLength,
            message: "Unexpected object length.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("Expect length {}.", expected)})
        }
    }

    pub(crate) fn invalid_key(unexpected_key: impl AsRef<str>, model: &Model) -> Self {
        Error {
            r#type: ErrorType::InvalidKey,
            message: format!("Invalid key '{}' accessed on model `{}'", unexpected_key.as_ref(), model.name()),
            errors: None
        }
    }

    pub(crate) fn invalid_operation(reason: impl AsRef<str>) -> Self {
        Error {
            r#type: ErrorType::InvalidOperation,
            message: reason.as_ref().to_string(),
            errors: None
        }
    }

    pub(crate) fn deletion_denied(relation_name: impl AsRef<str>) -> Self {
        Error {
            r#type: ErrorType::DeletionDenied,
            message: format!("Deletion denied by `{}'.", relation_name.as_ref()),
            errors: None
        }
    }

    pub(crate) fn validation_error<'a>(path: impl AsRef<KeyPath<'a>>, reason: impl Into<String>) -> Self {
        Error {
            r#type: ErrorType::ValidationError,
            message: "Validation failed.".to_string(),
            errors: Some(hashmap!{path.as_ref().to_string() => reason.into()})
        }
    }

    pub(crate) fn internal_server_error_with_path<'a>(path: impl AsRef<KeyPath<'a>>, reason: impl Into<String>) -> Self {
        Error {
            r#type: ErrorType::InternalServerError,
            message: "Internal server error.".to_string(),
            errors: Some(hashmap!{path.as_ref().to_string() => reason.into()})
        }
    }

    pub(crate) fn permission_error<'a>(path: impl AsRef<KeyPath<'a>>, reason: impl Into<String>) -> Self {
        Error {
            r#type: ErrorType::PermissionError,
            message: "Permission denied.".to_string(),
            errors: Some(hashmap!{path.as_ref().to_string() => reason.into()})
        }
    }

    pub(crate) fn is_custom_internal_server_error(&self) -> bool {
        self.r#type == ErrorType::CustomInternalServerError
    }

    pub(crate) fn is_custom_validation_error(&self) -> bool {
        self.r#type == ErrorType::CustomValidationError
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::custom_internal_server_error(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::custom_internal_server_error(value)
    }
}

unsafe impl Sync for Error {}
unsafe impl Send for Error {}
