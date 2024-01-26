use rquickjs::{Ctx, Error, Function, Value};

use crate::js::cast_to_string;

pub fn setup(ctx: &Ctx, id: &str, about: &str) -> Result<(), Error> {
    let globals = ctx.globals();

    globals.set(
        "print",
        Function::new(
            ctx.clone(),
            |msg: Value| print!("{}", cast_to_string(&msg),),
        ),
    )?;

    globals.set(
        "println",
        Function::new(ctx.clone(), |msg: Value| {
            println!("{}", cast_to_string(&msg),)
        }),
    )?;

    globals.set(
        "assert",
        Function::new(ctx.clone(), |x: bool| {
            // TODO: Add custom assertion message
            assert!(x, "JavaScript Assertion Failed");
        }),
    )?;

    globals.set(
        "throw",
        Function::new(ctx.clone(), |msg: Value| {
            panic!("JS Exception Thrown: {}", cast_to_string(&msg));
        }),
    )?;

    globals.set("ID", id)?;
    globals.set("ABOUT", about)?;

    Ok(())
}
