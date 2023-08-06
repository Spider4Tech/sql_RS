mod scanner;
mod systemtrayerror;
mod xml_generator;

use crate::scanner::scanner;
use crate::systemtrayerror::SystemTrayError;
use crate::xml_generator::XMLElement;

use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::{Arc, Mutex};

// Changing the name of function : red, masterball, e2pz , mrpropre is totally forbidden, if you do it you can wreck space time

// #[derive(Debug)]
// enum TableRelationType {
//     OneToOne,
//     OneToMany,
//     ManyToOne,
//     ManyToMany,
// }

// #[derive(Debug)]
// struct TableRelation {
//     from_table: String,
//     from_column: String,
//     to_table: String,
//     to_column: String,
//     relation_type: TableRelationType,
// }

#[derive(Debug)]
struct SqlTable<'a> {
    name: &'a str,
    column: Vec<TableColumn<'a>>,
    // relation: Vec<TableRelation>,
}



#[derive(Debug)]
struct TableColumn<'a> {
    name: &'a str,
    data_type: &'a str,
    is_primary_key: bool,
    is_foreign_key: bool,
    references_table: Option<&'a str>,
    references_column: Option<&'a str>,
}

/// Get name  of the table
fn mrpropre<'a>(input: &'a Vec<&'a str>, regex: &Regex) -> Vec<&'a str> {
    let first = input.first().unwrap();
    let mut result = input.to_owned();
    result.retain(|&str| regex.is_match(str));
    result.insert(0, first);
    result
        .par_iter()
        .map(|str| {
            let start_index = str.find('`').unwrap_or(0) + 1;
            let end_index = str.rfind('`').unwrap_or(str.len());
            &str[start_index..end_index]
        })
        .collect::<Vec<&str>>()
}

/// ## Masterball is lauched by Red
///
/// Get table name and column name and type to insert as key name table and as value vector of tuple of column name and type in a hashmap
fn masterball<'a>(input: &'a Vec<&'a str>, regex: &Regex, reg_table_column: &Regex, hashmap: &Mutex<HashMap<&'a str, Vec<(&'a str, &'a str)>>>) {
    let table = match reg_table_column.captures(input.first().unwrap()) {
        Some(captures) => captures.get(1).map(|m| m.as_str()).unwrap_or(""),
        None => return,
    };

    let mut result = input.to_owned();
    result.retain(|&str| regex.is_match(str));

    let mut vec: Vec<(&str, &str)> = Vec::new();

    for str in result.iter() {
        if let Some(captures) = reg_table_column.captures(str) {
            let mut column_name = "";
            let mut column_type = "";

            for (i, capture) in captures.iter().skip(1).enumerate() {
                match i {
                    0 => column_name = capture.unwrap().as_str(),
                    1 => column_type = capture.unwrap().as_str(),
                    _ => break,
                }
            }

            vec.push((column_name, column_type));
        }
    }

    let mut hashmap = hashmap.lock().unwrap();
    hashmap.insert(table, vec);
}

/// Iter on a vector to get table name and column info to return hashmap
fn red<'a>(input: &'a Vec<Vec<&'a str>>) -> HashMap<&'a str, Vec<(&'a str, &'a str)>> {
    let regex = Regex::new(r"^\s*`").unwrap();
    let reg_table_column = Regex::new(r"`([^`]+)`\s+([^,\s]+)").unwrap();

    let hashmap = HashMap::new();
    let hashmap = Arc::new(Mutex::new(hashmap));
    input.par_iter().for_each(|vec| masterball(vec, &regex, &reg_table_column, &hashmap));

    let hashmap = Arc::try_unwrap(hashmap).unwrap();
    
    hashmap.into_inner().unwrap()
}

