use crate::runner::TaskRunner;
use crate::task::Task;

#[test]
pub fn env_module() {
    crate::utils::init_logger().unwrap_or(());

    TaskRunner::new().run(&Task {
        id: "Test env module".to_string(),
        about: "Tests the env module".to_string(),
        src: r#"
import { set, get, remove } from "env";

// Test set and get of env variables
set("MY_ENV", "hello");
assert(get("MY_ENV") == "hello");

// Test removing env variables
remove("MY_ENV");
assert(get("MY_ENV") == null);
        "#.to_string(),
        pre_run: Vec::new(),
        post_run: Vec::new(),
    });
}

#[test]
pub fn fs_module() {
    crate::utils::init_logger().unwrap_or(());

    TaskRunner::new().run(&Task {
        id: "Test fs module".to_string(),
        about: "Tests the fs module".to_string(),
        src: r#"
import { readFile, writeFile, createFile, removeFile, createDir, removeDir, exists } from "fs";

// Test reading and writing files
createFile("FS_MODULE_UNIT_TEST.txt");
writeFile("FS_MODULE_UNIT_TEST.txt", "hello world!");
assert(readFile("FS_MODULE_UNIT_TEST.txt") == "hello world!");
assert(exists("FS_MODULE_UNIT_TEST.txt") == true);
removeFile("FS_MODULE_UNIT_TEST.txt");
        "#.to_string(),
        pre_run: Vec::new(),
        post_run: Vec::new(),
    });
}

#[test]
pub fn tasks() {
    crate::utils::init_logger().unwrap_or(());

    TaskRunner::new().run(&Task {
        id: "Test Tasks".to_string(),
        about: "Tests if Tasks and post/pre runs work".to_string(),
        src: r#"
import { get, remove } from "env";
assert(get("FOO") == "bar");
remove("FOO");

assert(ID == "Test Tasks");
assert(ABOUT == "Tests if Tasks and post/pre runs work");
        "#.to_string(),
        pre_run: vec![Task {
            id: "Set FOO".to_string(),
            about: "Sets FOO".to_string(),
            src: r#"
import { set } from "env";
set("FOO", "bar");
                "#.to_string(),
            pre_run: Vec::new(),
            post_run: Vec::new(),
        }],
        post_run: vec![Task {
            id: "Get FOO".to_string(),
            about: "Gets FOO".to_string(),
            src: r#"
import { get } from "env";
assert(get("FOO") == null);
                "#.to_string(),
            pre_run: Vec::new(),
            post_run: Vec::new(),
        }],
    });
}

#[test]
#[should_panic]
pub fn throw() {
    crate::utils::init_logger().unwrap_or(());

    TaskRunner::new().run(&Task {
        id: "Test throw()".to_string(),
        about: "Tests if throw() works".to_string(),
        src: r#"
throw("This is a test throw()");
        "#.to_string(),
        pre_run: Vec::new(),
        post_run: Vec::new(),
    });
}
