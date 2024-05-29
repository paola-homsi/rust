use std::fs;
use crate::models::CmdRequest;

pub fn execute(request: CmdRequest) {
    let content: String = match fs::read_to_string(request.get_name()) {
        Ok(content) => content,
        Err(error) =>  format!("error {}", error)
    };
    if(content.starts_with("error")){
        println!("error reading file");
        return;
    }
    for action in request.get_actions() {
        match &action[..] {
            "size" => calc_size(&content),
            "lines" => count_lines(&content),
            "help" => print_help(),
            _ =>  print_unkown(),
        }
    }
}

fn calc_size(content: &String) {
    println!("file size: {} bytes.", content.len());
}
fn count_lines(content: &String) {
    let lines = content.matches('\n').count() + 1;
    println!("no of lines: {} .", lines);
}
fn print_unkown() {
    println!("unkown command. use comand -h or --help for more information about the commands");
}
fn print_help() {
    println!("available commands: \n - -s or --size. to get file size \n -l or --lines. to get number of lines");
}