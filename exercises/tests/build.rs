//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // For tests7, we need to set an environment variable called `TEST_FOO`.
    // This variable should contain a timestamp that will be checked by the test.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // For tests8, we need to enable a feature called "pass".
    // This tells Cargo to activate the "pass" feature when compiling the crate.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
