use jni::{InitArgs, InitArgsBuilder, JNIVersion};
use std::sync::Arc;
use std::thread::{spawn, sleep};
use std::mem::forget;
use std::time::Duration;

fn main() {
    let init_args = InitArgsBuilder::new().version(JNIVersion::V8)
        .option("-XX:+HeapDumpOnOutOfMemoryError")
        .option("-Xmx4m").build().unwrap();
    let jvm = jni::JavaVM::new(init_args).unwrap();
    let jvm = Arc::new(jvm);

    let mut counter = 0;
    loop {
        let jvm_clone = jvm.clone();
        let thread = spawn(move || {
            let env = jvm_clone.attach_current_thread().unwrap();
            forget(env);
        });
        sleep(Duration::from_millis(1));
        counter += 1;
        if counter % 100 == 0 {
            println!("{}", counter);
        }
    }
}
