use std::collections::HashMap;

use crate::error::RuntimeError;
use crate::object::Object;
use crate::token::Token;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl<'src> Environment {
    pub fn new() -> Self {
        let values = HashMap::new();

        Environment { values }
    }

    pub fn define(&mut self, name: &'src str, value: Object) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn get(&self, name: &Token<'src>) -> Result<&Object, RuntimeError<'src>> {
        if let Some(value) = self.values.get(name.lexeme) {
            Ok(value)
        } else {
            Err(RuntimeError::new(
                name.clone(),
                format!("Undefined variable '{}'.", name.lexeme),
            ))
        }
    }

    pub fn assign(&mut self, name: &Token<'src>, value: &Object) -> Result<(), RuntimeError<'src>> {
        if self.values.contains_key(name.lexeme) {
            self.values.insert(name.lexeme.to_owned(), value.clone());

            Ok(())
        } else {
            Err(RuntimeError::new(
                name.clone(),
                format!("Undefined variable '{}'.", name.lexeme),
            ))
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
