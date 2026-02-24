mod support;

use support::{AppDriver, StoreDriver};

#[test]
fn create_daily_note_file() {
    let store = StoreDriver::new();
    let app = AppDriver::new(store.path());

    app.run();
    store.today_note_file_created();
}

#[test]
fn edit_existing_daily_note() {
    let store = StoreDriver::new();
    let app = AppDriver::new(store.path());
    let expected_content = "existing content";

    store.create_today_note(expected_content);
    let output = app.run();

    assert!(output.contains(expected_content));
}
