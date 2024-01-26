use std::fs;

use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Ctx, Function};

pub struct Module;

impl ModuleDef for Module {
    #[inline(always)]
    fn declare(declarations: &mut Declarations) -> rquickjs::Result<()> {
        declarations.declare("readFile")?;
        declarations.declare("writeFile")?;
        declarations.declare("readFileBytes")?;
        declarations.declare("writeFileBytes")?;
        declarations.declare("createFile")?;
        declarations.declare("removeFile")?;
        declarations.declare("createDir")?;
        declarations.declare("removeDir")?;
        declarations.declare("exists")?;

        Ok(())
    }

    #[inline(always)]
    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> rquickjs::Result<()> {
        exports.export("readFile", Function::new(ctx.clone(), Self::read_file))?;
        exports.export("writeFile", Function::new(ctx.clone(), Self::write_file))?;
        exports.export(
            "readFileBytes",
            Function::new(ctx.clone(), Self::read_file_bytes),
        )?;
        exports.export(
            "writeFileBytes",
            Function::new(ctx.clone(), Self::write_file_bytes),
        )?;
        exports.export("createFile", Function::new(ctx.clone(), Self::create_file))?;
        exports.export("removeFile", Function::new(ctx.clone(), Self::delete_file))?;
        exports.export("createDir", Function::new(ctx.clone(), Self::create_dir))?;
        exports.export("removeDir", Function::new(ctx.clone(), Self::delete_dir))?;
        exports.export("exists", Function::new(ctx.clone(), Self::exists))?;

        Ok(())
    }
}

impl Module {
    fn read_file(path: String) -> Option<String> {
        fs::read_to_string(path).ok()
    }

    fn write_file(path: String, content: String) {
        fs::write(path, content).expect("Failed writing file");
    }

    fn read_file_bytes(path: String) -> Option<Vec<u8>> {
        fs::read(path).ok()
    }

    fn write_file_bytes(path: String, content: Vec<u8>) {
        fs::write(path, content).expect("Failed writing file");
    }

    fn create_file(path: String) {
        fs::File::create(path).expect("Failed creating file");
    }

    fn delete_file(path: String) {
        fs::remove_file(path).expect("Failed deleting file");
    }

    fn create_dir(path: String) {
        fs::create_dir(path).expect("Failed creating dir");
    }

    fn delete_dir(path: String) {
        fs::remove_dir(path).expect("Failed deleting dir");
    }

    fn exists(path: String) -> bool {
        fs::metadata(path).is_ok()
    }
}
