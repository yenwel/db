extern crate cmdr;

use cmdr::*;

struct SQLScope {}

/// Example scope that implements two commands, greet and quit
#[cmdr]
impl SQLScope {
     
    fn prompt(&self) -> String {
        "db>".to_string()
    }   

    /// Cmdr command to quit the application by returning CommandResult::Quit
    #[cmd(quit, help = "Quit the application", alias(exit, x, q))]
    fn quit_method(&self, _args: &[String]) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
    
    #[cmd]
    fn execute_method(&self, _args: &[String]) -> CommandResult {

        CommandResult::Ok
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut SQLScope {});
}