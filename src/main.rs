mod components;
use components::counter::Counter;
use components::navbar::Navbar;
use leptos::*;
use leptos_router::{Route, RouteProps, Router, RouterProps, Routes, RoutesProps, A};

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
        <Router>
        <div>
            <Navbar />
            <main>
                <Routes>
                    <Route path="/" view=|| view! {
                        <div>"Welcome to my app!"</div>
                        <Counter initial_value=5 />
                    }/>
                    <Route path="/about" view=|| view! {
                        <div>"About my app!"</div>
                    }/>
                    <Route path="/contact" view=|| view! {
                        <div>"Contact me!"</div>
                        <A href="/">"Go to home"</A>
                    }/>
                </Routes>
            </main>
        </div>

        
        </Router>
    }
}

fn main() {
    log::set_logger(&SimpleLogger).expect("Failed to set logger");
    log::set_max_level(LevelFilter::Info);
    mount_to_body(App);
}
