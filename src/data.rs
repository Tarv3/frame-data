use std::{
    collections::hash_map::{HashMap, Entry},
    hash::Hash,
    error::Error,
    fmt::Debug,
};
use fnv::FnvHashMap;

#[derive(Copy, Clone, Debug)]
pub enum DataType {
    F32,
    I32,
    U32,
    Char,
    Bool,
    OwnedString,
}

impl DataType {
    pub fn to_default_value(&self) -> DataValue {
        match self {
            DataType::F32 => DataValue::F32(f32::default()),
            DataType::I32 => DataValue::I32(i32::default()),
            DataType::U32 => DataValue::U32(u32::default()),
            DataType::Char => DataValue::Char(char::default()),
            DataType::Bool => DataValue::Bool(bool::default()),
            DataType::OwnedString => DataValue::OwnedString(String::new()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum DataValue {
    F32(f32),
    I32(i32),
    U32(u32),
    Char(char),
    Bool(bool),
    OwnedString(String),
}

impl DataValue {
    pub fn set_value(&mut self, value: &str) -> Result<(), Box<Error>> {
        match self {
            DataValue::F32(ref mut val) => *val = value.parse()?,
            DataValue::I32(ref mut val) => *val = value.parse()?,
            DataValue::U32(ref mut val) => *val = value.parse()?,
            DataValue::Char(ref mut val) => *val = value.parse()?,
            DataValue::Bool(ref mut val) => *val = value.parse()?,
            DataValue::OwnedString(ref mut val) => *val = value.to_string(),
        }

        Ok(())
    }
}

impl std::fmt::Display for DataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataValue::F32(val) => write!(f, "{}", val),
            DataValue::I32(val) => write!(f, "{}", val),
            DataValue::U32(val) => write!(f, "{}", val),
            DataValue::Char(val) => write!(f, "{}", val),
            DataValue::Bool(val) => write!(f, "{}", val),
            DataValue::OwnedString(val) => write!(f, "{}", val),
        }
    }
}

pub struct DataDescription {
    pub name: String,
    pub dtype: DataType,
}

impl DataDescription {
    pub fn new(name: String, dtype: DataType) -> Self {
        DataDescription {
            name,
            dtype
        }
    }
}

pub struct DataTypeStorage<K> {
    pub description: Vec<DataDescription>,
    values: FnvHashMap<K, Vec<DataValue>>,
}

impl<K: Hash + Eq> DataTypeStorage<K> {
    pub fn new() -> DataTypeStorage<K> {
        DataTypeStorage {
            description: vec![],
            values: FnvHashMap::default(),
        }
    }

    pub fn gen_new(&mut self, key: K) -> &mut [DataValue] {
        let values = self.values.entry(key).or_insert(vec![]);
        values.clear();

        for desc in &self.description {
            values.push(desc.dtype.to_default_value());
        }

        values
    }

    pub fn get_desc(&self) -> &[DataDescription] {
        &self.description[..]
    }

    pub fn get(&self, key: &K) -> Option<&[DataValue]> {
        self.values.get(key).map(|x| x.as_slice())
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut [DataValue]> {
        self.values.get_mut(key).map(|x| &mut x[..])
    }

    pub fn get_with_desc(&self, key: &K) -> (&[DataDescription], Option<&[DataValue]>) {
        (self.description.as_slice(), self.get(key))
    }

    pub fn has_field_with_name(&self, name: &str) -> bool {
        for desc in self.description.iter() {
            if desc.name == name {
                return true;
            }
        }        

        false
    }

    pub fn add_field(&mut self, name: String, data_type: DataType) {
        self.description.push(DataDescription::new(name, data_type));

        for value in self.values.values_mut() {
            value.push(data_type.to_default_value());
        }
    }

    pub fn remove_field_with_name(&mut self, name: &str) {
        let mut contains = false;
        let mut index = 0;
        for (i, desc) in self.description.iter().enumerate() {
            if desc.name == name {
                contains = true;
                index = i;
                break;
            }
        }

        if !contains {
            return;
        }

        for value in self.values.values_mut() {
            value.remove(index);
        }
    }

    pub fn remove_field(&mut self, index: usize) {
        if index >= self.description.len() {
            return;
        }

        self.description.remove(index);

        for value in self.values.values_mut() {
            value.remove(index);
        }
    }
}

pub struct DataStorage<K>  {
    pub data: HashMap<String, DataTypeStorage<K>>
}

impl<K: Hash + Eq> DataStorage<K> {
    pub fn new() -> DataStorage<K> {
        DataStorage {
            data: HashMap::new(),
        }
    }

    pub fn new_data_type(&mut self, name: String) {
        self.insert(name, DataTypeStorage::new());
    }

    pub fn insert(&mut self, key: String, dtype: DataTypeStorage<K>) {
        self.data.insert(key, dtype);
    }

    pub fn entry(&mut self, key: String) -> Entry<String, DataTypeStorage<K>> {
        self.data.entry(key)
    }

    pub fn get_storage<'a>(&'a self, data_type: &str) -> Option<&'a DataTypeStorage<K>> {
        self.data.get(data_type)
    }

    pub fn get_storage_mut<'a>(&'a mut self, data_type: &str) -> Option<&'a mut DataTypeStorage<K>> {
        self.data.get_mut(data_type)
    }

    pub fn get(&self, data_type: &str, key: &K) -> Option<&[DataValue]> {
        self.data.get(data_type).map(|x| x.get(key)).unwrap_or(None)
    }

    pub fn get_with_desc(&self, data_type: &str, key: &K) -> Option<(&[DataDescription], &[DataValue])> {
        self.data.get(data_type).map(|x| {
            let (desc, value) = x.get_with_desc(key);

            value.map(|x| (desc, x))
        }).unwrap_or(None)
    }
}

pub struct IndexGenerator {
    max_index: Option<u32>,
    available: Vec<u32>,
}

impl IndexGenerator {
    pub fn new() -> IndexGenerator {
        IndexGenerator {
            max_index: None,
            available: vec![],
        }
    }

    pub fn next_index(&mut self) -> u32 {
        if self.max_index.is_none() {
            self.max_index = Some(0);
            0
        }
        else if let Some(index) = self.available.pop() {
            index
        }
        else {
            let value = self.max_index.as_mut().unwrap();
            *value += 1;
            *value
        }
    }

    pub fn remove_index(&mut self, index: u32) {
        if Some(index) == self.max_index && index == 0 {
            self.max_index = None;
            self.available.clear();
        }
        else if Some(index) == self.max_index && index != 0 {
            *self.max_index.as_mut().unwrap()  -= 1;
            self.available.retain(|x| *x != index);
        }
        else if let Some(ref mut ind) = self.max_index {
            if index < *ind && !self.available.contains(&index) {
                self.available.push(index);
            }
        }
    }
}