/// clean a vector of str by extracting a element inside a pattern
fn mrclean<'a>(input: &'a Vec<Vec<&'a str>>, patern: &'a str) -> Vec<Vec<&'a str>> {
    let reg = format!(r"^\s*{patern}");
    let regex = Regex::new(&reg).unwrap();
    input
        .par_iter()
        .filter(|vec| vec.iter().any(|&s| s.contains(patern)))
        .map(|vec| mrpropre(vec, &regex))
        .collect()
}

/// take all hashmap to generate the final struct
/// h1 contain name and colum of the db and h2 their different key
fn e2pz(column: HashMap<&str,Vec<(&str,&str)>>, primary_key: Vec<Vec<&str>>, foreign_key: HashMap<&str, Vec<(&str, &str, &str)>>){
    let mut tables = vec![];

    for (key,value) in column{
        let mut colum_data = vec![];

        let has_foreign_key = foreign_key.contains_key(key);

        for (name,type_k) in value {
            let mut col = TableColumn{
                name,
                data_type: type_k,
                is_primary_key: false,
                is_foreign_key: false,
                references_table: None,
                references_column: None,
            };

            if has_foreign_key {
                if let Some(data_foreign_key) = foreign_key.get(key) {
                    for data in data_foreign_key {
                        if data.0 == name {
                            col.is_foreign_key = true;
                            col.references_table = Some(data.1);
                            col.references_column = Some(data.2);
                            break;
                        }
                    }
                } 
            }

            for data in &primary_key {
                if data[0] == key && data[1] == name{
                    col.is_primary_key = true;
                    break;
                }
            }

            colum_data.push(col);
        }
            
        tables.push(SqlTable {
             name: key,
             column: colum_data,
        });
    }

    for table in tables {
        println!("Table: {}", table.name);
        println!("Columns: {:?}", table.column);
    }
}


fn main() -> Result<(), SystemTrayError> {
    let file = match File::open("keb3.sql") {
        Ok(file) => file,
        Err(err) => {
            return Err(SystemTrayError {
                message: err.to_string(),
            });
        }
    };

    let mut buf_reader = BufReader::new(file);

    let mut whole_file = String::new();

    if let Err(err) = buf_reader.read_to_string(&mut whole_file) {
        return Err(SystemTrayError {
            message: err.to_string(),
        });
    }

    let starter = scanner(&whole_file, "CREATE");

    let column = red(&starter);
    let foreign = mrclean(&starter, "CONSTRAINT");
    let primary = mrclean(&starter, "PRIMARY");

    omega(column, foreign, primary);

    mapping("sample.xml")
}

/// gather foreign key and link table 
fn omega(column: HashMap<&str,Vec<(&str,&str)>>, foreign: Vec<Vec<&str>>, primary: Vec<Vec<&str>>) {
    let mut foreign_key: HashMap<&str, Vec<(&str, &str, &str)>> = HashMap::new();

    for vec in foreign {
        if let Some((&first_element, datas)) = vec.split_first() {
            let mut x: Vec<(&str, &str, &str)> = Vec::new();
            for data in datas {
                let item: Vec<&str> = data.split('`').collect();
                let tuple = (item[2], item[4], item[6]);
                x.push(tuple);
            }
            foreign_key.insert(first_element, x);
        }
    }

    e2pz(column, primary, foreign_key)
}



/// generate mapping file
fn mapping(file: &str) -> Result<(), SystemTrayError> {
    

    let file = match File::create(file) {
        Ok(file) => file,
        Err(err) => {
            return Err(SystemTrayError {
                message: err.to_string(),
            });
        }
    };

    let mut person = XMLElement::new("MapSet");
    person.add_attribute(
        "forSchema",
        "https://www.worldline.com/namespaces/fs/ita/3ds/acs/business/config/v1/profileset",
    );
    let mut name = XMLElement::new("Mapping");
    name.add_attribute("sourceNodePath", "acspst:ProfileSet");
    name.add_attribute("targetTable", "SQL:ProfileSet");
    person.add_child(name);

    if let Err(err) = person.write(file) {
        return Err(SystemTrayError {
            message: err.to_string(),
        });
    }

    Ok(())
}
