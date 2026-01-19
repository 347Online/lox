#[derive(Debug)]
pub enum StackError {
    StackSizeExceeded(usize),
    PopWhileEmpty,
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackError::StackSizeExceeded(n) => write!(f, "exceeded maximum stack size of {n}"),
            StackError::PopWhileEmpty => write!(f, "todo"),
        }
    }
}

pub type StackResult<T> = Result<T, StackError>;

pub struct Stack<T, const N: usize> {
    inner: Vec<T>,
}

impl<T, const N: usize> Stack<T, N> {
    pub const fn new() -> Stack<T, N> {
        Stack { inner: vec![] }
    }

    pub fn try_push(&mut self, value: T) -> StackResult<()> {
        if self.inner.len() >= N {
            return Err(StackError::StackSizeExceeded(N));
        }

        self.inner.push(value);

        Ok(())
    }

    pub fn push(&mut self, value: T) {
        match self.try_push(value) {
            Ok(()) => (),
            Err(err) => panic!("{err}"),
        }
    }

    pub fn try_pop(&mut self) -> StackResult<T> {
        self.inner.pop().ok_or(StackError::PopWhileEmpty)
    }

    pub fn pop(&mut self) -> T {
        match self.try_pop() {
            Ok(value) => value,
            Err(err) => panic!("{err}"),
        }
    }

    pub const fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }
}

impl<T, const N: usize> Default for Stack<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
