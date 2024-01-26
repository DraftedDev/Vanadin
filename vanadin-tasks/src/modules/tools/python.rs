use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Ctx, Function};

pub struct Module;

impl ModuleDef for Module {
    #[inline(always)]
    fn declare(declarations: &mut Declarations) -> rquickjs::Result<()> {
        declarations.declare("py")?;
        declarations.declare("run")?;
        declarations.declare("launch")?;

        Ok(())
    }

    #[inline(always)]
    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> rquickjs::Result<()> {
        exports.export("py", Function::new(ctx.clone(), Self::py))?;
        exports.export("run", Function::new(ctx.clone(), Self::run))?;
        exports.export("launch", Function::new(ctx.clone(), Self::launch))?;

        Ok(())
    }
}

impl Module {
    #[inline(always)]
    fn py(cmd: String) -> Option<i32> {
        std::process::Command::new(format!("python {}", cmd))
            .spawn()
            .ok()?
            .wait()
            .ok()?
            .code()
    }

    #[inline(always)]
    fn run(file: Option<String>) -> Option<i32> {
        std::process::Command::new(format!(
            "python {} -X dev -b",
            file.unwrap_or("__main__.py".to_string())
        ))
        .spawn()
        .ok()?
        .wait()
        .ok()?
        .code()
    }

    #[inline(always)]
    fn launch(file: Option<String>) -> Option<i32> {
        std::process::Command::new(format!(
            "python {} -00",
            file.unwrap_or("__main__.py".to_string())
        ))
        .spawn()
        .ok()?
        .wait()
        .ok()?
        .code()
    }
}
