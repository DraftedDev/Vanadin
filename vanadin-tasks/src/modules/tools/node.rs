use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Ctx, Function};

pub struct Module;

impl ModuleDef for Module {
    #[inline(always)]
    fn declare(declarations: &mut Declarations) -> rquickjs::Result<()> {
        declarations.declare("node")?;
        declarations.declare("run")?;
        declarations.declare("launch")?;

        Ok(())
    }

    #[inline(always)]
    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> rquickjs::Result<()> {
        exports.export("node", Function::new(ctx.clone(), Self::node))?;
        exports.export("run", Function::new(ctx.clone(), Self::run))?;
        exports.export("launch", Function::new(ctx.clone(), Self::launch))?;

        Ok(())
    }
}

impl Module {
    #[inline(always)]
    fn node(cmd: String) -> Option<i32> {
        std::process::Command::new(format!("node {}", cmd))
            .spawn()
            .ok()?
            .wait()
            .ok()?
            .code()
    }

    #[inline(always)]
    fn run(file: String) -> Option<i32> {
        std::process::Command::new(format!(
            "node --pending-deprecation --report-uncaught-exception {}",
            file
        ))
        .spawn()
        .ok()?
        .wait()
        .ok()?
        .code()
    }

    #[inline(always)]
    fn launch(file: String) -> Option<i32> {
        std::process::Command::new(format!("node {}", file))
            .spawn()
            .ok()?
            .wait()
            .ok()?
            .code()
    }
}
