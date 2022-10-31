#![warn(clippy::missing_docs_in_private_items)]

//! Binary application that uses the veryrezsi api library and runs the server.

/// Entry point of the application.
fn main() {
    veryrezsi_api::start();
}
