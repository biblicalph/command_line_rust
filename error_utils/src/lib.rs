use std::{
    error,
    io::{self, ErrorKind},
};
use thiserror;
use walkdir::Error as WalkDirError;

#[derive(Debug)]
pub struct ProgramErrorParams<'a> {
    pathname: Option<&'a str>,
    prg_name: Option<&'a str>,
}

impl<'a> ProgramErrorParams<'a> {
    pub fn new() -> Self {
        Self {
            pathname: None,
            prg_name: None,
        }
    }
    pub fn program(mut self, name: &'a str) -> Self {
        self.prg_name = Some(name);
        self
    }
    pub fn pathname(mut self, name: &'a str) -> Self {
        self.pathname = Some(name);
        self
    }
    pub fn build(self) -> anyhow::Result<Self> {
        if let (Some(_), Some(_)) = (self.prg_name, self.pathname) {
            Ok(self)
        } else {
            Err(anyhow::anyhow!("Both prg_name and pathname are required!"))
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ProgramError<'a> {
    #[error("{}: {}: File or directory not found", prg_name, pathname)]
    FileNotFound {
        prg_name: &'a str,
        pathname: &'a str,
    },
    #[error("{prg_name}: {pathname}: Permission denied")]
    PermissionDenied {
        prg_name: &'a str,
        pathname: &'a str,
    },
    #[error("{prg_name}: {pathname}: {err}")]
    Other {
        prg_name: &'a str,
        pathname: &'a str,
        err: Box<dyn error::Error>,
    },
}

impl<'a> From<(io::Error, ProgramErrorParams<'a>)> for ProgramError<'a> {
    fn from((err, params): (io::Error, ProgramErrorParams<'a>)) -> Self {
        let (prg_name, pathname) = (params.prg_name.unwrap(), params.pathname.unwrap());

        match err.kind() {
            ErrorKind::NotFound => Self::FileNotFound { prg_name, pathname },
            ErrorKind::PermissionDenied => Self::PermissionDenied { prg_name, pathname },
            _ => Self::Other {
                prg_name,
                pathname,
                err: Box::new(err),
            },
        }
    }
}

impl<'a> From<(Box<dyn error::Error>, ProgramErrorParams<'a>)> for ProgramError<'a> {
    fn from((err, params): (Box<dyn error::Error>, ProgramErrorParams<'a>)) -> Self {
        if let Some(e) = err.downcast_ref::<WalkDirError>() {
            if let Some(io_err) = e.io_error() {
                return (io::Error::new(io_err.kind(), io_err.to_string()), params).into();
            }
        }
        if let Some(e) = err.downcast_ref::<io::Error>() {
            return (io::Error::new(e.kind(), e.to_string()), params).into();
        }
        Self::Other {
            prg_name: params.prg_name.unwrap(),
            pathname: params.pathname.unwrap(),
            err,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_not_found_error() -> anyhow::Result<()> {
        let err = io::Error::new(ErrorKind::NotFound, "file not found");
        let params = ProgramErrorParams::new()
            .pathname("blargh")
            .program("findr")
            .build()?;

        let actual: ProgramError = (err, params).into();
        assert_eq!(
            actual.to_string(),
            "findr: blargh: File or directory not found"
        );
        Ok(())
    }

    #[test]
    fn translate_permission_error() -> anyhow::Result<()> {
        let err = io::Error::new(ErrorKind::PermissionDenied, "you can't access this");
        let params = ProgramErrorParams::new()
            .pathname("blargh")
            .program("findr")
            .build()?;

        let actual: ProgramError = (err, params).into();
        assert_eq!(actual.to_string(), "findr: blargh: Permission denied");
        Ok(())
    }

    #[test]
    fn translate_non_io_error() -> anyhow::Result<()> {
        let err = "a12".parse::<i32>().err().unwrap();
        let params = ProgramErrorParams::new()
            .pathname("blargh")
            .program("findr")
            .build()?;

        let actual: ProgramError = (Box::new(err) as Box<dyn error::Error>, params).into();
        assert_eq!(
            actual.to_string(),
            "findr: blargh: invalid digit found in string"
        );
        Ok(())
    }
}
