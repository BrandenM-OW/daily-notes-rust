use std::env;
pub mod cmds;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("print out description of the program");
        return;
    }

    let command = &args[1];

    if command == "new" {
        let mut days = "0";
        if args.len() > 2 {
            days = &args[2];
        }

        cmds::new(&days);
    } else if command == "init" {
        cmds::init();
    } else {
        println!("Command not found");
    }
}
