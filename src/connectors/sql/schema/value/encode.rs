use chrono::{Date, Utc, DateTime, SecondsFormat};
use key_path::KeyPath;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::error::ActionError;
use crate::core::field::r#type::FieldType;
use crate::prelude::{Graph, Value};

pub trait ToSQLString {
    fn to_string(&self, dialect: SQLDialect) -> String;
}

pub trait TypeOrNull {
    fn or_null(&self, optional: bool) -> String;
}

impl TypeOrNull for &str {
    fn or_null(&self, optional: bool) -> String {
        self.to_string() + if optional { " or null" } else { "" }
    }
}

pub(crate) trait ValueToSQLString {
    fn to_sql_string<'a>(&self, r#type: &FieldType, optional: bool, graph: &Graph) -> String;
}

impl ValueToSQLString for Value {
    fn to_sql_string<'a>(&self, r#type: &FieldType, optional: bool, graph: &Graph) -> String {
        if optional {
            if self.is_null() {
                return "NULL".to_owned()
            }
        }
        match r#type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => panic!("SQL doesn't support `ObjectId`."),
            FieldType::String => self.as_str().unwrap().to_sql_input(),
            FieldType::Bool => self.as_bool().unwrap().to_sql_input(),
            FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 |
            FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 => if let Some(val) = self.as_i64() {
                val.to_string()
            } else if let Some(val) = self.as_u64() {
                val.to_string()
            } else {
                panic!("Uncoded number.")
            }
            FieldType::F32 | FieldType::F64 => if let Some(val) = self.as_f64() {
                val.to_string()
            } else if let Some(val) = self.as_i64() {
                val.to_string()
            } else if let Some(val) = self.as_u64() {
                val.to_string()
            } else {
                panic!("Uncoded number.")
            }
            FieldType::Enum(_) => self.as_str().unwrap().to_sql_input(),
            FieldType::Vec(element_field) => {
                let val = self.as_vec().unwrap();
                let mut result: Vec<String> = vec![];
                for (i, v) in val.iter().enumerate() {
                    result.push(v.to_sql_string(element_field.r#type(), element_field.is_optional(), graph));
                }
                result.join(", ").wrap_in_array()
            }
            _ => { panic!() }
        }
    }
}

impl ValueToSQLString for &Value {
    fn to_sql_string<'a>(&self, r#type: &FieldType, optional: bool, graph: &Graph) -> String {
        (*self).to_sql_string(r#type, optional, graph)
    }
}

impl ToSQLString for Value {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        match self {
            Value::Null => "NULL".to_owned(),
            Value::String(string) => string.to_sql_input(),
            Value::I8(i) => i.to_string(),
            Value::I16(i) => i.to_string(),
            Value::I32(i) => i.to_string(),
            Value::I64(i) => i.to_string(),
            Value::I128(i) => i.to_string(),
            Value::U8(i) => i.to_string(),
            Value::U16(i) => i.to_string(),
            Value::U32(i) => i.to_string(),
            Value::U64(i) => i.to_string(),
            Value::U128(i) => i.to_string(),
            Value::F32(i) => i.to_string(),
            Value::F64(i) => i.to_string(),
            Value::Bool(b) => b.to_sql_input(),
            Value::Date(d) => d.to_sql_input(),
            Value::DateTime(d) => d.to_sql_input(),
            _ => panic!("unhandled"),
        }
    }
}

pub trait ToWrapped {
    fn to_wrapped(&self) -> String;
}

impl ToWrapped for String {
    fn to_wrapped(&self) -> String {
        "(".to_owned() + self + ")"
    }
}

pub trait ToSQLInput {
    fn to_sql_input(&self) -> String;
}

impl ToSQLInput for String {
    fn to_sql_input(&self) -> String {
        let mut result = String::with_capacity(self.len() + 2);
        result.push('\'');
        for ch in self.chars() {
            match ch {
                '\'' => result.push_str("\\'"),
                _ => result.push(ch)
            }
        }
        result.push('\'');
        result
    }
}

impl ToSQLInput for &str {
    fn to_sql_input(&self) -> String {
        let mut result = String::with_capacity(self.len() + 2);
        result.push('\'');
        for ch in self.chars() {
            match ch {
                '\'' => result.push_str("\\'"),
                _ => result.push(ch)
            }
        }
        result.push('\'');
        result
    }
}

impl ToSQLInput for bool {
    fn to_sql_input(&self) -> String {
        if *self { "TRUE".to_owned() } else { "FALSE".to_owned() }
    }
}

impl ToSQLInput for Date<Utc> {
    fn to_sql_input(&self) -> String {
        self.format("%Y-%m-%d").to_string().to_sql_input()
    }
}

impl ToSQLInput for DateTime<Utc> {
    fn to_sql_input(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S.%f").to_string().to_sql_input()
    }
}

pub trait IfIMode {
    fn to_i_mode(&self, i_mode: bool) -> String;
}

impl IfIMode for &str {
    fn to_i_mode(&self, i_mode: bool) -> String {
        if i_mode {
            "LOWER(".to_owned() + self + ")"
        } else {
            self.to_string()
        }
    }
}

impl IfIMode for String {
    fn to_i_mode(&self, i_mode: bool) -> String {
        self.as_str().to_i_mode(i_mode)
    }
}

pub trait ToLike {
    fn to_like(&self, left: bool, right: bool) -> String;
}

impl ToLike for &str {
    fn to_like(&self, left: bool, right: bool) -> String {
        let mut retval = "".to_owned();
        retval.push(self.chars().nth(0).unwrap());
        if left {
            retval.push('%');
        }
        retval += &self[1..self.len() - 1];
        if right {
            retval.push('%');
        }
        retval.push(self.chars().nth(self.len() - 1).unwrap());
        retval
    }
}

impl ToLike for String {
    fn to_like(&self, left: bool, right: bool) -> String {
        self.as_str().to_like(left, right)
    }
}

pub trait WrapInArray {
    fn wrap_in_array(&self) -> String;
}

impl WrapInArray for &str {
    fn wrap_in_array(&self) -> String {
        "ARRAY[".to_owned() + self + "]"
    }
}

impl WrapInArray for String {
    fn wrap_in_array(&self) -> String {
        self.as_str().wrap_in_array()
    }
}