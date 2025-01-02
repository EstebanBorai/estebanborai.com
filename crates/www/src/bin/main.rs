use leptos::{mount_to_body, view};
use leptos_router::Router;

use app::App;

fn main() {
    mount_to_body(|| {
        view! {
            <Router>
                <App/>
            </Router>
        }
    })
}
