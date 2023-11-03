/* Suggestions: 1. Write a function for every command
                2. Start with the pwd command
                3. Continue with the other commands that do not have parameters
*/
use std::{env, fs, process, fs::File, io,path::Path};
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
    
    for i in newline..args.len()-1 {
        print!("{} ", args[i]);
    }
    print!("{}", args[args.len()-1]);

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

fn mkdir(args: &[String]) {
    for i in 0..args.len() {
        match fs::create_dir(args[i].clone()) {
            Ok(_s) => (),
            Err(_e) => process::exit(-30),

        }
    }
}

fn mv(args: Vec<String>) {
    if args.len()!=4 {
        panic!("INSUFICIENTE ARGUMENTE");
    }
    let source = args[2].clone();
    let dest = args[3].clone();
    if source == dest {
        return ();
    }
    match fs::rename(source,dest) {
        Ok(_) => (),
        Err(_e) => process::exit(-40),
    }
}
fn ln(args: Vec<String>) {
    if args.len()<4 {
        panic!("INSUFICIENTE ARGUmente");
    }
    let mut idx = 2;
    if args[2]=="-s" || args[2]=="--symbolic" {
        idx=3;
    }
    let oldpath = args[idx].clone();
    let newpath = args[idx+1].clone();

    if idx==2 {
        match fs::hard_link(oldpath,newpath) {
            Ok(_) => (),
            Err(_e) => process::exit(-50),
        }
    } else if idx==3 {
        match std::os::unix::fs::symlink(oldpath, newpath) {
            Ok(_) => (),
            Err(_e) => process::exit(-50),
        }
    }

}
fn rmdir(args: Vec<String>) {
   
        for i in 2..args.len() {
        match fs::remove_dir(args[i].clone()) {
            Ok(_) => (),
            Err(_e) => process::exit(-60),
        }
    }
    

}
fn rm(args: Vec<String>) {
    let mut rmdir_rec=false;
    let mut rmdir=false;
    let mut k=2;
    while args[k]=="-r" ||  args[k]=="-R" || args[k]=="--recursive" {
        rmdir_rec=true;
        k+=1;
    }
    while args[k]=="-d" ||args[k]=="--dir" {
        rmdir=true;
        k+=1;
    }
    let mut exit_code = 0;
    if rmdir==false && rmdir_rec==false {
        
        for i in 2..args.len() {
            match fs::remove_file(args[i].clone()) {
                Ok(_k) => (),
                Err(_e) => exit_code=-70,
            }
        }
        
    }else if rmdir==true && rmdir_rec == false {
        
        for i in k..args.len() {
            // let p = Path::new(&args[i]);
            // p.is_dir();
            match fs::metadata(args[i].clone()) {
                Ok(f) => {
                    let metadata = f;
                    if metadata.file_type().is_dir() == true {
                    match fs::remove_dir(args[i].clone()) {
                        Ok(_) => (),
                        Err(_e) => exit_code=-70,
                    }
                } else { match fs::remove_file(args[i].clone()) {
                    Ok(_k) => (),
                    Err(_e) => exit_code=-70,
                    }
                }},
                Err(_e) => exit_code=-70,
            };
            
        }
        
    }
    else if rmdir_rec==true {
        
        for i in k..args.len() {
            match fs::metadata(args[i].clone()) {
                Ok(f) => {
                    let metadata = f;
                    if metadata.file_type().is_dir() == true {
                    match fs::remove_dir_all(args[i].clone()) {
                        Ok(_) => (),
                        Err(_e) => exit_code=-70,
                    }
                } else { match fs::remove_file(args[i].clone()) {
                    Ok(_k) => (),
                    Err(_e) => exit_code=-70,
                    }
                }},
                Err(_e) => exit_code=-70,
            };
            
        }
        
    }
    process::exit(exit_code);
}
fn ls() {

}
fn cp_helper(source: &[String], dest: &String, errno: &mut i32) {

    for i in 0..source.len() {

    
    let p = Path::new(&source[i]);
    if p.is_dir(){
        
        match p.read_dir() {
            Ok(pa) =>  {let paths = pa;
                let mut vec = Vec::new();
                for entry in paths {
                    if let Ok(entry) = entry {
                        let x = entry.file_name().to_string_lossy().into_owned();
                        vec.push(x);

                    }
                }
                let mut dest2=dest.clone();
                dest2.push('/');
                dest2.push_str(&source[i]);
                mkdir(&dest2);
                cp_helper(&vec,&dest2,errno)
            },
            Err(_e) => *errno=-90, 
        }
            
    } else {
    let mut file2;
    let mut file1;
    match File::create(dest) {
        Ok(file) => file2=file,
        Err(_e) => process::exit(-80),
    }
    match File::open(&source[i]) {
        Ok(file) => file1=file,
        Err(_e) => process::exit(-80),
    }
    match io::copy(&mut file1,&mut file2) {
        Ok(_k) =>(),
        Err(_e) => process::exit(-80),
    }
    }
}

}
fn cp(args: Vec<String>) {
    if args.len()!=4 {
        panic!("INSUFICIENTE ARGUMENTE");
    }
    let mut errno=0;
    let len = args.len();
    let source = &args[2..len-1];
    let dest = args[len-1].clone();
    if source.len()==1 && source[0] == dest {
        return ();
    }
    cp_helper(source,&dest,&mut errno);

}
fn touch() {

}
fn chmod(args: Vec<String>) {
    //let perm = args[2].parse::<i32>().unwrap();
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
        "mkdir" => mkdir(&args[2..]),
        "mv" => mv(args),
        "ln" => ln(args),
        "rmdir" => rmdir(args),
        "rm" => rm(args),
        "ls" => ls(),
        "cp" => cp(args),
        "touch"=> touch(),
        "chmod" => chmod(args),
        _ => println!("Invalid arguments!"),
    }
    
}
