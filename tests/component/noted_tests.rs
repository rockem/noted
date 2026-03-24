use noted::version::VERSION;

use crate::support::{AppDriver, StoreDriver};

#[test]
fn show_version() {
    let app = AppDriver::new(StoreDriver::new().path());

    let output = app.run_with_args(&["--version"]).unwrap();

    assert!(
        output.stdout.contains(VERSION),
        "Expected version in stdout, got: {}",
        output.stdout
    );
}
