//! RINF bridge entry point for the Hyprbaric native runtime.

use rinf::write_interface;

write_interface!();

fn main() {
    hyprbaric::start();
}
