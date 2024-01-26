use crate::runner::TaskRunner;
use crate::task::Task;

#[test]
pub fn env_module() {
    TaskRunner::new().run(&Task {
        id: "Test env module",
        about: "Tests the env module",
        src: r#"
import { set, get, remove } from "env";

// Test set and get of env variables
set("FOO", "bar");
assert(get("FOO") == "bar");

// Test removing env variables
remove("FOO");
assert(get("FOO") == null);
        "#,
        pre_run: &[],
        post_run: &[],
    });
}

#[test]
pub fn fs_module() {
    TaskRunner::new().run(&Task {
        id: "Test fs module",
        about: "Tests the fs module",
        src: r#"
import { readFile, writeFile, createFile, removeFile, createDir, removeDir, exists } from "fs";

// Test reading and writing files
createFile("FS_MODULE_UNIT_TEST.txt");
writeFile("FS_MODULE_UNIT_TEST.txt", "hello world!");
assert(readFile("FS_MODULE_UNIT_TEST.txt") == "hello world!");
assert(exists("FS_MODULE_UNIT_TEST.txt") == true);
removeFile("FS_MODULE_UNIT_TEST.txt");
        "#,
        pre_run: &[],
        post_run: &[],
    });
}

#[test]
pub fn tasks() {
    TaskRunner::new().run(&Task {
        id: "Test Tasks",
        about: "Tests if Tasks and post/pre runs work",
        src: r#"
import { get, remove } from "env";
assert(get("FOO") == "bar");
remove("FOO");

assert(ID == "Test Tasks");
assert(ABOUT == "Tests if Tasks and post/pre runs work");
        "#,
        pre_run: &[Task {
            id: "Set FOO",
            about: "Sets FOO",
            src: r#"
import { set } from "env";
set("FOO", "bar");
                "#,
            pre_run: &[],
            post_run: &[],
        }],
        post_run: &[Task {
            id: "Get FOO",
            about: "Gets FOO",
            src: r#"
import { get } from "env";
assert(get("FOO") == null);
                "#,
            pre_run: &[],
            post_run: &[],
        }],
    });
}

#[test]
#[should_panic]
pub fn throw() {
    TaskRunner::new().run(&Task {
        id: "Test throw()",
        about: "Tests if throw() works",
        src: r#"
throw("This is a test throw()");
        "#,
        pre_run: &[],
        post_run: &[],
    });
}
