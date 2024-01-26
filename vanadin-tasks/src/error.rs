use log::{debug, error, info};
use rquickjs::{Ctx, Error};

use crate::task::Task;

#[inline(never)]
pub fn evaluate_result(result: Result<(), Error>, task: &Task, ctx: &Ctx) {
    let catch = ctx.catch();

    if let Some(err) = result.err() {
        match err {
            Error::Allocation => {
                error!("Failed to allocate memory. Am I running out of memory?");
            }
            Error::DuplicateExports => {
                error!("A module tried to export an item with a duplicate name.");
            }
            Error::InvalidString(_) => {
                error!("Found an invalid string.");
            }
            Error::InvalidCStr(_) => {
                error!("Found an invalid string.");
            }
            Error::Utf8(ref e) => {
                error!("Invalid Utf8 String: {e}");
            }
            Error::Io(ref e) => {
                error!("IO Error: {e}");
            }
            Error::ClassBorrow(ref e) => {
                error!("Class borrow error: {e}");
            }
            Error::FunctionBorrow(ref e) => {
                error!("Function borrow error: {e}");
            }
            Error::Exception => {
                error!("An exception was thrown.");
                let maybe_exception = catch.as_exception();

                if let Some(exc) = maybe_exception {
                    error!(
                        "Exception at {}:{}",
                        exc.line().unwrap_or(-1),
                        exc.column().unwrap_or(-1)
                    );
                    error!(
                        "{}",
                        exc.message()
                            .unwrap_or(String::from("No error message provided"))
                    );

                    error!(
                        "Exception at {}:{}",
                        exc.line().unwrap_or(-1),
                        exc.column().unwrap_or(-1)
                    );
                    error!(
                        "{}",
                        exc.message()
                            .unwrap_or(String::from("No error message provided"))
                    );
                }
            }
            Error::FromJs {
                ref message,
                ref from,
                ref to,
            } => {
                error!(
                    "Error converting Rust from JS: {from} to {to}: {:#?}",
                    message
                )
            }
            Error::IntoJs {
                ref message,
                ref from,
                ref to,
            } => {
                error!(
                    "Error converting Rust into JS: {from} to {to}: {:#?}",
                    message
                )
            }
            Error::MissingArgs {
                ref expected,
                ref given,
            } => {
                error!("Missing arguments. Expected {expected} but got {given}.")
            }
            Error::TooManyArgs {
                ref expected,
                ref given,
            } => {
                error!("Too many arguments. Expected {expected} but got {given}.")
            }

            Error::UnrelatedRuntime => {
                error!(
                    "Tried to restore a Persistent in a runtime other than the original runtime."
                );
            }

            _ => {
                error!("An unknown error occurred: {err}");
            }
        }
        debug!("Raw error: {:#?}", err);
        debug!("Catch: {:#?}", catch);

        error!("Finished Task '{}' => FAILURE", task.id);

        panic!("Exiting due to error in '{}'", task.id);
    } else {
        info!("Finished Task '{}' => SUCCESS", task.id);
    }
}
