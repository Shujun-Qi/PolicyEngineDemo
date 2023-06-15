use crate::types::intoto_types::{IntotoLayout, IntotoLink};
use std::collections::HashMap;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, from_value};
use struct_iterable::Iterable;
use std::process::Command;
use std::fs;
use std::any::type_name;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// fn parse_intoto_steps(dirname:&str, name: &str) -> Result<()> {
//     // Some JSON input data as a &str. Maybe this comes from the user.
//     let mut filename = dirname.to_owned();
//     filename = filename + name + ".link";
//     let contents = fs::read_to_string(filename)
//         .expect("Something went wrong reading the file");
//     // Parse the string of data into serde_json::Value.
//     let intoto_link: IntotoLink = serde_json::from_str(&contents)?;
//     let stepname = &intoto_link.signed.name;
//     let keyid = &intoto_link.signatures[0].keyid;
//     let mut file = fs::OpenOptions::new()
//         .write(true)
//         .append(true)
//         .open("IntotoCheck.pl")
//         .unwrap();
//     if let Err(e) = writeln!(file, "    intoto:validate('{}','{}'),", keyid, stepname) {
//         eprintln!("Couldn't write to file: {}", e);
//     }

//     Ok(())
// }

// fn parse_intoto(dirname:&str, content:&str) -> Result<()> {
//     // Some JSON input data as a &str. Maybe this comes from the user.
//     // Parse the string of data into serde_json::Value.
//     // let v: Value = serde_json::from_str(content)?;
//     let intoto_meta: IntotoLayout = serde_json::from_str(content)?;
//     if let Err(_) = fs::remove_file("IntotoCheck.pl"){
//         println!("No file to remove");
//     }
//     let mut checkfile = fs::OpenOptions::new()
//         .create_new(true)
//         .write(true)
//         .append(true)
//         .open("IntotoCheck.pl")
//         .unwrap();
//     if let Err(e) = writeln!(checkfile, ":- consult('IntotoAsserts.pl').") {
//         eprintln!("Couldn't write to file: {}", e);
//     }
    
//     if let Err(e) = writeln!(checkfile, "main:-") {
//         eprintln!("Couldn't write to file: {}", e);
//     }
//     if let Err(_) = fs::remove_file("IntotoAsserts.pl"){
//         println!("No file to remove");
//     }
//     let mut assertfile = fs::OpenOptions::new()
//         .create_new(true)
//         .write(true)
//         .append(true)
//         .open("IntotoAsserts.pl")
//         .unwrap();
//     if let Err(e) = writeln!(assertfile, ":- consult('Policy/intoto.pl').") {
//         eprintln!("Couldn't write to file: {}", e);
//     }
//     if let Err(e) = writeln!(assertfile, "intoto:root('{}').", intoto_meta.signatures[0].keyid) {
//         eprintln!("Couldn't write to file: {}", e);
//     }

//     // Access parts of the data by indexing with square brackets.
//     let mut stepindex = Some(0);
//     while let Some(x) = stepindex {
//         if x < intoto_meta.signed.steps.len() {
//             let stepname = &intoto_meta.signed.steps[x].name;
//             stepindex = Some(x+1);
//             if let Err(e) = writeln!(assertfile, "intoto:delegate('{}','{}','{}').", intoto_meta.signatures[0].keyid, intoto_meta.signed.steps[x].pubkeys[0], stepname) {
//                 eprintln!("Couldn't write to file: {}", e);
//             }

//             let res = parse_intoto_steps(dirname, stepname);
//             println!("Result: {:?}", res);
//         } else {
//             break;
//         }
//     }
//     let mut checkfile = fs::OpenOptions::new()
//         .write(true)
//         .append(true)
//         .open("IntotoCheck.pl")
//         .unwrap();
//     if let Err(e) = writeln!(checkfile, "    nl.") {
//         eprintln!("Couldn't write to file: {}", e);
//     }
//     // if let Err(e) = writeln!(checkfile, "main.") {
//     //     eprintln!("Couldn't write to file: {}", e);
//     // }
    

//     Ok(())
// }

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

// pub fn handle_intoto_layout(dirname:&str) -> Result<()>{
//     let filename = dirname.to_owned() + "root.layout";
//     let contents = fs::read_to_string(filename)
//         .expect("Something went wrong reading the file");

