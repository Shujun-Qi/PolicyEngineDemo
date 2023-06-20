use std::collections::HashMap;
use std::io::prelude::*;
// use std::process::exit;
use serde_json::{Result, Value, from_value};
use std::fs;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use regex::Regex;

#[derive(Debug, Default)]
struct Rules{
    speaker: String,
    condition: Vec<(String, String)>,
    relation: String,
    facts: Vec<String>
}




fn recursive_write(speaker: &str, id:&str,map: HashMap<String,Value>, file_path: &str, rules:&Vec<Rules>, flag:bool) -> Result<()>{
    
    // let file_path = "Example_Output/".to_owned()+sourcetype+"/"+sourcetype+".pl";
    let mut output_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    let mut output_string: String = "".to_string().to_owned();
    // println!("{:?}", rules);
    let sp_rules: Vec<&Rules> = rules.into_iter().filter(|&r| r.speaker == speaker.clone()).filter(|&a| (a.condition == [])||(map.get(&a.condition[0].0).unwrap().to_string().replace("\"", "") == a.condition[0].1)).collect();
    println!("{:?}", sp_rules);
    if sp_rules.len() == 0{
        return Ok(());
    }
    let rule = sp_rules[0];
    if rule.facts[0] == "$"{
        if !flag{
            for (key, value) in map{
                if value.is_object(){
                    // let hash: String = my_hash(value.to_string()).to_string();
                    output_string = output_string+&key+",";
                    let newmap: HashMap<String, Value> = from_value(value)?;
                    let _ = recursive_write(speaker, &key, newmap, file_path, rules, true);
                }
            }
        }
        else{
            for key in &rule.facts{
                if key == "$"{
                    continue;
                }
                let value = map.get(key).unwrap();
                if value.is_object(){
                    let hash: String = my_hash(value.to_string()).to_string();
                    output_string = output_string+key+",";
                    let newmap: HashMap<String, Value> = from_value(value.clone())?;
                    let _ = recursive_write(key, &hash, newmap, file_path, rules, false);
                }
                else if value.is_array(){
                    output_string = output_string+"[";
                    // println!("{},{},Array", speaker, hash);
                    let array : Vec<Value> = from_value(value.clone())?;
                    for item in array{
                        if item.is_object(){
                            let hash: String = my_hash(item.to_string()).to_string();
                            output_string = output_string+&hash+",";
                            let newmap: HashMap<String, Value> = from_value(item)?;
                            let _ = recursive_write(key, &hash, newmap, file_path, rules, false);
                        }
                        else{
                            output_string = output_string+&item.to_string()+",";
                        }
                    }
                    output_string.pop();
                    output_string = output_string+"],";
                }
                else {
                    output_string = output_string+&value.to_string()+",";
                }
            }
        }
    }
    else{
        for key in &rule.facts{
            println!("{}", key);
            let value = map.get(key).unwrap();
            if value.is_object(){
                let hash: String = my_hash(value.to_string()).to_string();
                output_string = output_string+&hash+",";
                let newmap: HashMap<String, Value> = from_value(value.clone())?;
                // println!("{:?}", newmap);
                let _ = recursive_write(key, &hash, newmap, file_path, rules, false);
            }
            else if value.is_array(){
                output_string = output_string+"[";
                // println!("{},{},Array", speaker, hash);
                let array : Vec<Value> = from_value(value.clone())?;
                for item in array{
                    if item.is_object(){
                        let hash: String = my_hash(item.to_string()).to_string();
                        output_string = output_string+&hash+",";
                        let newmap: HashMap<String, Value> = from_value(item)?;
                        let _ = recursive_write(key, &hash, newmap, file_path, rules, false);
                    }
                    else{
                        output_string = output_string.to_owned()+&item.to_string()+",";
                    }
                }
                output_string.pop();
                output_string = output_string+"],";
            }
            else {
                output_string = output_string+&value.to_string()+",";
            }
        }
    }
    output_string.pop();
    if let Err(e) = writeln!(output_file, "{}:{}({}).",id, rule.relation ,output_string) {
        eprintln!("Couldn't write to file: {}", e);
    }
    println!("The output is ({}).", {output_string});
    
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

fn parse_files(filename:&str, sourcetype: &str, types:&Vec<Rules>) -> Result<()>{
    let file_path = "Example_Output/".to_owned()+sourcetype+"/"+sourcetype+".pl";
    let output_name = file_path.clone();
    let mut output_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();
    if let Err(e) = writeln!(output_file, "\n#Facts from {}.", filename) {
        eprintln!("Couldn't write to file: {}", e);
    }

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let intoto_root: Value = serde_json::from_str(&contents)?;
    let speaker: String = my_hash(intoto_root.to_string()).to_string();
    
    if intoto_root.is_object(){
        let map: HashMap<String, Value> = from_value(intoto_root)?;
        let _ = recursive_write("root", &speaker, map, &output_name, types, false);
    }
    else{

    } 

    Ok(())
}

fn parse_rules(rule_file:&str) -> Vec<Rules>{
    let mut types: Vec<Rules> = Default::default();
    let rule_contents = fs::read_to_string(rule_file)
        .expect("Something went wrong reading the file");
    let binding = rule_contents.trim().replace("\n", "").replace("\r", "");
    let rules: Vec<&str> = binding.split(".").collect();
    for rule in rules{
        if rule == "" {
            continue;
        }
        let parts: Vec<&str> = rule.split(":").collect();
        let speaker = parts[0];
        let relation = parts[1];
        let token_re = Regex::new(r"\((.*?)\)").unwrap();
        let sp:Vec<&str> = speaker.split("(").collect();
        let rt:Vec<&str> = relation.split("(").collect();
        let mut new_rule = Rules{..Default::default()};
        new_rule.relation = rt[0].to_string();
        new_rule.speaker = sp[0].to_string();
        for cap in token_re.captures_iter(speaker){
            // println!("{}", &cap[1]);
            let conditions: Vec<&str> = cap[1].split(",").collect();
            for cond in conditions{
                let pair: Vec<&str> = cond.split("=").collect();
                new_rule.condition.push((pair[0].to_owned(),pair[1].to_owned()));
            }
        }
        for cap in token_re.captures_iter(relation){
            // println!("{}", &cap[1]);
            let facts: Vec<&str> = cap[1].split(",").collect();
            for fact in facts{
                new_rule.facts.push(fact.to_string());
            }
        }
        types.push(new_rule);
    }
    return types;
}

pub fn build_facts_library(dirname:&str, sourcetype: &str){
    
    // let re: Regex = Regex::new(r"\((.+)\)").unwrap();
    
    let rule_file = "Rules/".to_owned()+sourcetype+".rule";
    let rules = parse_rules(&rule_file);
    println!("{:?}", rules);
    let paths = fs::read_dir(dirname).unwrap();
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

    for path in paths {
        let _ =   parse_files(path.unwrap().path().as_os_str().to_str().unwrap(), sourcetype,  &rules);
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