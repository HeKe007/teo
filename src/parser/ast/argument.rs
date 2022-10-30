use std::fmt::{Display, Formatter};
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub struct Argument {
    pub(crate) name: Option<Identifier>,
    pub(crate) value: Expression,
    pub(crate) span: Span,
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            f.write_str(&name.name)?;
            f.write_str(":")?;
        }
        Display::fmt(&self.value, f)
    }
}