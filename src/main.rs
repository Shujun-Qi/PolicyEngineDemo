use std::env;
use std::fs;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct IntotoSignatures{
    keyid: String,
    method: String,
    sig: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IntotoSteps{
    _type: String,
    name: String,
    expected_command: Vec<String>,
    pubkeys: Vec<String>,
    threshold: u32,
    expected_materials: Vec<Vec<String>>,
    expected_products: Vec<Vec<String>>,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct IntotoKeys{
//     keyid: String,
//     keytype: String,
//     keyval: Value,
// }

#[derive(Serialize, Deserialize, Debug)]
struct IntotoLayoutSigned{
    _type: String,
    readme: String,
    expires: String,
    steps: Vec<IntotoSteps>,
    inspect: Vec<String>,
    keys: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct IntotoLinkSigned{
    _type: String,
    name: String,
    command: Vec<String>,
    materials: Value,
    products: Value,
    byproducts: Value,
    environment: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct IntotoLayout {
    signed: IntotoLayoutSigned,
    signatures: Vec<IntotoSignatures>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IntotoLink {
    signed: IntotoLinkSigned,
    signatures: Vec<IntotoSignatures>,
}

fn parse_intoto_steps(dirname:&str, name: &str) -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let mut filename = dirname.to_owned();
    filename = filename + name + ".link";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    // Parse the string of data into serde_json::Value.
    let intoto_link: IntotoLink = serde_json::from_str(&contents)?;
    let stepname = &intoto_link.signed.name;
    let keyid = &intoto_link.signatures[0].keyid;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("IntotoCheck.pl")
        .unwrap();
    if let Err(e) = writeln!(file, "    intoto:validate('{}','{}'),", keyid, stepname) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}

fn parse_intoto(dirname:&str, content:&str) -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    // Parse the string of data into serde_json::Value.
    // let v: Value = serde_json::from_str(content)?;
    let intoto_meta: IntotoLayout = serde_json::from_str(content)?;
    if let Err(_) = fs::remove_file("IntotoCheck.pl"){
        println!("No file to remove");
    }
    let mut checkfile = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open("IntotoCheck.pl")
        .unwrap();
    if let Err(e) = writeln!(checkfile, ":- consult('IntotoAsserts.pl').") {
        eprintln!("Couldn't write to file: {}", e);
    }
    
    if let Err(e) = writeln!(checkfile, "main:-") {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(_) = fs::remove_file("IntotoAsserts.pl"){
        println!("No file to remove");
    }
    let mut assertfile = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open("IntotoAsserts.pl")
        .unwrap();
    if let Err(e) = writeln!(assertfile, ":- consult('Policy/intoto.pl').") {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(e) = writeln!(assertfile, "intoto:root('{}').", intoto_meta.signatures[0].keyid) {
        eprintln!("Couldn't write to file: {}", e);
    }

    // Access parts of the data by indexing with square brackets.
    let mut stepindex = Some(0);
    while let Some(x) = stepindex {
        if x < intoto_meta.signed.steps.len() {
            let stepname = &intoto_meta.signed.steps[x].name;
            stepindex = Some(x+1);
            if let Err(e) = writeln!(assertfile, "intoto:delegate('{}','{}','{}').", intoto_meta.signatures[0].keyid, intoto_meta.signed.steps[x].pubkeys[0], stepname) {
                eprintln!("Couldn't write to file: {}", e);
            }

            let res = parse_intoto_steps(dirname, stepname);
            println!("Result: {:?}", res);
        } else {
            break;
        }
    }
    let mut checkfile = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("IntotoCheck.pl")
        .unwrap();
    if let Err(e) = writeln!(checkfile, "    nl.") {
        eprintln!("Couldn't write to file: {}", e);
    }
    // if let Err(e) = writeln!(checkfile, "main.") {
    //     eprintln!("Couldn't write to file: {}", e);
    // }
    

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dirname = &args[1];
    println!("Reading file from {}", dirname);
    let filename = dirname.to_owned() + "root.layout";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);
    let result =   parse_intoto(dirname, &contents);

    println!("Result: {:?}", result);
    let output = Command::new("swipl")
        .arg("-f")
        .arg("IntotoCheck.pl")
        .arg("-g")
        .arg("main")
        .arg("-t")
        .arg("halt")
        .output()
        .expect("check failed to start");

    if output.status.success() {
        println!("Intoto check passed");
    } else {
        println!("Intoto check failed");
    }
}