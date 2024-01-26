use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Ctx, Function};

pub struct Module;

impl ModuleDef for Module {
    #[inline(always)]
    fn declare(declarations: &mut Declarations) -> rquickjs::Result<()> {
        declarations.declare("cargo")?;
        declarations.declare("run")?;
        declarations.declare("build")?;
        declarations.declare("test")?;
        declarations.declare("clean")?;
        declarations.declare("release")?;

        Ok(())
    }

    #[inline(always)]
    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> rquickjs::Result<()> {
        exports.export("cargo", Function::new(ctx.clone(), Self::cargo))?;
        exports.export("run", Function::new(ctx.clone(), Self::run))?;
        exports.export("build", Function::new(ctx.clone(), Self::build))?;
        exports.export("test", Function::new(ctx.clone(), Self::test))?;
        exports.export("clean", Function::new(ctx.clone(), Self::clean))?;
        exports.export("release", Function::new(ctx.clone(), Self::release))?;

        Ok(())
    }
}

impl Module {
    #[inline(always)]
    fn cargo(cmd: String) -> Option<i32> {
        std::process::Command::new(format!("cargo {}", cmd))
            .spawn()
            .ok()?
            .wait()
            .ok()?
            .code()
    }

    #[inline(always)]
    fn run() -> Option<i32> {
        std::process::Command::new("cargo run")
            .spawn()
            .ok()?
            .wait()
            .ok()?
            .code()
    }

    #[inline(always)]
    fn build() -> Option<i32> {
        std::process::Command::new("cargo build")
            .spawn()
            .ok()?
            .wait()
            .ok()?
            .code()
    }

    #[inline(always)]
    fn test(test: String) -> Option<i32> {
        std::process::Command::new(format!("cargo test --nocapture --test {}", test))
            .spawn()
            .ok()?
            .wait()
            .ok()?
            .code()
    }

    #[inline(always)]
    fn clean() -> Option<i32> {
        std::process::Command::new("cargo clean")
            .spawn()
            .ok()?
            .wait()
            .ok()?
            .code()
    }

    #[inline(always)]
    fn release() -> Option<i32> {
        std::process::Command::new(
            "cargo build --release --config strip=true --future-incompat-report",
        )
        .spawn()
        .ok()?
        .wait()
        .ok()?
        .code()
    }
}
