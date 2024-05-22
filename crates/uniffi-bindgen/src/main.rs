// mod args;
// mod bindgen_error;
// mod post_process;
// mod post_process_kotlin;
// mod post_process_swift;

// use crate::post_process::*;

fn main() {
    // println!("🔮 Running sargon-bindgen");
    uniffi::uniffi_bindgen_main();
    // post_process();
    // println!("🔮 Finished with sargon-bindgen ✅");
}
