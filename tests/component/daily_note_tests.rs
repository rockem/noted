use noted::errors::DAILY_NOTE_CREATE_FAILED;
use regex::Regex;

use crate::support::{AppDriver, StoreDriver};

#[test]
fn create_daily_note_file() {
    let store = StoreDriver::new();
    let app = AppDriver::new(store.path()).with_editor("touch");
    app.run().unwrap();
    store.today_note_file_created();
}

#[test]
fn edit_existing_daily_note() {
    let store = StoreDriver::new();
    let app = AppDriver::new(store.path()).with_editor("cat");

    let expected_content = "existing content";

    store.create_today_note(expected_content);
    let output = app.run().unwrap();

    assert!(output.stdout.contains(expected_content));
}

#[test]
fn fail_to_create_daily_note_path() {
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
fn quick_capture_creates_daily_note_if_not_exists() {
    let store = StoreDriver::new();
    let app = AppDriver::new(store.path());
    let capture_text = "some text";

    app.run_with_args(&["-e", capture_text]).unwrap();
    assert!(store.today_note_content().contains(capture_text));
}

#[test]
fn quick_capture_with_piped_input() {
    let store = StoreDriver::new();
    let app = AppDriver::new(store.path());

    app.run_with_stdin(&["-e"], "piped text").unwrap();

    assert!(store.today_note_content().contains("piped text"));
}

#[test]
fn quick_capture_appends_to_existing_note() {
    let store = StoreDriver::new();
    let app = AppDriver::new(store.path());
    store.create_today_note("existing content");

    app.run_with_args(&["-e", "appended text"]).unwrap();

    let content = store.today_note_content();
    assert!(
        Regex::new(r"existing content\n.*appended text")
            .unwrap()
            .is_match(&content),
        "text wasn't matched in: {}",
        &content
    );
}
