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


pub fn recursive_write(speaker: &str, facts: Value, sourcetype: &str, types:HashMap<String, String>) -> Result<()>{
    
    let file_path = "Example_Output/".to_owned()+sourcetype+"/"+sourcetype+".pl";
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();
    
    if facts.is_string(){
        // println!("{},{},String", speaker, facts);
        if let Err(e) = writeln!(file, "{}:signs('{}','{}'),",sourcetype, speaker, facts) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    else if facts.is_object(){
        let hash: String = my_hash(facts.to_string()).to_string();
        // println!("{},{},Object", speaker, hash);
        if let Err(e) = writeln!(file, "{}:signs('{}','{}'),",sourcetype, speaker, hash) {
            eprintln!("Couldn't write to file: {}", e);
        }
        let newmap: HashMap<String, Value> = from_value(facts)?;
        for (key, value) in newmap{
            let _ = recursive_write(&key, value, sourcetype, types.clone());
        }
        
    }
    else if facts.is_array(){
        let hash: String = my_hash(facts.to_string()).to_string();
        // println!("{},{},Array", speaker, hash);
        if let Err(e) = writeln!(file, "{}:signs('{}','{}'),",sourcetype, speaker, hash) {
            eprintln!("Couldn't write to file: {}", e);
        }
        let array : Vec<Value> = from_value(facts)?;
        for item in array{
            if item.is_object(){
                let _ = recursive_write(speaker, item, sourcetype, types.clone());
            }
        }
    }
    else{
        // println!("{},{},Int", speaker, facts);
        if let Err(e) = writeln!(file, "{}:signs('{}','{}'),",sourcetype, speaker, facts) {
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

pub fn parse_files(filename:&str, sourcetype: &str, types:HashMap<String, String>) -> Result<()>{
    println!("{}", filename);
    let file_path = "Example_Output/".to_owned()+sourcetype+"/"+sourcetype+".pl";
    let file_name = file_path.clone();
    if let Err(_) = fs::remove_file(file_name){
        println!("No file to remove");
    }
    let mut output_file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();
    let policy_file = "Policy/".to_owned()+sourcetype+".pl";
    if let Err(e) = writeln!(output_file, ":-consult({}).", policy_file) {
        eprintln!("Couldn't write to file: {}", e);
    }


    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let intoto_root: Value = serde_json::from_str(&contents)?;
    let speaker: String = my_hash(intoto_root.to_string()).to_string();

    
    let _ = recursive_write(&speaker, intoto_root, sourcetype, types);

    Ok(())
}

pub fn build_facts_library(dirname:&str, sourcetype: &str){
    
    let mut types:HashMap<String, String> = Default::default();
    let rule_file = "Rules/".to_owned()+sourcetype+".rule";
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
        let _ =   parse_files(path.unwrap().path().as_os_str().to_str().unwrap(),sourcetype,  types.clone());
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