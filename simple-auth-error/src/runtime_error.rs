use std::any::TypeId;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::ErrorKind;

#[derive(Debug)]
pub struct RuntimeError {
    pub kind: ErrorKind,
    pub title: &'static str,
    pub message: &'static str,
    pub file: &'static str,
    pub line: u32,
    inner: Option<InnerError>
}

#[derive(Debug)]
struct InnerError {
    type_id: TypeId,
    error: Box<dyn Error>,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.inner.is_some() {
            let err = self.get_inner().unwrap();
            write!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl Error for RuntimeError {

}

impl RuntimeError {
    pub fn new(file: &'static str, line: u32, error_kind: ErrorKind) -> Self {
        Self {
            kind: error_kind,
            title: "",
            message: "",
            file,
            line,
            inner: None,
        }
    }

    pub fn with_error<E>(mut self, error: E) -> Self
        where E: Error + Sized + 'static
    {
        self.inner = Some(InnerError {
            type_id: TypeId::of::<E>(),
            error: Box::new(error)
        });
        self
    }

    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn with_message(mut self, message: &'static str) -> Self {
        self.message = message;
        self
    }

    pub fn has_inner(&self) -> bool {
        self.inner.is_some()
    }

    pub fn get_inner(&self) -> Option<&Box<dyn Error>> {
        self.inner.as_ref().map(|x|&x.error)
    }

    pub fn get_inner_type(&self) -> Option<&TypeId> {
        self.inner.as_ref().map(|x|&x.type_id)
    }

    pub fn has_inner_of<E: Error + 'static>(&self) -> bool {
        if !self.has_inner() {
            false
        }
        else {
            let inner = self.inner.as_ref().unwrap();
            inner.type_id == TypeId::of::<E>()
        }
    }

    pub fn get_inner_as<E: Error + 'static>(&self) -> Option<&E> {
        match self.get_inner() {
            Some(error) => {
                error.downcast_ref()
            }
            _ => None
        }
    }
}

macro_rules! error {
    () => { RuntimeError::new(file!(), line!(), ErrorKind::Unknown) };
    ($kind:expr) => { RuntimeError::new(file!(), line!(), $kind) };
}

macro_rules! error_from {
    ($error:expr) => {
        RuntimeError::new(file!(), line!(), ErrorKind::Unknown).with_error($error)
    };
    ($error:expr, $kind:expr) => {
        error!($kind).with_error($error)
    };
}

#[cfg(test)]
mod test {
    use std::fs;
    use crate::ErrorKind;
    use super::{RuntimeError};

    #[test]
    fn runtime_error_compiles(){
        let error = RuntimeError {
            kind: ErrorKind::Unknown,
            title: "Some error",
            message: "A description",
            file: file!(),
            line: line!(),
            inner: None,
        };
        assert_eq!(line!() - 3, error.line);
        assert_eq!(file!(), error.file);

        let error = error!();
        assert_eq!(line!() - 1, error.line);

        let error = error!().with_title("Some error");
        assert_eq!("Some error", error.title);

        let error = error!(ErrorKind::InvalidArgument).with_message("Another description");
        assert_eq!("Another description", error.message);
    }

    #[test]
    fn runtime_error_from_compiles() {
        let error = fs::read_dir("missing directory")
            .map_err(|e|error_from!(e))
            .unwrap_err();

        assert!(100 < error.line);
        assert!(error.has_inner());
        assert!(error.has_inner_of::<std::io::Error>());

        let inner = error.get_inner_as::<std::io::Error>();
        assert!(inner.is_some());
    }
}