/* Suggestions: 1. Write a function for every command
                2. Start with the pwd command
                3. Continue with the other commands that do not have parameters
*/
use std::{env, fs, process};
fn pwd() {

    // TODO 3: Implement the logic for pwd
    let path_result = env::current_dir();
    match path_result {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("{}", err),
    }    
}

fn echo(args: Vec<String>) {
    let mut newline = 2;
    if args[2]=="-n" {
        newline=3;
    }
    for i in newline..args.len() {
        print!("{} ", args[i]);
    }
    

    if newline==2{
        print!("\n");
    }

}
fn cat(args: Vec<String>) {
    for i in 2..args.len() {
    match fs::read_to_string(args[i].clone()) {
        Ok(s) => print!("{}",s),
        Err(_err) => process::exit(-20),
    }
}
}

fn mkdir(args: Vec<String>) {
    for i in 2..args.len() {
        match fs::create_dir(args[i].clone()) {
            Ok(_s) => (),
            Err(_e) => eprintln!("{}", -30),

        }
    }
}

fn mv() {

}
fn ln() {

}
fn rmdir() {

}
fn rm() {

}
fn ls() {

}
fn cp() {

}
fn touch() {

}
fn chmod() {

}

fn main() {

    // TODO 1: Read the command line arguments
    let args: Vec<String> = env::args().collect();
    // for i in 1..args.len() {
    //     println!("args[{}]={}",i,args[i]);
    // }
    // TODO 2: If the first argument is pwd, call pwd()
    // enum commands {
    //     Pwd,
    //     Echo,
    //     Cat,
    //     mkdir,
    //     mv,
    //     ln,
    //     rmdir,
    //     rm,
    //     ls,
    //     cp,
    //     touch,
    //     chmod,
    // };
    if args.len()==1 {
        println!("Please enter a command!");
        return ();
    }
    match args[1].as_str(){
        "pwd" => pwd(),
        "echo" => echo(args),
        "cat" => cat(args),
        "mkdir" => mkdir(args),
        "mv" => mv(),
        "ln" => ln(),
        "rmdir" => rmdir(),
        "rm" => rm(),
        "ls" => ls(),
        "cp" => cp(),
        "touch"=> touch(),
        "chmod" => chmod(),
        _ => println!("Invalid arguments!"),
    }
    
}
