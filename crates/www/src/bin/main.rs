use leptos::{mount_to_body, view};

use app::App;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
