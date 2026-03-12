mod support;

use support::{AppDriver, StoreDriver};

fn setup() -> (AppDriver, StoreDriver) {
    let store = StoreDriver::new();
    let app = AppDriver::new(store.path());
    (app, store)
}

#[test]
fn create_daily_note_file() {
    let (app, store) = setup();
    let output = app.run();
    assert!(
        output.success,
        "noted command should succeed\nstderr: {}",
        output.stderr
    );
    store.today_note_file_created();
}

#[test]
fn edit_existing_daily_note() {
    let (app, store) = setup();

    let expected_content = "existing content";

    store.create_today_note(expected_content);
    let output = app.run();

    assert!(output.stdout.contains(expected_content));
}

#[test]
fn fail_to_create_daily_note() {
    let store = StoreDriver::new_with_create();
    let app = AppDriver::new(store.path());

    store.make_read_only();

    let output = app.run();

    assert!(!output.success);
    assert!(
        output.stderr.contains("Error: Failed to create daily note"),
        "Expected error message in stderr, got: {}",
        output.stderr
    );
}
