/* Suggestions: 1. Write a function for every command
                2. Start with the pwd command
                3. Continue with the other commands that do not have parameters
*/
//fs::File, io
use std::{env, fs, path::Path, process};

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
    if args[2] == "-n" {
        newline = 3;
    }

    for i in newline..args.len() - 1 {
        print!("{} ", args[i]);
    }
    print!("{}", args[args.len() - 1]);

    if newline == 2 {
        print!("\n");
    }
}
fn cat(args: Vec<String>) {
    for i in 2..args.len() {
        match fs::read_to_string(args[i].clone()) {
            Ok(s) => print!("{}", s),
            Err(_err) => process::exit(-20),
        }
    }
}

fn mkdir(args: &[String]) -> i32 {
    //println!("mkdir name {}", args[0]);
    for i in 0..args.len() {
        match fs::create_dir(args[i].clone()) {
            Ok(_s) => (),
            Err(_e) => return -30,
        }
    }
    0
}

fn mv(args: Vec<String>) {
    if args.len() != 4 {
        panic!("INSUFICIENTE ARGUMENTE");
    }
    let source = args[2].clone();
    let dest = args[3].clone();
    if source == dest {
        return ();
    }
    match fs::rename(source, dest) {
        Ok(_) => (),
        Err(_e) => process::exit(-40),
    }
}
fn ln(args: Vec<String>) {
    if args.len() < 4 {
        panic!("INSUFICIENTE ARGUmente");
    }
    let mut idx = 2;
    if args[2] == "-s" || args[2] == "--symbolic" {
        idx = 3;
    }
    let oldpath = args[idx].clone();
    let newpath = args[idx + 1].clone();

    if idx == 2 {
        match fs::hard_link(oldpath, newpath) {
            Ok(_) => (),
            Err(_e) => process::exit(-50),
        }
    } else if idx == 3 {
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
    let mut rmdir_rec = false;
    let mut rmdir = false;
    let mut k = 2;
    while args[k] == "-r" || args[k] == "-R" || args[k] == "--recursive" {
        rmdir_rec = true;
        k += 1;
    }
    while args[k] == "-d" || args[k] == "--dir" {
        rmdir = true;
        k += 1;
    }
    let mut exit_code = 0;
    if rmdir == false && rmdir_rec == false {
        for i in 2..args.len() {
            match fs::remove_file(args[i].clone()) {
                Ok(_k) => (),
                Err(_e) => exit_code = -70,
            }
        }
    } else if rmdir == true && rmdir_rec == false {
        for i in k..args.len() {
            // let p = Path::new(&args[i]);
            // p.is_dir();
            match fs::metadata(args[i].clone()) {
                Ok(f) => {
                    let metadata = f;
                    if metadata.file_type().is_dir() == true {
                        match fs::remove_dir(args[i].clone()) {
                            Ok(_) => (),
                            Err(_e) => exit_code = -70,
                        }
                    } else {
                        match fs::remove_file(args[i].clone()) {
                            Ok(_k) => (),
                            Err(_e) => exit_code = -70,
                        }
                    }
                }
                Err(_e) => exit_code = -70,
            };
        }
    } else if rmdir_rec == true {
        for i in k..args.len() {
            match fs::metadata(args[i].clone()) {
                Ok(f) => {
                    let metadata = f;
                    if metadata.file_type().is_dir() == true {
                        match fs::remove_dir_all(args[i].clone()) {
                            Ok(_) => (),
                            Err(_e) => exit_code = -70,
                        }
                    } else {
                        match fs::remove_file(args[i].clone()) {
                            Ok(_k) => (),
                            Err(_e) => exit_code = -70,
                        }
                    }
                }
                Err(_e) => exit_code = -70,
            };
        }
    }
    process::exit(exit_code);
}
fn ls_elementar(dir_path:&String,all:bool) ->Vec<String>{
    
        let p = Path::new(dir_path);
        if p.is_dir() {
            match p.read_dir() {
                Ok(pa) => {
                    let paths = pa;
                    let mut vect: Vec<String> = Vec::new();
                    if all {
                        vect.push(String::from("."));
                        vect.push(String::from(".."));
                    }
                    for iter in paths {
                        if let Ok(iter) = iter {
                            let x = iter.file_name().to_string_lossy().into_owned();
                            if x.starts_with('.') && all==false {
                                
                            }
                            else {
                                vect.push(x);
                            }
                        }
                    }
                    vect.sort();
                    for elem in &vect {
                        println!("{}",elem);
                    }
                   vect
                }
                Err(_) => process::exit(-80),
    }
    } else if p.is_file() {
        println!("{}",dir_path);
        process::exit(0);
    }
    else { //if file doesn't exist
        process::exit(-80);
    }
    
}
fn ls_rec(dir_path:&String,all:bool) {
    let p = Path::new(dir_path);
    if p.is_dir(){
        println!("{}:",dir_path);
    }
    let mut vect = ls_elementar(&dir_path,all);
   // println!();
    for elem in &mut vect {
        if elem!="." && elem!=".." {

        
        let p2=Path::new(&elem);
        let p_dir=p.join(&p2);
        let dir_p=String::from(p_dir.to_string_lossy());
        if p_dir.is_dir() {
            ls_rec(&dir_p, all);
        }
    }
    }

}
fn ls(args: &[String]) {
    //ls primeste cel mult un director ca argument
    let mut k=0;
    let mut all=false;
    let mut rec = false;
    let length = args.len();
    let current_dir;
    let params:Vec<&str>=vec!["-a","--all","-R","--recursive"];
    while k<args.len() && params.contains(&args[k].as_str()){
        
        match args[k].as_str() {
            "-a" | "--all" => all=true,
            "-R" | "--recursive" => rec=true,
            _ => (),
        }
        k+=1;
    }


    if k==length {
        current_dir = String::from(".");
    }
    else {
        current_dir = args[k].clone();
    }
    if rec {
        ls_rec(&current_dir,all);
    } else {
        ls_elementar(&current_dir,all);
    }
}
fn cp_helper(source: &[String], dest: String, errno: &mut i32) {
    for i in 0..source.len() {
        //println!("FILE: {}", &source[i]);

        let p = Path::new(&source[i]);
        //aici se intra in director si se parcurg fisierele din el
        if p.is_dir() {
            match p.read_dir() {
                Ok(pa) => {
                    let paths = pa;
                    let mut vec: Vec<String> = Vec::new();
                    let mut j = 0;
                    for entry in paths {
                        if let Ok(entry) = entry {
                            let x = entry.file_name().to_string_lossy().into_owned();
                            vec.push(source[i].clone());
                            vec[j].push('/');
                            vec[j].push_str(&x);
                            j += 1;
                        }
                    }
                    let s_path = Path::new(&source[i]);
                    let d_path = Path::new(&dest);
                    let last_word = s_path.file_name().unwrap().to_str().unwrap();
                    let new_path;
                    if *errno != 777 {
                        new_path = d_path.join(last_word);
                    } else {
                        new_path = d_path.to_path_buf();
                    }
                    let dest3 = new_path.to_str().unwrap();
                    let dest2 = String::from(dest3);
                    // println!("Fixed Directory Path Is: {}", &dest2);
                    let vec_dest: Vec<String> = vec![dest2.clone()];
                    // println!("Errno flag renamedir copy {}", errno);
                    let err2 = mkdir(&vec_dest[0..]);
                    if err2 == -30 {
                        // println!("Failed mkdir!");
                        process::exit(-90);
                    }
                    // println!("Se apeleaza functia recursiva...! cu dest2: {}", dest2);

                    if *errno == 777 {
                        *errno = 0;
                    }

                    cp_helper(&vec[0..], dest2.clone(), errno);
                }
                Err(_e) => *errno = -90,
            }
        } else {
            //daca am mai multe argumente inseamna ca destinatia trebuie sa fie un director
            //verific daca dest e un director existent
            let p2 = Path::new(&dest);
            if p2.is_dir() {
                // let mut dest2=dest.clone();
                // dest2.push('/');
                // dest2.push_str(&source[i]);
                let s_path = Path::new(&source[i]);
                let d_path = Path::new(&dest);
                let last_word = s_path.file_name().unwrap().to_str().unwrap();
                let new_path = d_path.join(last_word);
                let dest2 = new_path.to_str().unwrap();
                // println!("Destination fixed path is: {}", dest2);
                match fs::copy(&source[i], &dest2) {
                    Ok(_k) => (),
                    Err(_) => {
                        *errno = -90;
                        // println!("Eroare la copiere file {} to directory {}!", source[i],dest2);
                    }
                }
            //daca am un singur argument destinatia va fi file.
            } else {
                match fs::copy(&source[0], &dest) {
                    Ok(_k) => (),
                    Err(_) => {
                        *errno = -90;
                        // println!("Eroare la copiere file {} to file {}", source[0], dest);
                    }
                }
            }
        }
    }
}
fn cp(args: Vec<String>) {
    if args.len() < 4 {
        panic!("INSUFICIENTE ARGUMENTE");
    }
    let mut errno = 0;
    let len = args.len();
    let mut k = 2;
    let mut rec = false;
    while args[k] == "-r" || args[k] == "-R" || args[k] == "--recursive" {
        rec = true;
        k += 1;
    }
    let source = &args[k..len - 1];
    let dest = args[len - 1].clone();
    // for i in 0..source.len() {
    //     println!("Sources: {} ", source[i]);
    // }
    // println!("Destination: {}", dest);
    if source.len() == 1 && source[0] == dest {
        return ();
    }

    let p = Path::new(&dest);
    if source.len() > 1 {
        if !p.is_dir() {
            //println!("File {} is not a directory!", &dest);
            process::exit(-90);
        }
    } else {
        let p2 = Path::new(&source[0]);
        if p2.is_dir() && p.exists() && !p.is_dir() {
            process::exit(-90);
        } else if p2.is_dir() && !p.exists() {
            errno = 777; // rename directory flag
        }
    }
    if rec {
        //cp -R -r --recursive
        cp_helper(source, dest, &mut errno);
    } else {
        //daca am mai multe argumente inseamna ca destinatia trebuie sa fie un director
        //verific daca dest e un director existent

        if p.is_dir() {
            for i in 0..source.len() {
                // let mut dest2=dest.clone();
                // dest2.push('/');
                let s_path = Path::new(&source[i]);
                let d_path = Path::new(&dest);
                let last_word = s_path.file_name().unwrap().to_str().unwrap();
                let new_path = d_path.join(last_word);
                let dest2 = new_path.to_str().unwrap();
                //dest2.push_str(&source[i]);
                //println!("Destination fixed path is: {}", dest2);
                match fs::copy(&source[i], &dest2) {
                    Ok(_k) => (),
                    Err(_) => {
                        errno = -90;
                        //println!("Eroare la copiere file to directory!");
                    }
                }
            }

        //daca am un singur argument destinatia va fi file.
        } else {
            match fs::copy(&source[0], &dest) {
                Ok(_k) => (),
                Err(_) => errno = -90,
            }
        }
    }
    //println!("Final errno is: {}", errno);
    process::exit(errno);
}
fn touch() {
    // fn touch(path: &Path) -> io::Result<()> {
    //     match OpenOptions::new().create(true).write(true).open(path) {
    //         Ok(_) => Ok(()),
    //         Err(e) => Err(e),
    //     }
    // } https://doc.rust-lang.org/stable/rust-by-example/std_misc/fs.html
}
fn chmod() {
    //let perm = args[2].parse::<i32>().unwrap();
}

fn main() {
    // TODO 1: Read the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please enter a command!");
        return ();
    }
    match args[1].as_str() {
        "pwd" => pwd(),
        "echo" => echo(args),
        "cat" => cat(args),
        "mkdir" => process::exit(mkdir(&args[2..])),
        "mv" => mv(args),
        "ln" => ln(args),
        "rmdir" => rmdir(args),
        "rm" => rm(args),
        "ls" => ls(&args[2..]),
        "cp" => cp(args),
        "touch" => touch(),
        "chmod" => chmod(),
        _ => {println!("Invalid command");
        process::exit(-1);}
    }
}
