use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

use crate::error::Exception;
use crate::object::Object;
use crate::token::Token;

pub trait EnvLookup {
    fn enclosing(&self) -> Option<Box<Self>>;

    fn ancestor(&self, distance: usize) -> Option<Box<Self>>
    where
        Self: Clone,
    {
        let mut environment = Box::new(self.clone());

        for _ in 0..distance {
            environment = environment.enclosing()?;
        }

        Some(environment)
    }
}

#[derive(Debug)]
pub struct Environment {
    id: Uuid,
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub(crate) fn new_raw() -> Self {
        let id = Uuid::new_v4();
        let values = HashMap::new();

        Environment {
            id,
            enclosing: None,
            values,
        }
    }

    pub(crate) fn finish(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    pub fn new() -> Rc<RefCell<Self>> {
        Environment::new_raw().finish()
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        let id = Uuid::new_v4();
        let enclosing = Some(enclosing);
        let values = HashMap::new();

        Rc::new(RefCell::new(Environment {
            id,
            enclosing,
            values,
        }))
    }

    pub fn define(&mut self, name: &str, value: &Object) {
        self.values.insert(name.to_owned(), value.clone());
    }

    pub fn ancestor(
        this: Rc<RefCell<Environment>>,
        distance: usize,
    ) -> Option<Rc<RefCell<Environment>>> {
        let mut environment = Some(this.clone());

        for _ in 0..distance {
            environment = environment.unwrap().borrow().enclosing.clone();
        }

        environment
    }

    pub fn get_at(this: Rc<RefCell<Environment>>, distance: usize, name: &str) -> Object {
        Self::ancestor(this, distance)
            .unwrap()
            .borrow()
            .values
            .get(name)
            .unwrap()
            .clone()
    }

    pub fn assign_at(
        this: Rc<RefCell<Environment>>,
        distance: usize,
        name: &Token,
        value: &Object,
    ) {
        Self::ancestor(this, distance)
            .unwrap()
            .borrow_mut()
            .values
            .insert(name.lexeme.to_owned(), value.clone());
    }

    pub fn get(&self, name: &Token) -> Result<Object, Exception> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }

        Err(Exception::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn assign(&mut self, name: &Token, value: &Object) -> Result<(), Exception> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_owned(), value.clone());

            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(name, value)?;

            return Ok(());
        }

        Err(Exception::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }
}

impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Environment {}
