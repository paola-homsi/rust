use std::env;

mod parser;
mod models;
mod executer;
use models::CmdRequest;


fn main() {
    let arguments: Vec<String> = env::args().collect();
    let req: CmdRequest = parser::parse(arguments);
    executer::execute(req);
}
