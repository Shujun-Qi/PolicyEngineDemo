use std::fmt::write;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::process::Command;
use std::fs;
use std::collections::HashMap;
use struct_iterable::Iterable;

#[derive(Serialize, Deserialize, Debug)]
pub struct Steps (Vec<IntotoSteps>);

impl std::fmt::Display for Steps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = write!(f, "[");
        for item in &self.0{
            let _ = write!(f, "'{:p}',", item);
        }
        write!(f, "]")
    }
}


#[derive(Serialize, Deserialize, Debug, Iterable)]
pub struct IntotoSignatures{
    pub keyid: String,
    pub method: String,
    pub sig: String,
}

impl std::fmt::Display for IntotoSignatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "intoto:signatures('{:p}','{}','{}','{}').", &self, self.keyid, self.method, self.sig)
    }
}

#[derive(Serialize, Deserialize, Debug, Iterable)]
pub struct IntotoSteps{
    pub _type: String,
    pub name: String,
    pub expected_command: Vec<String>,
    pub pubkeys: Vec<String>,
    pub threshold: u32,
    pub expected_materials: Vec<Vec<String>>,
    pub expected_products: Vec<Vec<String>>,
}

impl std::fmt::Display for IntotoSteps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "intoto:steps('{:p}','{}','{}','{:?}','{:?}','{}','{:?}','{:?}').", &self, self._type, self.name, self.expected_command, self.pubkeys, self.threshold, self.expected_materials, self.expected_products)
    }
}



#[derive(Serialize, Deserialize, Debug, Iterable)]
pub struct IntotoKeysVal{
    private: String,
    public: String,
}

impl std::fmt::Display for IntotoKeysVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "intoto:keysval('{:p}','{}','{}').", &self, self.private, self.public)
    }
}

#[derive(Serialize, Deserialize, Debug, Iterable)]
pub struct IntotoKeys{
    keyid: String,
    keytype: String,
    keyval: IntotoKeysVal,
}

impl std::fmt::Display for IntotoKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = writeln!(f, "intoto:keys('{:p}','{}','{}','{:p}).", &self, self.keyid, self.keytype, &self.keyval);
        write!(f, "{:?}", self.keyval)
    }
}

#[derive(Serialize, Deserialize, Debug, Iterable)]
pub struct IntotoLayoutSigned{
    pub _type: String,
    pub readme: String,
    pub expires: String,
    pub steps: Steps,
    pub inspect: Vec<String>,
    pub keys: HashMap<String, IntotoKeys>,
}

impl std::fmt::Display for IntotoLayoutSigned {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = writeln!(f, "intoto:layoutSigned('{:p}','{}','{}','{}','{}','{:?}','{:p}').", &self, self._type, self.readme, self.expires, self.steps, self.inspect, &self.keys);
        for item in self.steps.0.iter(){
            let _ = writeln!(f, "{}", item);
        }
        write!(f, "{:?}", self.keys)
    }
}


#[derive(Serialize, Deserialize, Debug, Iterable)]
pub struct IntotoLinkByproducts{
    return_value: u32,
    stdout: String,
    stderr: String,
}

impl std::fmt::Display for IntotoLinkByproducts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "intoto:byproducts('{:p}','{}','{}','{}).", &self, self.return_value, self.stdout, self.stderr)
    }
}

#[derive(Serialize, Deserialize, Debug, Iterable)]
pub struct IntotoLinkSigned{
    pub _type: String,
    pub name: String,
    pub command: Vec<String>,
    pub materials: HashMap<String, Value>,
    pub products: HashMap<String, Value>,
    pub byproducts: IntotoLinkByproducts,
    pub environment: Value,
}

impl std::fmt::Display for IntotoLinkSigned {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        let _ = writeln!(f, "intoto:linkSigned('{:p}','{}','{}','{:?}','{:?}','{:?}','{:p}','{:?}').", &self, self._type, self.name, self.command, self.materials, self.products, &self.byproducts, self.environment);
        write!(f, "{}", self.byproducts)
    }
}

#[derive(Serialize, Deserialize, Debug, Iterable)]
pub struct IntotoLayout {
    pub signed: IntotoLayoutSigned,
    pub signatures: Vec<IntotoSignatures>,
}

impl std::fmt::Display for IntotoLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = writeln!(f, "intoto:layout('{:p}','{:p}','{:p}').", &self, &self.signed, &self.signatures[0]);
        let _ = writeln!(f, "{}", self.signed);
        write!(f, "{}", self.signatures[0])
    }
}

#[derive(Serialize, Deserialize, Debug, Iterable)]
pub struct IntotoLink {
    pub signed: IntotoLinkSigned,
    pub signatures: Vec<IntotoSignatures>,
}

impl std::fmt::Display for IntotoLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = writeln!(f, "intoto:link('{:p}','{:p}','{:p}').", &self, &self.signed, &self.signatures[0]);
        let _ = writeln!(f, "{}", self.signed);
        write!(f, "{}", self.signatures[0])
    }
}