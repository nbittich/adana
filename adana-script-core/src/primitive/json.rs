use std::collections::BTreeMap;

use anyhow::anyhow;
use serde_json::{Value, json};

use super::Primitive;

pub trait Json {
    fn from_json(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn to_json(&self) -> anyhow::Result<String>;
}

fn json_to_primitive(value: Value) -> anyhow::Result<Primitive> {
    match value {
        Value::Null => Ok(Primitive::Null),
        Value::Bool(b) => Ok(Primitive::Bool(b)),
        Value::Number(number) => number
            .as_u64()
            .map(|n| Primitive::Int(n as i128))
            .or(number.as_i64().map(|n| Primitive::Int(n as i128)))
            .or(number.as_f64().map(Primitive::Double))
            .ok_or(anyhow!("could not parse json number")),
        Value::String(s) => Ok(Primitive::String(s)),
        Value::Array(json_array) => {
            let mut prim_array = Vec::with_capacity(json_array.len());
            for v in json_array {
                prim_array.push(json_to_primitive(v)?);
            }
            Ok(Primitive::Array(prim_array))
        }
        Value::Object(o) => {
            let mut struct_array = BTreeMap::new();
            for (k, v) in o {
                struct_array.insert(k, json_to_primitive(v)?);
            }
            Ok(Primitive::Struct(struct_array))
        }
    }
}
fn primitive_to_value(p: &Primitive) -> anyhow::Result<Value> {
    match p {
        Primitive::Ref(r) => {
            let r =
                r.read().map_err(|e| anyhow!("could not acquire lock! {e}"))?;
            primitive_to_value(&r)
        }
        Primitive::U8(u) => Ok(json!(u)),
        Primitive::I8(u) => Ok(json!(u)),
        Primitive::Int(u) => Ok(json!(u)),
        Primitive::Double(u) => Ok(json!(u)),
        Primitive::Bool(b) => Ok(json!(b)),
        Primitive::Null => Ok(Value::Null),
        Primitive::String(s) => Ok(Value::String(s.to_owned())),
        Primitive::Array(prim_arr) => {
            let mut json_arr = Vec::with_capacity(prim_arr.len());
            for p in prim_arr {
                json_arr.push(primitive_to_value(p)?);
            }
            Ok(Value::Array(json_arr))
        }
        Primitive::Struct(s) => {
            let mut o = serde_json::Map::with_capacity(s.len());
            for (k, v) in s {
                o.insert(k.to_string(), primitive_to_value(v)?);
            }
            Ok(Value::Object(o))
        }
        v => Ok(json!(v.to_string())),
    }
}

impl Json for Primitive {
    fn from_json(s: &str) -> anyhow::Result<Self> {
        let value = serde_json::from_str(s)?;
        json_to_primitive(value)
    }

    fn to_json(&self) -> anyhow::Result<String> {
        let value = primitive_to_value(self)?;
        serde_json::to_string_pretty(&value).map_err(|e| anyhow!("{e}"))
    }
}
