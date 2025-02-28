use std::sync::{LazyLock, Mutex};

use reqwest::{blocking::Client, header::HeaderMap};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

static OFFSET: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("-1".to_string()));
static HANDLE: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new());

#[derive(Debug, Deserialize)]
struct TempValue {
    value: Option<Value>,
    headers: Value,
}

#[derive(Debug)]
pub enum SyncOperation {
    Insert(Insert),
    Update(Update),
    Delete(Delete),
    UpToDate,
}

impl<'de> Deserialize<'de> for SyncOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = TempValue::deserialize(deserializer)?;
        match value.headers.get("operation") {
            Some(x) if x == "insert" => Ok(Self::Insert(
                Insert::deserialize(value.value.unwrap()).unwrap(),
            )),
            Some(x) if x == "delete" => Ok(Self::Delete(
                Delete::deserialize(value.value.unwrap()).unwrap(),
            )),
            Some(x) if x == "update" => Ok(Self::Update(
                Update::deserialize(value.value.unwrap()).unwrap(),
            )),
            Some(_) => Ok(Self::UpToDate),
            None => Ok(Self::UpToDate),
        }
    }
}

#[derive(Debug)]
pub struct Insert {
    pub id: i32,
    pub name: String,
    pub age: Option<i32>,
}

impl<'de> Deserialize<'de> for Insert {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Ok(Self {
            id: value.get("id").unwrap().as_str().unwrap().parse().unwrap(),
            name: value.get("name").unwrap().as_str().unwrap().to_string(),
            age: value
                .get("age")
                .unwrap()
                .as_str()
                .map(|x| x.parse().unwrap()),
        })
    }
}

#[derive(Debug)]
pub struct Update {
    pub id: i32,
    pub name: Option<String>,
    pub age: Option<Option<i32>>,
}

impl<'de> Deserialize<'de> for Update {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Ok(Self {
            id: value.get("id").unwrap().as_str().unwrap().parse().unwrap(),
            name: value.get("name").map(|x| x.as_str().unwrap().to_string()),
            age: value
                .get("age")
                .map(|x| x.as_str().map(|y| y.parse().unwrap())),
        })
    }
}

#[derive(Debug)]
pub struct Delete {
    pub id: i32,
}

impl<'de> Deserialize<'de> for Delete {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Ok(Self {
            id: value.get("id").unwrap().as_str().unwrap().parse().unwrap(),
        })
    }
}

impl From<TempValue> for SyncOperation {
    fn from(tt: TempValue) -> Self {
        match tt.headers.get("operation") {
            Some(x) if x == "insert" => {
                Self::Insert(serde_json::from_value::<Insert>(tt.value.unwrap()).unwrap())
            }
            Some(x) if x == "update" => {
                Self::Update(serde_json::from_value::<Update>(tt.value.unwrap()).unwrap())
            }
            Some(x) if x == "delete" => {
                Self::Delete(serde_json::from_value::<Delete>(tt.value.unwrap()).unwrap())
            }
            _ => Self::UpToDate,
        }
    }
}

fn get_header(headers: &HeaderMap, header_name: &str) -> String {
    headers
        .get(header_name)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn sync() -> Vec<SyncOperation> {
    let mut handle = HANDLE.lock().unwrap();
    let mut offset = OFFSET.lock().unwrap();
    let res = CLIENT
        .get(format!(
            "http://localhost:3000/v1/shape?table=person&offset={}{}",
            offset,
            handle
                .as_ref()
                .map_or("".into(), |x| format!("&handle={}", x))
        ))
        .send()
        .unwrap();
    let headers = res.headers().clone();
    *handle = Some(get_header(&headers, "electric-handle"));
    *offset = get_header(&headers, "electric-offset");
    res.json::<Vec<SyncOperation>>().unwrap()
}
