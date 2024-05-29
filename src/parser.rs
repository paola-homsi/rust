use crate::models::CmdRequest;

pub fn parse(args: Vec<String>) -> CmdRequest {
    let mut req = CmdRequest::new("My Request".to_string());
    for (index, arg) in args.iter().enumerate() {
        if index == 0 {
            continue;
        }        
        if arg == "-n" || arg == "--name" {
            req.set_name(args[index+1].to_string());
            continue;
        }
        else if arg == "-s" || arg == "--size" {
            req.add_action("size".to_string());
        }
        else if arg == "-l" || arg == "--lines" {
            req.add_action("lines".to_string());
        }
        else if arg == "-h" || arg == "--help" {
            req.add_action("help".to_string());
        }
        else {
            req.add_action("unkown".to_string())
        }
    }
    return req;
}