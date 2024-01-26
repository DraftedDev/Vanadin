use log::info;
use rquickjs::context::{intrinsic, EvalOptions};
use rquickjs::loader::{BuiltinLoader, BuiltinResolver, FileResolver, ModuleLoader, ScriptLoader};
use rquickjs::{Context, Ctx, Runtime};

use crate::error::evaluate_result;
use crate::modules;
use crate::task::Task;

#[derive(Clone)]
pub struct TaskRunner {
    ctx: Context,
}

impl TaskRunner {
    #[inline(always)]
    pub fn new() -> Self {
        let rt = Runtime::new().expect("Failed to create QuickJS runtime");

        let resolver = (
            FileResolver::default(),
            BuiltinResolver::default()
                .with_module("env")
                .with_module("process")
                .with_module("math")
                .with_module("log")
                .with_module("fs")
                .with_module("sys")
                .with_module("tools/node")
                .with_module("tools/python")
                .with_module("tools/cargo"),
        );

        let loader = (
            BuiltinLoader::default(),
            ScriptLoader::default(),
            ModuleLoader::default()
                .with_module("env", modules::env::Module)
                .with_module("process", modules::process::Module)
                .with_module("math", modules::math::Module)
                .with_module("log", modules::log::Module)
                .with_module("fs", modules::fs::Module)
                .with_module("sys", modules::sys::Module)
                .with_module("tools/node", modules::tools::node::Module)
                .with_module("tools/python", modules::tools::python::Module)
                .with_module("tools/cargo", modules::tools::cargo::Module),
        );

        rt.set_loader(resolver, loader);

        Self {
            ctx: Context::builder()
                .with::<intrinsic::Eval>()
                .with::<intrinsic::BaseObjects>()
                .build(&rt)
                .expect("Failed to create QuickJS context"),
        }
    }

    #[inline(always)]
    pub fn run(&self, task: &Task) {
        info!("Running Task => '{}'", task.id);

        self.ctx.with(|ctx| {
            modules::globals::setup(&ctx, &task.id, &task.about).expect("Failed to setup globals");
            self.run_task(task, &ctx);
        });
    }

    #[inline(always)]
    #[allow(clippy::only_used_in_recursion)]
    pub fn run_task(&self, task: &Task, ctx: &Ctx) {
        for pre in &task.pre_run {
            self.run_task(pre, ctx);
        }

        let result = ctx.eval_with_options::<(), &str>(
            task.src.as_str(),
            EvalOptions {
                global: false,
                strict: false,
                backtrace_barrier: false,
            },
        );
        evaluate_result(result, task, ctx);

        for post in &task.post_run {
            self.run_task(post, ctx);
        }
    }
}

impl Default for TaskRunner {
    fn default() -> Self {
        Self::new()
    }
}
