use crate::state::TerminalInterface;

pub mod actions;
pub mod effect;
pub mod models;
pub mod state;

fn main() {
    println!("Volo's Virtual Toolkit");
    TerminalInterface::new().run();
}
