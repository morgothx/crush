use crate::lang::value::{Value, ValueType};
use std::collections::{HashSet, HashMap};
use std::path::Path;
use std::fs::File;
use crate::lang::errors::{CrushResult, to_crush_error, error, CrushError};
use std::io::{Write, Seek, Read, Cursor};
use prost::Message;
use std::convert::TryFrom;
use crate::util::identity_arc::Identity;
use model::SerializedValue;
use model::Element;
use model::element;
use chrono::Duration;
use crate::lang::list::List;
use std::ops::Deref;
use crate::lang::table::{Table, ColumnType, Row};

mod integer_serializer;
mod list_serializer;
mod value_type_serializer;
mod value_serializer;
mod table_serializer;

//mod model;


pub mod model {
    include!(concat!(env!("OUT_DIR"), "/lang.serialization.model.rs"));
}

pub struct SerializationState {
    pub with_id: HashMap<u64, usize>,
    pub values: HashMap<Value, usize>,
}

pub struct DeserializationState {
    pub values: HashMap<usize, Value>,
    pub lists: HashMap<usize, List>,
    pub types: HashMap<usize, ValueType>,
}

pub fn serialize(value: &Value, destination: &Path) -> CrushResult<()> {
    let mut res = SerializedValue::default();
    let mut state = SerializationState {
        with_id: HashMap::new(),
        values: HashMap::new(),
    };
    res.root = value. clone().materialize().serialize(&mut res.elements, &mut state)? as u64;

    let mut buf = Vec::new();
    buf.reserve(res.encoded_len());
    res.encode(&mut buf).unwrap();

    let mut file_buffer = to_crush_error(File::create(destination))?;
    let mut pos = 0;

    while pos < buf.len() {
        let bytes_written = to_crush_error(file_buffer.write(&buf[pos..]))?;
        pos += bytes_written;
    }
    Ok(())
}

pub fn deserialize(source: &Path) -> CrushResult<Value> {
    let mut buf = Vec::new();
    let mut file_buffer = to_crush_error(File::open(source))?;
    buf.reserve(to_crush_error(source.metadata())?.len() as usize);
    file_buffer.read_to_end(&mut buf);

    let mut state = DeserializationState {
        values: HashMap::new(),
        types: HashMap::new(),
        lists: HashMap::new(),
    };

    let mut res = SerializedValue::decode(&mut Cursor::new(buf)).unwrap();

    println!("AAA {:?}", res);

    Ok(Value::deserialize(res.root as usize, &res.elements, &mut state)?)
}

pub trait Serializable<T> {
    fn deserialize(id: usize, elements: &Vec<Element>, state: &mut DeserializationState) -> CrushResult<T>;
    fn serialize(&self, elements: &mut Vec<Element>, state: &mut SerializationState) -> CrushResult<usize>;
}

