use gloo_worker::Registrable;
use plus_client::NormalEntries;

fn main() {
    console_error_panic_hook::set_once();
    NormalEntries::registrar().register();
}