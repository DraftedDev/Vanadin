use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Ctx, Function};

pub struct Module;

impl ModuleDef for Module {
    #[inline(always)]
    fn declare(declarations: &mut Declarations) -> rquickjs::Result<()> {
        declarations.declare("cmd")?;
        declarations.declare("exit")?;
        declarations.declare("id")?;

        Ok(())
    }

    #[inline(always)]
    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> rquickjs::Result<()> {
        exports.export("cmd", Function::new(ctx.clone(), Self::cmd))?;
        exports.export("exit", Function::new(ctx.clone(), Self::exit))?;
        exports.export("id", Function::new(ctx.clone(), Self::id))?;

        Ok(())
    }
}

impl Module {
    #[inline(always)]
    fn cmd(program: String, args: Vec<String>) -> Option<i32> {
        std::process::Command::new(program)
            .args(args)
            .spawn()
            .ok()?
            .wait()
            .ok()?
            .code()
    }

    #[inline(always)]
    fn exit(code: i32) {
        std::process::exit(code);
    }

    #[inline(always)]
    fn id() -> u32 {
        std::process::id()
    }
}
