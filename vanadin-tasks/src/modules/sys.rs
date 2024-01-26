use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Ctx, Function};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct Module;

impl ModuleDef for Module {
    #[inline(always)]
    fn declare(declarations: &mut Declarations) -> rquickjs::Result<()> {
        declarations.declare("cpuCount")?;
        declarations.declare("cpuBrands")?;
        declarations.declare("cpuNames")?;
        declarations.declare("cpuFrequencies")?;
        declarations.declare("cpuUsages")?;
        declarations.declare("availableMemory")?;
        declarations.declare("freeMemory")?;
        declarations.declare("totalMemory")?;
        declarations.declare("usedMemory")?;

        Ok(())
    }

    #[inline(always)]
    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> rquickjs::Result<()> {
        exports.export("cpuCount", Function::new(ctx.clone(), Self::cpu_count))?;
        exports.export("cpuBrands", Function::new(ctx.clone(), Self::cpu_brands))?;
        exports.export("cpuNames", Function::new(ctx.clone(), Self::cpu_names))?;
        exports.export(
            "cpuFrequencies",
            Function::new(ctx.clone(), Self::cpu_frequencies),
        )?;
        exports.export("cpuUsages", Function::new(ctx.clone(), Self::cpu_usage))?;
        exports.export(
            "availableMemory",
            Function::new(ctx.clone(), Self::available_memory),
        )?;
        exports.export("freeMemory", Function::new(ctx.clone(), Self::free_memory))?;
        exports.export(
            "totalMemory",
            Function::new(ctx.clone(), Self::total_memory),
        )?;
        exports.export("usedMemory", Function::new(ctx.clone(), Self::used_memory))?;

        Ok(())
    }
}

impl Module {
    fn cpu_count() -> Option<usize> {
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::new()))
            .physical_core_count()
    }

    #[inline(always)]
    fn cpu_brands() -> Vec<String> {
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::new()))
            .cpus()
            .iter()
            .map(|e| e.brand().to_string())
            .collect()
    }

    #[inline(always)]
    fn cpu_names() -> Vec<String> {
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::new()))
            .cpus()
            .iter()
            .map(|e| e.name().to_string())
            .collect()
    }

    #[inline(always)]
    fn cpu_frequencies() -> Vec<u64> {
        System::new_with_specifics(
            RefreshKind::new().with_cpu(CpuRefreshKind::new().with_frequency()),
        )
        .cpus()
        .iter()
        .map(|e| e.frequency())
        .collect()
    }

    #[inline(always)]
    fn cpu_usage() -> Vec<f32> {
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::new()))
            .cpus()
            .iter()
            .map(|e| e.cpu_usage())
            .collect()
    }

    #[inline(always)]
    fn available_memory() -> u64 {
        System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
        )
        .available_memory()
    }

    #[inline(always)]
    fn free_memory() -> u64 {
        System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
        )
        .free_memory()
    }

    #[inline(always)]
    fn total_memory() -> u64 {
        System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
        )
        .total_memory()
    }

    #[inline(always)]
    fn used_memory() -> u64 {
        System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
        )
        .used_memory()
    }
}
