extern crate cmdr;

use cmdr::*;

struct SQLScope {}

/// Example scope that implements two commands, greet and quit
#[cmdr]
impl SQLScope {   

    /// Cmdr command to quit the application by returning CommandResult::Quit
    #[cmd]
    fn quit(&self, _args: &[String]) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut SQLScope {});
}