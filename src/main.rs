use jni::{InitArgs, InitArgsBuilder, JNIVersion};
use std::sync::Arc;
use std::thread::{spawn, sleep};
use std::mem::forget;
use std::time::Duration;
use crate::executors::{HackyExecutor, JniExecutor};

mod executors;

fn main() {
    let jvm = jni::JavaVM::new(InitArgsBuilder::new().version(JNIVersion::V8).build().unwrap()).unwrap();
    let jvm = Arc::new(jvm);

    let executor = HackyExecutor::new(jvm, 2);

    let executor1 = executor.clone();

    let thread1 = spawn(move || {
        executor1.with_attached(|env| {
            println!("Some operation");
            sleep(Duration::from_millis(1000));
            println!("Finished");
            Ok(())
        }).unwrap();
    });

    let executor2 = executor.clone();

    let thread2 = spawn(move || {
        executor2.with_attached(|env| {
            println!("Another operation");
            sleep(Duration::from_millis(500));
            println!("Finished 2");
            Ok(())
        }).unwrap();
    });

    sleep(Duration::from_millis(2000));
}
