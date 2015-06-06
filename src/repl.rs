use std::io;
use std::io::BufRead;

use embed_lang::typecheck::*;
use embed_lang::vm::{VM, run_expr, load_script};

macro_rules! tryf {
    ($e:expr) => (try!(($e).map_err(|e| format!("{}", e))))
}

fn print(vm: &VM) {
    println!("{:?}", vm.pop());
}

#[allow(dead_code)]
pub fn run() {
    let vm = VM::new();
    vm.extern_function("printInt", vec![INT_TYPE.clone()], UNIT_TYPE.clone(), Box::new(print))
        .unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match run_line(&vm, line) {
            Ok(continue_repl) => {
                if !continue_repl {
                    break
                }
            }
            Err(e) => println!("{}", e)
        }
    }
}

fn run_command(vm: &VM, command: char, args: &str) -> Result<bool, String> {
    match command {
        'q' => Ok(false),
        'l' => {
            try!(load_file(vm, args));
            Ok(true)
        }
        't' => {
            match vm.env().find_type_info(&vm.intern(args)) {
                Some(typ) => {
                    println!("type {} = {}", args, typ);
                }
                None => println!("{} is not a type", args)
            }
            Ok(true)
        }
        _ => Err("Invalid command ".to_string() + &*command.to_string())
    }
}

fn run_line(vm: &VM, line: io::Result<String>) -> Result<bool, String> {
    let expr_str = tryf!(line);
    match expr_str.chars().next().unwrap() {
        ':' => {
            run_command(vm, expr_str.chars().skip(1).next().unwrap(), expr_str[2..].trim())
        }
        _ =>  {
            let v = try!(run_expr(vm, &expr_str));
            println!("{:?}", v);
            Ok(true)
        }
    }
}

fn load_file(vm: &VM, filename: &str) -> Result<(), String> {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    let path = Path::new(filename);
    let mut file = tryf!(File::open(path));
    let mut buffer = String::new();
    tryf!(file.read_to_string(&mut buffer));
    let name = path.file_stem().and_then(|f| f.to_str()).expect("filename");
    load_script(vm, name, &buffer)
}

