use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Ctx, Function, Value};

use crate::js::cast_to_string;

pub struct Module;

impl ModuleDef for Module {
    #[inline(always)]
    fn declare(declarations: &mut Declarations) -> rquickjs::Result<()> {
        declarations.declare("set")?;
        declarations.declare("get")?;
        declarations.declare("remove")?;
        declarations.declare("dir")?;
        declarations.declare("os")?;
        declarations.declare("arch")?;
        declarations.declare("family")?;

        Ok(())
    }

    #[inline(always)]
    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> rquickjs::Result<()> {
        exports.export("set", Function::new(ctx.clone(), Self::set))?;
        exports.export("get", Function::new(ctx.clone(), Self::get))?;
        exports.export("remove", Function::new(ctx.clone(), Self::remove))?;
        exports.export("dir", Function::new(ctx.clone(), Self::dir))?;
        exports.export("os", Function::new(ctx.clone(), Self::os))?;
        exports.export("arch", Function::new(ctx.clone(), Self::arch))?;
        exports.export("family", Function::new(ctx.clone(), Self::family))?;

        Ok(())
    }
}

impl Module {
    #[inline(always)]
    fn set(key: String, value: Value) {
        std::env::set_var(key, cast_to_string(&value));
    }

    #[inline(always)]
    fn get(key: Value) -> Option<String> {
        std::env::var(key.get::<String>().expect("Value must be string")).ok()
    }

    #[inline(always)]
    pub fn remove(key: String) {
        std::env::remove_var(key);
    }

    #[inline(always)]
    pub fn dir() -> Option<String> {
        Some(std::env::current_dir().ok()?.to_str()?.to_string())
    }

    #[inline(always)]
    pub fn os() -> String {
        std::env::consts::OS.to_string()
    }

    #[inline(always)]
    pub fn arch() -> String {
        std::env::consts::ARCH.to_string()
    }

    #[inline(always)]
    pub fn family() -> String {
        std::env::consts::FAMILY.to_string()
    }
}