//     let intoto_root: IntotoLayout = serde_json::from_str(&contents)?;

//     let principal = &intoto_root.signatures[0].keyid;

//     // Generating example output
    // if let Err(_) = fs::remove_file("IntotoRoot.pl"){
    //     println!("No file to remove");
    // }
//     let mut rootfile = fs::OpenOptions::new()
//         .create_new(true)
//         .write(true)
//         .append(true)
//         .open("IntotoRoot.pl")
//         .unwrap();
//     if let Err(e) = writeln!(rootfile, ":- consult('Policy/intoto.pl').") {
//         eprintln!("Couldn't write to file: {}", e);
//     }
//     if let Err(e) = writeln!(rootfile, "intoto:root('{}').", principal) {
//         eprintln!("Couldn't write to file: {}", e);
//     }
//     if let Err(e) = writeln!(rootfile, "{}", intoto_root) {
//         eprintln!("Couldn't write to file: {}", e);
//     }
    

//     Ok(())
// }

pub fn recursive_write(speaker: &str, facts: Value, types:HashMap<String, String>) -> Result<()>{
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("Example_Output/Intoto/IntotoRoot.pl")
        .unwrap();
    
    if facts.is_string(){
        // println!("{},{},String", speaker, facts);
        if let Err(e) = writeln!(file, "intoto:signs('{}','{}'),", speaker, facts) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    else if facts.is_object(){
        let hash: String = my_hash(facts.to_string()).to_string();
        // println!("{},{},Object", speaker, hash);
        if let Err(e) = writeln!(file, "intoto:signs('{}','{}'),", speaker, hash) {
            eprintln!("Couldn't write to file: {}", e);
        }
        let newmap: HashMap<String, Value> = from_value(facts)?;
        for (key, value) in newmap{
            let _ = recursive_write(&key, value, types.clone());
        }
        
    }
    else if facts.is_array(){
        let hash: String = my_hash(facts.to_string()).to_string();
        // println!("{},{},Array", speaker, hash);
        if let Err(e) = writeln!(file, "intoto:signs('{}','{}'),", speaker, hash) {
            eprintln!("Couldn't write to file: {}", e);
        }
        let array : Vec<Value> = from_value(facts)?;
        for item in array{
            if item.is_object(){
                let _ = recursive_write(speaker, item, types.clone());
            }
        }
    }
    else{
        // println!("{},{},Int", speaker, facts);
        if let Err(e) = writeln!(file, "intoto:signs('{}','{}'),", speaker, facts) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    Ok(())
}

fn my_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

pub fn handle_intoto_layout(filename:&str, types:HashMap<String, String>) -> Result<()>{
    println!("{}", filename);
    if let Err(_) = fs::remove_file("Example_Output/Intoto/IntotoRoot.pl"){
        println!("No file to remove");
    }
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let intoto_root: Value = serde_json::from_str(&contents)?;
    let speaker: String = my_hash(intoto_root.to_string()).to_string();

    
    let _ = recursive_write(&speaker, intoto_root, types);

    Ok(())
}

pub fn enforce_intoto(dirname:&str){
    
    let mut types:HashMap<String, String> = Default::default();
    let rule_file = "Rules/Intoto.rule";
    let rule_contents = fs::read_to_string(rule_file)
        .expect("Something went wrong reading the file");
    let rules = rule_contents.split("\n");
    for rule in rules{
        let str = rule.split(":").collect::<Vec<_>>();
        println!("{:?}",str);
        if str.len()>1{
            types.insert(str[0].to_string(), str[1].to_string());
        }
    }

    let paths = fs::read_dir(dirname).unwrap();

    for path in paths {
        let _ =   handle_intoto_layout(path.unwrap().path().as_os_str().to_str().unwrap(), types.clone());
    }
    

    // println!("Result: {:?}", result);
    // let output = Command::new("swipl")
    //     .arg("-f")
    //     .arg("IntotoCheck.pl")
    //     .arg("-g")
    //     .arg("main")
    //     .arg("-t")
    //     .arg("halt")
    //     .output()
    //     .expect("check failed to start");

    // if output.status.success() {
    //     println!("Intoto check passed");
    // } else {
    //     println!("Intoto check failed");
    // }
}