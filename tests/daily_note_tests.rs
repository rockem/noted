mod support;

use support::{AppDriver, StoreDriver};

#[test]
fn create_daily_note_file() {
    let store = StoreDriver::new();
    let app = AppDriver::new(store.path());

    app.run();
    store.today_note_file_created();
}
