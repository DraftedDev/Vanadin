use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Ctx, Function, Value};

use crate::js::cast_to_string;

pub struct Module;

impl ModuleDef for Module {
    #[inline(always)]
    fn declare(declarations: &mut Declarations) -> rquickjs::Result<()> {
        declarations.declare("info")?;
        declarations.declare("warn")?;
        declarations.declare("error")?;
        declarations.declare("debug")?;
        declarations.declare("trace")?;

        Ok(())
    }

    #[inline(always)]
    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> rquickjs::Result<()> {
        exports.export("info", Function::new(ctx.clone(), Self::info))?;
        exports.export("warn", Function::new(ctx.clone(), Self::warn))?;
        exports.export("error", Function::new(ctx.clone(), Self::error))?;
        exports.export("debug", Function::new(ctx.clone(), Self::debug))?;
        exports.export("trace", Function::new(ctx.clone(), Self::trace))?;

        Ok(())
    }
}

impl Module {
    #[inline(always)]
    fn info(msg: Value) {
        log::info!("{}", cast_to_string(&msg),)
    }

    #[inline(always)]
    fn warn(msg: Value) {
        log::warn!("{}", cast_to_string(&msg),)
    }

    #[inline(always)]
    fn error(msg: Value) {
        log::error!("{}", cast_to_string(&msg),)
    }

    #[inline(always)]
    fn debug(msg: Value) {
        log::debug!("{}", cast_to_string(&msg),)
    }

    #[inline(always)]
    fn trace(msg: Value) {
        log::trace!("{}", cast_to_string(&msg))
    }
}
