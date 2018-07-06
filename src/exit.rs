use colored::*;
use std::fmt::Display;
use std::process;
use VERBOSE;

pub trait Exit<T> {
    fn unwrap_or_exit(self, message: &str) -> T;
}

impl<T> Exit<T> for Option<T> {
    fn unwrap_or_exit(self, message: &str) -> T {
        match self {
            Some(value) => value,
            None => {
                error!("{}", message);
                process::exit(1)
            }
        }
    }
}

impl<T, E: Display> Exit<T> for Result<T, E> {
    fn unwrap_or_exit(self, message: &str) -> T {
        match self {
            Ok(value) => value,
            Err(verbose_error) => {
                error!("{}", message);
                log!("{}", verbose_error);
                process::exit(1)
            }
        }
    }
}
