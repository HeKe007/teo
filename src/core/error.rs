use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use serde::{Serialize};
use maplit::hashmap;
use key_path::KeyPath;
use crate::core::model::Model;

#[derive(Debug, PartialEq, Serialize)]
pub enum ErrorType {
    UnknownDatabaseWriteError,
    UnknownDatabaseDeleteError,
    UnknownDatabaseFindError,
    UnknownDatabaseFindUniqueError,
    UnknownDatabaseCountError,
    InternalServerError,
    ObjectIsNotSaved,
    FieldIsNotUnique,
    WrongIdentityModel,
    PropertySetterError,

    // request destination
    DestinationNotFound,

    // request input
    IncorrectJSONFormat,
    UnexpectedInputRootType,
    UnexpectedInputType,
    UnexpectedInputKey,
    UnexpectedInputValue,
    MissingRequiredInput,
    UnexpectedObjectLength,

    // request token
    InvalidAuthToken,

    // request permission
    PermissionDenied,
    DeletionDenied,

    // response destination
    ObjectNotFound,

    // response output
    UnexpectedOutputException,

    // object api
    InvalidKey,
    InvalidOperation,

    // user defined
    CustomError,

    // database
    RecordDecodingError,
}

impl ErrorType {
    pub fn code(&self) -> u16 {
        match self {
            ErrorType::IncorrectJSONFormat => { 400 }
            ErrorType::ObjectIsNotSaved => { 400 }
            ErrorType::UnknownDatabaseWriteError => { 500 }
            ErrorType::UnknownDatabaseDeleteError => { 500 }
            ErrorType::UnknownDatabaseFindError => { 500 }
            ErrorType::UnknownDatabaseFindUniqueError => { 500 }
            ErrorType::UnknownDatabaseCountError => { 500 }
            ErrorType::DestinationNotFound => { 404 }
            ErrorType::InternalServerError => { 500 }
            ErrorType::ObjectNotFound => { 404 }
            ErrorType::FieldIsNotUnique => { 400 }
            ErrorType::InvalidAuthToken => { 401 }
            ErrorType::CustomError => { 500 }
            ErrorType::WrongIdentityModel => { 401 }
            ErrorType::PropertySetterError => { 400 }
            ErrorType::UnexpectedInputRootType => { 400 }
            ErrorType::UnexpectedInputType => { 400 }
            ErrorType::UnexpectedInputKey => { 400 }
            ErrorType::UnexpectedInputValue => { 400 }
            ErrorType::MissingRequiredInput => { 400 }
            ErrorType::UnexpectedObjectLength => { 400 }
            ErrorType::InvalidKey => { 500 }
            ErrorType::InvalidOperation => { 500 }
            ErrorType::PermissionDenied => { 401 }
            ErrorType::UnexpectedOutputException => { 500 }
            ErrorType::DeletionDenied => { 400 }
            ErrorType::RecordDecodingError => { 500 }
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Error {
    pub r#type: ErrorType,
    pub message: String,
    pub errors: Option<HashMap<String, String>>
}

impl Error {

    pub fn unexpected_enum_value(field: impl Into<String>) -> Self {
        let mut errors: HashMap<String, String> = HashMap::with_capacity(1);
        errors.insert(field.into(), "Enum value is unexpected.".to_string());
        Error {
            r#type: ErrorType::UnexpectedInputValue,
            message: "Enum value is unexpected.".to_string(),
            errors: Some(errors)
        }
    }

    pub fn unique_value_duplicated_reason(field: impl AsRef<str>, reason: impl AsRef<str>) -> Self {
        let mut errors: HashMap<String, String> = HashMap::with_capacity(1);
        errors.insert(field.as_ref().into(), format!("{}", reason.as_ref()));
        Error {
            r#type: ErrorType::UnexpectedInputValue,
            message: "Unique value duplicated.".to_string(),
            errors: Some(errors)
        }
    }

    pub fn unique_value_duplicated(field: impl AsRef<str>) -> Self {
        let mut errors: HashMap<String, String> = HashMap::with_capacity(1);
        errors.insert(field.as_ref().into(), "Value is not unique.".into());
        Error {
            r#type: ErrorType::UnexpectedInputValue,
            message: "Unique value duplicated.".to_string(),
            errors: Some(errors)
        }
    }

    pub fn internal_server_error(reason: String) -> Self {
        Error {
            r#type: ErrorType::InternalServerError,
            message: reason,
            errors: None
        }
    }

    pub fn unknown_database_write_error() -> Self {
        Error {
            r#type: ErrorType::UnknownDatabaseWriteError,
            message: "An unknown database write error occurred.".to_string(),
            errors: None
        }
    }

    pub fn unknown_database_delete_error() -> Self {
        Error {
            r#type: ErrorType::UnknownDatabaseDeleteError,
            message: "An unknown database delete error occurred.".to_string(),
            errors: None
        }
    }

    pub fn destination_not_found() -> Self {
        Error {
            r#type: ErrorType::DestinationNotFound,
            message: "The request destination is not found.".to_string(),
            errors: None
        }
    }

    pub fn object_not_found() -> Self {
        Error {
            r#type: ErrorType::ObjectNotFound,
            message: "The requested object is not exist.".to_string(),
            errors: None
        }
    }

    pub fn object_is_not_saved() -> Self {
        Error {
            r#type: ErrorType::ObjectIsNotSaved,
            message: "This object is not saved thus can't be deleted.".to_string(),
            errors: None
        }
    }

    pub fn field_is_not_unique() -> Self {
        Error {
            r#type: ErrorType::FieldIsNotUnique,
            message: format!("Unique where input is not unique."),
            errors: None
        }
    }

    pub fn unknown_database_find_error() -> Self {
        Error {
            r#type: ErrorType::UnknownDatabaseFindError,
            message: "An unknown query error occurred.".to_string(),
            errors: None
        }
    }

    pub fn unknown_database_find_unique_error() -> Self {
        Error {
            r#type: ErrorType::UnknownDatabaseFindUniqueError,
            message: "An unknown query unique error occurred.".to_string(),
            errors: None
        }
    }

    pub fn unknown_database_count_error() -> Self {
        Error {
            r#type: ErrorType::UnknownDatabaseCountError,
            message: "An unknown count error occurred.".to_string(),
            errors: None
        }
    }

    pub fn record_decoding_error<'a>(model: &str, path: impl AsRef<KeyPath<'a>>, expected: impl AsRef<str>) -> Self {
        Error {
            r#type: ErrorType::RecordDecodingError,
            message: format!("Expect `{}' for value at path `{}' of model `{model}'.", expected.as_ref(), path.as_ref()),
            errors: None
        }
    }

    pub fn invalid_auth_token() -> Self {
        Error {
            r#type: ErrorType::InvalidAuthToken,
            message: "This auth token is invalid.".to_string(),
            errors: None
        }
    }

    pub fn custom_error(message: impl Into<String>) -> Self {
        Error {
            r#type: ErrorType::CustomError,
            message: message.into(),
            errors: None
        }
    }

    pub fn wrong_identity_model() -> Self {
        Error {
            r#type: ErrorType::WrongIdentityModel,
            message: format!("This identity is valid but is not of this model."),
            errors: None
        }
    }

    pub fn property_setter_error(reason: impl Into<String>) -> Self {
        Error {
            r#type: ErrorType::PropertySetterError,
            message: reason.into(),
            errors: None
        }
    }

    // new error types which should be used across the project

    pub fn incorrect_json_format() -> Self {
        Error {
            r#type: ErrorType::IncorrectJSONFormat,
            message: "Incorrect JSON format.".to_string(),
            errors: None
        }
    }

    pub fn unexpected_input_root_type<'a>(expected: impl AsRef<str>) -> Self {
        Error {
            r#type: ErrorType::UnexpectedInputRootType,
            message: format!("Unexpected root input type. Expect {}.", expected.as_ref()),
            errors: None
        }
    }

    pub fn unexpected_input_type<'a>(expected: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error {
            r#type: ErrorType::UnexpectedInputType,
            message: "Unexpected input type found.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("Expect {}.", expected.into())}),
        }
    }

    pub fn unexpected_input_key<'a>(unexpected: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error {
            r#type: ErrorType::UnexpectedInputKey,
            message: "Unexpected key found.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("Unexpected key '{}'.", unexpected.into())}),
        }
    }

    pub fn unexpected_input_value<'a>(expected: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error {
            r#type: ErrorType::UnexpectedInputValue,
            message: "Unexpected value found.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("Expect `{}'.", expected.into())}),
        }
    }

    pub fn unexpected_input_value_with_reason<'a>(reason: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error {
            r#type: ErrorType::UnexpectedInputValue,
            message: "Unexpected value found.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("{}", reason.into())}),
        }
    }

    pub fn missing_required_input<'a>(expected: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error {
            r#type: ErrorType::MissingRequiredInput,
            message: "Missing required input.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("Expect `{}'.", expected.into())})
        }
    }

    pub fn unexpected_object_length<'a>(expected: usize, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        Error {
            r#type: ErrorType::UnexpectedObjectLength,
            message: "Unexpected object length.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("Expect length {}.", expected)})
        }
    }

    pub fn invalid_key(unexpected_key: impl AsRef<str>, model: &Model) -> Self {
        Error {
            r#type: ErrorType::InvalidKey,
            message: format!("Invalid key '{}' accessed on model `{}'", unexpected_key.as_ref(), model.name()),
            errors: None
        }
    }

    pub fn invalid_operation(reason: impl AsRef<str>) -> Self {
        Error {
            r#type: ErrorType::InvalidOperation,
            message: reason.as_ref().to_string(),
            errors: None
        }
    }

    pub fn permission_denied(action: impl AsRef<str>) -> Self {
        Error {
            r#type: ErrorType::PermissionDenied,
            message: format!("Permission denied for `{}'.", action.as_ref()),
            errors: None
        }
    }

    pub fn unexpected_output_exception<'a>(path: impl AsRef<KeyPath<'a>>, reason: impl AsRef<str>) -> Self {
        Error {
            r#type: ErrorType::UnexpectedOutputException,
            message: format!("Unexpected output exception."),
            errors: Some(hashmap!{path.as_ref().to_string() => reason.as_ref().to_string()})
        }
    }

    pub fn deletion_denied(relation_name: impl AsRef<str>) -> Self {
        Error {
            r#type: ErrorType::DeletionDenied,
            message: format!("Deletion denied by `{}'.", relation_name.as_ref()),
            errors: None
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.r#type.fmt(f)
    }
}

impl std::error::Error for Error { }

unsafe impl Sync for Error {}
unsafe impl Send for Error {}
