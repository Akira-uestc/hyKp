mod sig;
mod save;
mod reload;

use sig::signal_shutdown;

fn main() {
    crate::reload::restore_window();
    loop {
        if signal_shutdown() {
            crate::save::save_layout();
        }
    }
}
