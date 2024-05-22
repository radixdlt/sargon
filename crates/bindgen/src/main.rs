/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

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
