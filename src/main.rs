use recover::Recover;
use log::error;

mod recover;
mod utils;

fn main() {
    let recover = Recover::new();
    match recover.run() {
        Ok(()) => (),
        Err(e) => error!("{}", e)
    }
}