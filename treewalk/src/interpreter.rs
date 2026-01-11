use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::environment::Environment;
use crate::error::Exception;
use crate::expr::{Expr, ExprData};
use crate::function::{LoxFunction, native_fn};
use crate::lox::{Lox, LoxState};
use crate::object::Object;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};

fn stdlib(env: &mut Environment) {
    env.define(
        "clock",
        &native_fn!(|_, _| {
            Object::from(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            )
        }),
    );

    env.define(
        "dbg",
        &native_fn!(1, |_, args| {
            let x = &args[0];

            println!("{x:#?}");

            Object::Nil
        }),
    );
}

#[derive(Debug)]
pub struct Interpreter {
    pub(crate) state: Rc<RefCell<LoxState>>,
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<Expr, usize>,
}

impl Interpreter {
    pub fn new(state: Rc<RefCell<LoxState>>) -> Self {
        let mut lib = Environment::new_raw();

        stdlib(&mut lib);

        let globals = lib.finish();
        let environment = globals.clone();
        #[allow(clippy::mutable_key_type)]
        let locals = HashMap::new();

        Interpreter {
            state,
            globals,
            environment,
            locals,
        }
    }

    fn look_up_var(&self, name: &Token, expr: &Expr) -> Result<Object, Exception> {
        if let Some(distance) = self.locals.get(expr) {
            Ok(Environment::get_at(
                self.environment.clone(),
                *distance,
                &name.lexeme,
            ))
        } else {
            self.globals.borrow().get(name)
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, Exception> {
        let value = match &expr.data {
            ExprData::Literal { value } => value.clone(),
            ExprData::Grouping { expr } => self.evaluate(expr.deref())?,
            ExprData::Unary { op, rhs } => match op.kind {
                TokenType::Bang => (!self.evaluate(rhs.deref())?.is_truthy()).into(),

                TokenType::Minus => {
                    if let ExprData::Literal {
                        value: Object::Number(value),
                    } = rhs.data
                    {
                        Object::Number(-value)
                    } else {
                        return Err(Exception::num(op.clone()));
                    }
                }

                _ => unreachable!("no other unary expression"),
            },
            ExprData::Binary { op, lhs, rhs } => {
                let (lhs, rhs) = (self.evaluate(lhs.as_ref())?, self.evaluate(rhs.as_ref())?);

                macro_rules! binary {
                    ($op:tt, $kind:tt) => {
                        if let (Object::Number(lhs), Object::Number(rhs)) = (lhs, rhs) {
                            Ok(Object::$kind(lhs $op rhs))
                        } else {
                            Err(Exception::num_pair(op.clone()))
                        }
                    };
                }

                match op.kind {
                    TokenType::Minus => binary!(-, Number)?,
                    TokenType::Slash => binary!(/, Number)?,
                    TokenType::Star => binary!(*, Number)?,

                    TokenType::Plus => match (lhs, rhs) {
                        (Object::Number(lhs), Object::Number(rhs)) => (lhs + rhs).into(),
                        (Object::String(lhs), Object::String(rhs)) => (lhs + &rhs).as_str().into(),

                        _ => {
                            return Err(Exception::nums_or_strings(op.clone()));
                        }
                    },

                    TokenType::Greater => binary!(>, Boolean)?,
                    TokenType::GreaterEqual => binary!( >=, Boolean)?,
                    TokenType::Less => binary!(<, Boolean)?,
                    TokenType::LessEqual => binary!(<=, Boolean)?,

                    TokenType::BangEqual => (lhs != rhs).into(),
                    TokenType::EqualEqual => (lhs == rhs).into(),

                    _ => unreachable!(),
                }
            }
            // ExprData::Variable { name } => self.environment.borrow().get(name)?.clone(),
            ExprData::Variable { name } => self.look_up_var(name, expr)?,
            ExprData::Assign { name, value } => {
                let value = self.evaluate(value)?;
                // self.environment.borrow_mut().assign(name, &value)?;
                if let Some(distance) = self.locals.get(expr) {
                    // self.environment.assign
                    Environment::assign_at(self.environment.clone(), *distance, name, &value);
                } else {
                    self.globals.borrow_mut().assign(name, &value)?;
                }

                value
            }
            ExprData::Logical { op, lhs, rhs } => {
                let lhs = self.evaluate(lhs)?;
                if op.kind == TokenType::Or {
                    if lhs.is_truthy() {
                        return Ok(lhs);
                    }
                } else if !lhs.is_truthy() {
                    return Ok(lhs);
                }

                self.evaluate(rhs)?
            }
            ExprData::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evaluate(callee)?;

                let mut args = vec![];
                for argument in arguments {
                    args.push(self.evaluate(argument)?);
                }

                let Object::Fn(function) = callee else {
                    let paren = paren.clone();
                    return Err(Exception::new(paren, "Can only call functions and classes"));
                };

                let paren = paren.clone();
                if arguments.len() != function.arity() {
                    return Err(Exception::new(
                        paren,
                        format!(
                            "Expected {} arguments but got {}.",
                            function.arity(),
                            arguments.len()
                        ),
                    ));
                }
                function.call(self, &args)?
            }
        };

        Ok(value)
    }

    pub(crate) fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), Exception> {
        let previous = self.environment.clone();

        let result = 'block: {
            self.environment = environment;

            for stmt in statements {
                match self.execute(stmt) {
                    Ok(_) => (),
                    x => break 'block x,
                }
            }

            Ok(())
        };

        self.environment = previous;

        result
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), Exception> {
        match stmt {
            Stmt::Expr { expr } => {
                self.evaluate(expr)?;
            }
            Stmt::Print { expr } => {
                let value = self.evaluate(expr)?;
                println!("{value}");
            }
            Stmt::Var { name, initializer } => {
                let value = if let Some(initializer) = initializer {
                    self.evaluate(initializer)?
                } else {
                    Object::Nil
                };

                self.environment.borrow_mut().define(&name.lexeme, &value);
            }
            Stmt::Block { statements } => {
                self.execute_block(
                    statements,
                    Environment::new_enclosed(self.environment.clone()),
                )?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?;
                }
            }
            Stmt::While { condition, body } => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?;
                }
            }
            Stmt::Function {
                name,
                parameters,
                body,
            } => {
                let function = LoxFunction::new(
                    name.clone(),
                    parameters.clone(),
                    body.clone(),
                    self.environment.clone(),
                );

                self.environment
                    .borrow_mut()
                    .define(&name.lexeme, &Object::from(function));
            }
            Stmt::Return { expr, .. } => {
                let value = if let Some(expr) = expr {
                    self.evaluate(expr)?
                } else {
                    Object::Nil
                };

                return Err(Exception::Return(value));
            }
        }

        Ok(())
    }

    pub fn interpret(&mut self, statements: &[Stmt]) {
        let result = 'block: {
            for stmt in statements {
                match self.execute(stmt) {
                    Ok(_) => (),
                    x => break 'block x,
                }
            }

            Ok(())
        };

        match result {
            Ok(_) => (),
            Err(Exception::Error { token, message }) => {
                Lox::runtime_error(self.state.borrow_mut(), Exception::Error { token, message })
            }
            Err(Exception::Return(x)) => unreachable!("Escaped return signal: {x}"),
        }
    }

    pub(crate) fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.clone(), depth);
    }
}
