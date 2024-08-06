mod components;
use components::navbar::Navbar;
use components::counter::Counter;
use leptos::*;

use log::{Level, LevelFilter, Log, Metadata, Record};

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <Navbar />
            <main>
                <h1>"Welcome to My App!"</h1>
            </main>
        </div>

        <Counter initial_value=5 />
    }
}

fn main() {
    log::set_logger(&SimpleLogger).expect("Failed to set logger");
    log::set_max_level(LevelFilter::Info);
    mount_to_body(App);
}