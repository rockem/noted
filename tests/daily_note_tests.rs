mod support;
use noted::errors::DAILY_NOTE_CREATE_FAILED;
use noted::version::VERSION;

use support::{AppDriver, StoreDriver};

fn setup() -> (AppDriver, StoreDriver) {
    let store = StoreDriver::new();
    let app = AppDriver::new(store.path());
    (app, store)
}

#[test]
fn create_daily_note_file() {
    let (app, store) = setup();
    app.run().unwrap();
    store.today_note_file_created();
}

#[test]
fn edit_existing_daily_note() {
    let (app, store) = setup();

    let expected_content = "existing content";

    store.create_today_note(expected_content);
    let output = app.run().unwrap();

    assert!(output.stdout.contains(expected_content));
}

#[test]
fn fail_to_create_daily_note() {
    let store = StoreDriver::new_with_create();
    let app = AppDriver::new(store.path());

    store.make_read_only();

    let err = app.run().expect_err("Expected to fail");

    assert!(
        err.contains(DAILY_NOTE_CREATE_FAILED),
        "Expected error message in stderr, got: {}",
        err
    );
}

#[test]
fn show_version() {
    let (app, _store) = setup();
    let output = app.run_with_args(&["--version"]).unwrap();

    assert!(
        output.stdout.contains(VERSION),
        "Expected version in stdout, got: {}",
        output.stdout
    );
}
