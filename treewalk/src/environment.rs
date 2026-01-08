use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::RuntimeError;
use crate::object::Object;
use crate::token::Token;

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl<'src> Environment {
    pub fn new() -> Self {
        let values = HashMap::new();

        Environment {
            enclosing: None,
            values,
        }
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self {
        let enclosing = Some(enclosing);
        let values = HashMap::new();

        Environment { enclosing, values }
    }

    pub fn define(&mut self, name: &'src str, value: Object) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn get(&self, name: &Token<'src>) -> Result<Object, RuntimeError<'src>> {
        if let Some(value) = self.values.get(name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }

        Err(RuntimeError::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn assign(&mut self, name: &Token<'src>, value: &Object) -> Result<(), RuntimeError<'src>> {
        if self.values.contains_key(name.lexeme) {
            self.values.insert(name.lexeme.to_owned(), value.clone());

            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(name, value)?;

            return Ok(());
        }

        Err(RuntimeError::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
