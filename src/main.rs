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
    fn quit_command(&self, _args: &[String]) -> CommandResult {
        println!("Quitting");
        CommandResult::Quit
    }
    
    #[cmd(insert)]
    fn insert_command(&self, _args: &[String]) -> CommandResult {
        println!("Inserted");
        CommandResult::Ok
    }

    #[cmd(select)]
    fn select_command(&self, _args: &[String]) -> CommandResult {
        println!("Selected");
        CommandResult::Ok
    }

    #[cmd(delete)]
    fn delete_command(&self, _args: &[String]) -> CommandResult {
        println!("Deleted");
        CommandResult::Ok
    }

    #[cmd(update)]
    fn update_command(&self, _args: &[String]) -> CommandResult {
        println!("Updated");
        CommandResult::Ok
    }
    
    #[cmd(list)]
    fn list_command(&self, _args: &[String]) -> CommandResult {
        println!("Listed");
        CommandResult::Ok
    }
}

/// Main function that creates the scope and starts a command loop for it
fn main() {
    cmd_loop(&mut SQLScope {});
}

struct PrepareResult{ }

trait Statement {
    fn prepare(args: &[String]) -> Self;
    fn execute();
}

struct DeleteStatement{
    prep : PrepareResult
}

impl Statement for DeleteStatement{

    fn prepare(args: &[String]) -> Self
    {
        DeleteStatement{prep: PrepareResult{}}
    }
    fn execute()
    {

    }
}


struct InsertStatement{
    prep : PrepareResult,
}

impl Statement for InsertStatement{

    fn prepare(args: &[String]) -> Self
    {
        InsertStatement{prep: PrepareResult{}}
    }
    fn execute()
    {
        
    }
}