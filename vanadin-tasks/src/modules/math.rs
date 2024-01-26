use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Ctx, Function};

pub struct Module;

impl ModuleDef for Module {
    #[inline(always)]
    fn declare(declarations: &mut Declarations) -> rquickjs::Result<()> {
        declarations.declare("PI")?;
        declarations.declare("E")?;

        declarations.declare("sqrt")?;
        declarations.declare("pow")?;
        declarations.declare("sin")?;
        declarations.declare("cos")?;
        declarations.declare("tan")?;
        declarations.declare("asin")?;
        declarations.declare("acos")?;
        declarations.declare("atan")?;
        declarations.declare("sinh")?;
        declarations.declare("cosh")?;
        declarations.declare("tanh")?;
        declarations.declare("asinh")?;
        declarations.declare("acosh")?;
        declarations.declare("atanh")?;
        declarations.declare("exp")?;
        declarations.declare("log")?;
        declarations.declare("log2")?;
        declarations.declare("log10")?;
        declarations.declare("floor")?;
        declarations.declare("ceil")?;
        declarations.declare("round")?;

        Ok(())
    }

    #[inline(always)]
    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> rquickjs::Result<()> {
        exports.export("PI", std::f64::consts::PI)?;
        exports.export("E", std::f64::consts::E)?;

        exports.export("pow", Function::new(ctx.clone(), Self::pow))?;
        exports.export("sqrt", Function::new(ctx.clone(), Self::sqrt))?;
        exports.export("sin", Function::new(ctx.clone(), Self::sin))?;
        exports.export("cos", Function::new(ctx.clone(), Self::cos))?;
        exports.export("tan", Function::new(ctx.clone(), Self::tan))?;
        exports.export("asin", Function::new(ctx.clone(), Self::asin))?;
        exports.export("acos", Function::new(ctx.clone(), Self::acos))?;
        exports.export("atan", Function::new(ctx.clone(), Self::atan))?;
        exports.export("sinh", Function::new(ctx.clone(), Self::sinh))?;
        exports.export("cosh", Function::new(ctx.clone(), Self::cosh))?;
        exports.export("tanh", Function::new(ctx.clone(), Self::tanh))?;
        exports.export("asinh", Function::new(ctx.clone(), Self::asinh))?;
        exports.export("acosh", Function::new(ctx.clone(), Self::acosh))?;
        exports.export("atanh", Function::new(ctx.clone(), Self::atanh))?;
        exports.export("exp", Function::new(ctx.clone(), Self::exp))?;
        exports.export("log", Function::new(ctx.clone(), Self::log))?;
        exports.export("log2", Function::new(ctx.clone(), Self::log2))?;
        exports.export("log10", Function::new(ctx.clone(), Self::log10))?;
        exports.export("floor", Function::new(ctx.clone(), Self::floor))?;
        exports.export("ceil", Function::new(ctx.clone(), Self::ceil))?;
        exports.export("round", Function::new(ctx.clone(), Self::round))?;

        Ok(())
    }
}

impl Module {
    #[inline(always)]
    fn pow(base: f64, exp: f64) -> f64 {
        base.powf(exp)
    }

    #[inline(always)]
    fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    #[inline(always)]
    fn sin(x: f64) -> f64 {
        x.sin()
    }

    #[inline(always)]
    fn cos(x: f64) -> f64 {
        x.cos()
    }

    #[inline(always)]
    fn tan(x: f64) -> f64 {
        x.tan()
    }

    #[inline(always)]
    fn asin(x: f64) -> f64 {
        x.asin()
    }

    #[inline(always)]
    fn acos(x: f64) -> f64 {
        x.acos()
    }

    #[inline(always)]
    fn atan(x: f64) -> f64 {
        x.atan()
    }

    #[inline(always)]
    fn sinh(x: f64) -> f64 {
        x.sinh()
    }

    #[inline(always)]
    fn cosh(x: f64) -> f64 {
        x.cosh()
    }

    #[inline(always)]
    fn tanh(x: f64) -> f64 {
        x.tanh()
    }

    #[inline(always)]
    fn asinh(x: f64) -> f64 {
        x.asinh()
    }

    #[inline(always)]
    fn acosh(x: f64) -> f64 {
        x.acosh()
    }

    #[inline(always)]
    fn atanh(x: f64) -> f64 {
        x.atanh()
    }

    #[inline(always)]
    fn exp(x: f64) -> f64 {
        x.exp()
    }

    #[inline(always)]
    fn log(x: f64, base: f64) -> f64 {
        x.log(base)
    }

    #[inline(always)]
    fn log2(x: f64) -> f64 {
        x.log2()
    }

    #[inline(always)]
    fn log10(x: f64) -> f64 {
        x.log10()
    }

    #[inline(always)]
    fn floor(x: f64) -> f64 {
        x.floor()
    }

    #[inline(always)]
    fn ceil(x: f64) -> f64 {
        x.ceil()
    }

    #[inline(always)]
    fn round(x: f64) -> f64 {
        x.round()
    }
}
