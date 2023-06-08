use std::{io::prelude::*, collections::HashMap};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::process::Command;
use std::fs;
use sha256::{digest, try_digest};
use ring::signature;



#[derive(Serialize, Deserialize, Debug)]
struct TufSignatures{
    keyid: String,
    sig: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TufLayout{
    signatures: Vec<TufSignatures>,
    signed: Value,
}


fn handle_tuf_target(dirname:&str) -> Result<()>{
    
    let targetfile = dirname.to_owned() + "targets.json";
    let target_contents = fs::read_to_string(targetfile)
    .expect("Something went wrong reading the snapshot file");
    let tuf_target: TufLayout = serde_json::from_str(&target_contents)?;

    if let Err(_) = fs::remove_file("TufTarget.pl"){
        println!("No file to remove");
    }
    let mut target_pl_file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open("TufTarget.pl")
        .unwrap();
    if let Err(e) = writeln!(target_pl_file, ":- consult('TufSnapshot.pl').") {
        eprintln!("Couldn't write to file: {}", e);
    }
    for roles in tuf_target.signed["delegations"]["roles"].as_array().unwrap(){
        if let Err(e) = writeln!(target_pl_file, "tuf:target_delegation('{}', '{}', '{}', '{}').",tuf_target.signatures[0].keyid, roles["keyids"][0], roles["name"] ,roles["paths"] ) {
            eprintln!("Couldn't write to file: {}", e);
        }
        let filename = roles["name"].to_string().replace("\"","").as_str().to_owned() + ".json";
        let rolefile = dirname.to_owned() + filename.as_str();
        println!("{}", rolefile);
        let role_contents = fs::read_to_string(rolefile)
        .expect("Something went wrong reading the snapshot file");
        let tuf_role: TufLayout = serde_json::from_str(&role_contents)?;
        let role_map =  tuf_role.signed["targets"].as_object().unwrap();
        let mut checkfile = fs::OpenOptions::new()
        // .create_new(true)
        .write(true)
        .append(true)
        .open("TufCheck.pl")
        .unwrap();
        for (key, _) in role_map.iter(){
            if let Err(e) = writeln!(checkfile, "    tuf:check_target('{}','{}'),", tuf_role.signatures[0].keyid, key) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
        if let Err(e) = writeln!(checkfile, "    tuf:validate('{}', '{}', '{}', '{}').", "timestamp", "", tuf_role.signed["version"], filename) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    let map =  tuf_target.signed["targets"].as_object().unwrap();
    for (key, _) in map.iter(){
        if let Err(e) = writeln!(target_pl_file, "tuf:target_delegation('{}', '{}', '{}', '{}').", tuf_target.signatures[0].keyid, tuf_target.signatures[0].keyid, "target", key) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    let mut checkfile = fs::OpenOptions::new()
        // .create_new(true)
        .write(true)
        .append(true)
        .open("TufCheck.pl")
        .unwrap();
    if let Err(e) = writeln!(checkfile, "    tuf:match_key('{}','{}'),", "target", tuf_target.signatures[0].keyid) {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(e) = writeln!(checkfile, "    tuf:validate('{}', '{}', '{}', '{}').", "timestamp", "", tuf_target.signed["version"], "target.json") {
        eprintln!("Couldn't write to file: {}", e);
    }
    
    Ok(())
}



fn handle_tuf_snapshot(dirname:&str) -> Result<()>{
    let snapshotfile = dirname.to_owned() + "snapshot.json";
    let snapshot_contents = fs::read_to_string(snapshotfile)
        .expect("Something went wrong reading the snapshot file");
    let tuf_snapshot: TufLayout = serde_json::from_str(&snapshot_contents)?;
    let snapshot_hash = digest(snapshot_contents);

    if let Err(_) = fs::remove_file("TufSnapshot.pl"){
        println!("No file to remove");
    }
    let mut snapshot_pl_file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open("TufSnapshot.pl")
        .unwrap();
    if let Err(e) = writeln!(snapshot_pl_file, ":- consult('TufTimeStamp.pl').") {
        eprintln!("Couldn't write to file: {}", e);
    }

    let map =  tuf_snapshot.signed["meta"].as_object().unwrap();
    for (key, value) in map.iter(){
        if let Err(e) = writeln!(snapshot_pl_file, "tuf:sign('{}', '{}', '{}', '{}').", tuf_snapshot.signatures[0].keyid, "", value["version"], key) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    let mut checkfile = fs::OpenOptions::new()
        // .create_new(true)
        .write(true)
        .append(true)
        .open("TufCheck.pl")
        .unwrap();
    if let Err(e) = writeln!(checkfile, "    tuf:match_key('{}','{}'),", "snapshot", tuf_snapshot.signatures[0].keyid) {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(e) = writeln!(checkfile, "    tuf:validate('{}', '{}', '{}', '{}').", "timestamp", snapshot_hash, tuf_snapshot.signed["version"], "snapshot.json") {
        eprintln!("Couldn't write to file: {}", e);
    }


    Ok(())
}

fn handle_tuf_timestamp(dirname:&str) -> Result<()>{
    let timefile = dirname.to_owned() + "timestamp.json";
    let time_contents = fs::read_to_string(timefile)
        .expect("Something went wrong reading the time file");
    let tuf_time: TufLayout = serde_json::from_str(&time_contents)?;
    let time_hash= tuf_time.signed["meta"]["snapshot.json"]["hashes"]["sha256"].to_string().replace("\"", "");

    if let Err(_) = fs::remove_file("TufTimeStamp.pl"){
        println!("No file to remove");
    }
    let mut time_pl_file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open("TufTimeStamp.pl")
        .unwrap();
    if let Err(e) = writeln!(time_pl_file, ":- consult('TufRoot.pl').") {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(e) = writeln!(time_pl_file, "tuf:sign('{}', '{}', '{}', '{}').", tuf_time.signatures[0].keyid, time_hash, tuf_time.signed["meta"]["snapshot.json"]["version"], "snapshot.json") {
        eprintln!("Couldn't write to file: {}", e);
    }

    let mut checkfile = fs::OpenOptions::new()
        // .create_new(true)
        .write(true)
        .append(true)
        .open("TufCheck.pl")
        .unwrap();
    if let Err(e) = writeln!(checkfile, "    tuf:match_key('{}','{}'),", "timestamp", tuf_time.signatures[0].keyid) {
        eprintln!("Couldn't write to file: {}", e);
    }
    
    

    Ok(())
}

fn handle_tuf_root(dirname:&str) -> Result<()> {
    let rootfile = dirname.to_owned() + "root.json";
    let root_contents = fs::read_to_string(rootfile)
        .expect("Something went wrong reading the root file");
    let tuf_root: TufLayout = serde_json::from_str(&root_contents)?;
    let sig = tuf_root.signatures[0].sig.to_string().replace("\"", "");
    let keyid = tuf_root.signatures[0].keyid.to_string().replace("\"", "");
    // let public_key = tuf_root.signed["keys"][keyid]["keyval"]["public"].to_string().replace("\"", "");
    let tuf_roles = &tuf_root.signed["roles"];


    if let Err(_) = fs::remove_file("TufRoot.pl"){
        println!("No file to remove");
    }
    let mut root_pl_file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open("TufRoot.pl")
        .unwrap();
    if let Err(e) = writeln!(root_pl_file, ":- consult('Policy/tuf.pl').") {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(e) = writeln!(root_pl_file, "tuf:root('{}').", keyid) {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(e) = writeln!(root_pl_file, "tuf:delegate('{}','{}','{}').", keyid, "root", tuf_roles["root"]["keyids"][0].to_string().replace("\"","")) {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(e) = writeln!(root_pl_file, "tuf:delegate('{}','{}','{}').", keyid, "snapshot", tuf_roles["snapshot"]["keyids"][0].to_string().replace("\"","")) {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(e) = writeln!(root_pl_file, "tuf:delegate('{}','{}','{}').", keyid, "targets", tuf_roles["targets"]["keyids"][0].to_string().replace("\"","")) {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(e) = writeln!(root_pl_file, "tuf:delegate('{}','{}','{}').", keyid, "timestamp", tuf_roles["timestamp"]["keyids"][0].to_string().replace("\"","")) {
        eprintln!("Couldn't write to file: {}", e);
    }

    if let Err(_) = fs::remove_file("TufCheck.pl"){
        println!("No file to remove");
    }
    let mut checkfile = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open("TufCheck.pl")
        .unwrap();
    if let Err(e) = writeln!(checkfile, ":- consult('TufTarget.pl').") {
        eprintln!("Couldn't write to file: {}", e);
    }
    
    if let Err(e) = writeln!(checkfile, "main:-") {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}

pub fn enforce_tuf(dirname:&str){
    
    let _ = handle_tuf_root(dirname);
    let _ = handle_tuf_timestamp(dirname);
    let _ = handle_tuf_snapshot(dirname);
    let _ = handle_tuf_target(dirname);

    let output = Command::new("swipl")
        .arg("-f")
        .arg("TufCheck.pl")
        .arg("-g")
        .arg("main")
        .arg("-t")
        .arg("halt")
        .output()
        .expect("check failed to start");

    if output.status.success() {
        println!("Tuf check passed");
    } else {
        println!("Tuf check failed");
    }
}