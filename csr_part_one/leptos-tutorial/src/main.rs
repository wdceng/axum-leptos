use leptos::prelude::*;

fn main() {
    // Set a panic hook to log Rust panics to the browser console (useful for debugging in WASM)
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| {
        view! { <p>"Hello, world!"</p> }
    });
}
