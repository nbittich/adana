use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::Display,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use anyhow::Result;

use serde::{Deserialize, Serialize};

use super::{constants::NULL, Value};

const MAX_U32_AS_I128: i128 = u32::MAX as i128;

pub const TYPE_U8: &str = "u8";
pub const TYPE_I8: &str = "i8";
pub const TYPE_INT: &str = "int";
pub const TYPE_BOOL: &str = "bool";
pub const TYPE_NULL: &str = "null";
pub const TYPE_DOUBLE: &str = "double";
pub const TYPE_STRING: &str = "string";
pub const TYPE_ARRAY: &str = "array";
pub const TYPE_ERROR: &str = "error";
pub const TYPE_NATIVE_LIB: &str = "nativeLibrary";
pub const TYPE_FUNCTION: &str = "function";
pub const TYPE_UNIT: &str = "unit";
pub const TYPE_STRUCT: &str = "struct";
pub const TYPE_NO_RETURN: &str = "!";

#[derive(Debug)]
pub struct NativeLibrary {
    #[cfg(target_arch = "wasm32")]
    _lib: (),
    #[cfg(not(target_arch = "wasm32"))]
    lib: libloading::Library,
    path: PathBuf,
}

pub type NativeFunctionCallResult = anyhow::Result<Primitive>;
pub type Compiler = dyn FnMut(Value, BTreeMap<String, RefPrimitive>) -> NativeFunctionCallResult
    + Send;

#[cfg(not(target_arch = "wasm32"))]
#[allow(improper_ctypes_definitions)]
pub type NativeFunction<'lib> = libloading::Symbol<
    'lib,
    fn(Vec<Primitive>, Box<Compiler>) -> NativeFunctionCallResult,
>;

#[cfg(target_arch = "wasm32")]
pub type NativeFunction = ();

impl NativeLibrary {
    /// # Safety
    /// trust me bro
    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn new(path: &Path) -> anyhow::Result<NativeLibrary> {
        let lib = libloading::Library::new(path)
            .map_err(|e| anyhow::format_err!("could not load lib, {e}"))?;
        Ok(NativeLibrary { lib, path: path.to_path_buf() })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(_path: &Path) -> anyhow::Result<NativeLibrary> {
        Err(anyhow::format_err!("cannot use lib loading in wasm context!"))
    }

    pub fn get_path(&self) -> &Path {
        self.path.as_path()
    }

    /// # Safety
    /// trust me bro
    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn get_function(
        &self,
        key: &str,
    ) -> anyhow::Result<NativeFunction> {
        let r = self.lib.get(key.as_bytes())?;
        Ok(r)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_function(&self, _key: &str) -> anyhow::Result<NativeFunction> {
        Err(anyhow::format_err!("cannot use lib loading in wasm context!"))
    }

    /// # Safety
    /// trust me bro
    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn call_function(
        &self,
        key: &str,
        params: Vec<Primitive>,
        compiler: Box<Compiler>,
    ) -> anyhow::Result<Primitive> {
        let fun = self.get_function(key)?;
        fun(params, compiler)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn call_function(
        &self,
        _key: &str,
        _params: Vec<Primitive>,
        _compiler: Box<Compiler>,
    ) -> anyhow::Result<Primitive> {
        Err(anyhow::format_err!("cannot use lib loading in wasm context!"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub enum Primitive {
    U8(u8),
    I8(i8),
    Int(i128),
    Bool(bool),
    Ref(RefPrimitive),
    Null,
    Double(f64),
    String(String),
    Array(Vec<Primitive>),
    Struct(BTreeMap<String, Primitive>),
    Error(String),
    Function {
        parameters: Vec<Value>,
        exprs: Vec<Value>,
    },
    Unit,
    NoReturn,
    EarlyReturn(Box<Primitive>),
    #[serde(skip_serializing, skip_deserializing)]
    NativeLibrary(Arc<NativeLibrary>),
    #[serde(skip_serializing, skip_deserializing)]
    NativeFunction(String, Arc<NativeLibrary>),
}

pub type RefPrimitive = Arc<RwLock<Primitive>>;

// region: traits
impl Primitive {
    pub fn ref_prim(self) -> RefPrimitive {
        Arc::new(RwLock::new(self))
    }
    pub fn to_value(self) -> anyhow::Result<Value> {
        let v = match self {
            Primitive::U8(u) => Value::U8(u),
            Primitive::I8(i) => Value::I8(i),
            Primitive::Int(i) => Value::Integer(i),
            Primitive::Bool(b) => Value::Bool(b),
            Primitive::Ref(r) => {
                let r = r
                    .read()
                    .map_err(|e| {
                        anyhow::format_err!("could not acquire lock {e}")
                    })?
                    .clone();
                r.to_value()?
            }
            Primitive::Null => Value::Null,
            Primitive::Double(d) => Value::Decimal(d),
            Primitive::String(s) => Value::String(s),
            Primitive::Array(a) => {
                let mut vek = vec![];
                for p in a {
                    let p = p.to_value()?;
                    vek.push(p);
                }
                Value::Array(vek)
            }
            Primitive::Struct(s) => {
                let mut map = BTreeMap::new();
                for (k, v) in s {
                    let p = v.to_value()?;
                    map.insert(k, p);
                }
                Value::Struct(map)
            }
            Primitive::Error(e) => return Err(anyhow::format_err!("{e}")),
            Primitive::Function { parameters, exprs } => Value::Function {
                parameters: Box::new(Value::BlockParen(parameters)),
                exprs,
            },
            Primitive::Unit => Value::NoOp,
            Primitive::NoReturn => Value::NoOp,
            Primitive::EarlyReturn(e) => {
                let v = e.to_value()?;
                Value::EarlyReturn(Box::new(Some(v)))
            }
            Primitive::NativeLibrary(_) => {
                return Err(anyhow::anyhow!(
                    "cannot convert native lib to value"
                ));
            }
            Primitive::NativeFunction(_method, _lib) => {
                return Err(anyhow::anyhow!(
                    "cannot convert native function to value"
                ));
            }
        };
        Ok(v)
    }
}

pub trait TypeOf {
    fn type_of(&self) -> Self;
    fn type_of_str(&self) -> &'static str;
}
pub trait BitShift {
    fn left_shift(&self, n: &Self) -> Self;
    fn right_shift(&self, n: &Self) -> Self;
}
pub trait ToBool {
    fn to_bool(&self) -> Self;
}
pub trait ToNumber {
    fn to_int(&self) -> Self;
    fn to_double(&self) -> Self;
}
pub trait Pow {
    fn pow(&self, n: &Self) -> Self;
}

pub trait And {
    fn and(&self, n: &Self) -> Self;
    fn bitwise_and(&self, n: &Self) -> Self;
}
pub trait Or {
    fn or(&self, n: &Self) -> Self;
    fn bitwise_or(&self, n: &Self) -> Self;
    fn bitwise_xor(&self, n: &Self) -> Self;
}
pub trait Sqrt {
    fn sqrt(&self) -> Self;
}
pub trait Abs {
    fn abs(&self) -> Self;
}
pub trait Logarithm {
    fn log(&self) -> Self;
    fn ln(&self) -> Self;
}

pub trait DisplayHex {
    fn to_hex(&self) -> Self;
}

pub trait DisplayBinary {
    fn to_binary(&self) -> Self;
}

pub trait Sin {
    fn sin(&self) -> Self;
}
pub trait Array {
    fn index_at(&self, rhs: &Self) -> Self;
    fn len(&self) -> Primitive;
    fn is_empty(&self) -> Primitive;
    fn swap_mem(&mut self, rhs: &mut Self, index: &Primitive) -> Self;
    fn remove(&mut self, key: &Self) -> anyhow::Result<()>;
}

pub trait Cos {
    fn cos(&self) -> Self;
}
pub trait Tan {
    fn tan(&self) -> Self;
}

pub trait Add {
    fn add(&self, rhs: &Self) -> Self;
}

pub trait Neg {
    fn neg(&self) -> Self;
}

pub trait Not {
    fn bitwise_not(&self) -> Self;
    fn not(&self) -> Self;
}

pub trait Sub {
    fn sub(&self, rhs: &Self) -> Self;
}

pub trait Mul {
    fn mul(&self, rhs: &Self) -> Self;
}

pub trait Div {
    fn div(&self, rhs: &Self) -> Self;
}

pub trait Rem {
    fn rem(&self, rhs: &Self) -> Self;
}
// endregion traits

// region: impl primitive
#[allow(dead_code)]
impl Primitive {
    pub fn is_greater_than(&self, other: &Primitive) -> Primitive {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => Primitive::Bool(true),
            Some(Ordering::Less) => Primitive::Bool(false),
            Some(Ordering::Equal) => Primitive::Bool(false),
            None => Primitive::Error(format!(
                "call to is_greater_than() for two different types {self} => {other}"
            )),
        }
    }
    pub fn is_greater_or_equal(&self, other: &Primitive) -> Primitive {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => Primitive::Bool(true),
            Some(Ordering::Less) => Primitive::Bool(false),
            None => Primitive::Error(format!(
                "call to is_greater_or_equal() for two different types {self} => {other}"
            )),
        }
    }
    pub fn is_less_than(&self, other: &Primitive) -> Primitive {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => Primitive::Bool(true),
            Some(Ordering::Greater) => Primitive::Bool(false),
            Some(Ordering::Equal) => Primitive::Bool(false),
            None => Primitive::Error(format!(
                "call to is_less_than() for two different types {self} => {other}"
            )),
        }
    }
    pub fn is_less_or_equal(&self, other: &Primitive) -> Primitive {
        match self.partial_cmp(other) {
            Some(Ordering::Less) | Some(Ordering::Equal) => Primitive::Bool(true),
            Some(Ordering::Greater) => Primitive::Bool(false),
            None => Primitive::Error(format!(
                "call to is_less_or_equal() for two different types {self} => {other}"
            )),
        }
    }
    pub fn is_equal(&self, other: &Primitive) -> Primitive {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => Primitive::Bool(true),
            Some(Ordering::Less) | Some(Ordering::Greater) => Primitive::Bool(false),
            None => match (self, other) {
                (Primitive::Null, _) | (_, Primitive::Null) => Primitive::Bool(false),
                (Primitive::Struct(_), Primitive::Struct(_)) => Primitive::Bool(false),
                _ => Primitive::Error(format!(
                    "call to is_equal() for two different types {self} => {other}"
                )),
            },
        }
    }
    pub fn as_ref_ok(&self) -> Result<&Primitive> {
        match self {
            Primitive::Error(msg) => Err(anyhow::Error::msg(msg.to_string())),

            _ => Ok(self),
        }
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Ref(s) => {
                let lock = s.read().expect("FMT ERROR: could not acquire lock");
                write!(f, "{lock}")
            }
            Primitive::U8(u) => write!(f, "{u}"),
            Primitive::I8(u) => write!(f, "{u}"),
            Primitive::Int(i) => write!(f, "{i}"),
            Primitive::Double(d) => write!(f, "{d}"),
            Primitive::Bool(b) => write!(f, "{b}"),
            Primitive::Error(e) => write!(f, "Err: {e}"),
            Primitive::String(s) => {
                write!(f, "{s}")
            }
            Primitive::Unit => write!(f, "()"),
            Primitive::Array(arr) => {
                let joined_arr = arr
                    .iter()
                    .map(|p| match p {
                        Primitive::String(s) => format!(r#""{s}""#),
                        _ => p.to_string(),
                    })
                    .collect::<Vec<_>>();
                write!(f, "[{}]", joined_arr[..].join(", "))
            }
            Primitive::Struct(struc) => {
                let joined_arr = struc
                    .iter()
                    .map(|(k, p)| {
                        format!(
                            "\t{k}: {}",
                            if let Primitive::String(s) = p {
                                format!(r#""{s}""#)
                            } else {
                                p.to_string()
                            }
                        )
                    })
                    .collect::<Vec<_>>();
                write!(f, "struct {{\n{}\n}}", joined_arr[..].join(", \n"))
            }
            Primitive::Function { parameters, exprs: _ } => {
                let mut parameters_formatted = String::new();
                let len = parameters.len();
                for (idx, p) in parameters.iter().enumerate() {
                    match p {
                        Value::VariableUnused => parameters_formatted.push('_'),
                        _ => {
                            parameters_formatted.push('p');
                            parameters_formatted.push(
                                char::from_digit(idx as u32, 10).unwrap_or('0'),
                            );
                        }
                    }
                    if idx < len {
                        parameters_formatted.push(',');
                    }
                }
                write!(f, "({parameters_formatted}) => {{..}}")
            }
            Primitive::NoReturn => write!(f, "!"),
            Primitive::Null => write!(f, "{NULL}"),
            Primitive::EarlyReturn(p) => write!(f, "{p}"),
            Primitive::NativeLibrary { .. } => write!(f, "__native_lib__"),
            Primitive::NativeFunction(key, _) => {
                write!(f, "__native_fn__{key}")
            }
        }
    }
}

impl Sin for Primitive {
    fn sin(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock =
                    s.read().expect("SIN ERORR: could not acquire lock!");
                lock.sin()
            }
            Primitive::U8(i) => Primitive::Double((*i as f64).sin()),
            Primitive::I8(i) => Primitive::Double((*i as f64).sin()),
            Primitive::Int(i) => Primitive::Double((*i as f64).sin()),
            Primitive::Double(d) => Primitive::Double(d.sin()),

            Primitive::Error(e) => panic!("call to sin() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to sin() => {self}")),
        }
    }
}

impl Cos for Primitive {
    fn cos(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock =
                    s.read().expect("COS ERORR: could not acquire lock!");
                lock.cos()
            }
            Primitive::Int(i) => Primitive::Double((*i as f64).cos()),
            Primitive::U8(i) => Primitive::Double((*i as f64).cos()),
            Primitive::I8(i) => Primitive::Double((*i as f64).cos()),
            Primitive::Double(d) => Primitive::Double(d.cos()),
            Primitive::Error(e) => panic!("call to cos() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to cos() => {self}")),
        }
    }
}

impl Tan for Primitive {
    fn tan(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock =
                    s.read().expect("TAN ERORR: could not acquire lock!");
                lock.tan()
            }

            Primitive::U8(i) => Primitive::Double((*i as f64).tan()),
            Primitive::I8(i) => Primitive::Double((*i as f64).tan()),
            Primitive::Int(i) => Primitive::Double((*i as f64).tan()),
            Primitive::Double(d) => Primitive::Double(d.tan()),
            Primitive::Error(e) => panic!("call to tan() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to tan() => {self}")),
        }
    }
}

impl Logarithm for Primitive {
    fn log(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock =
                    s.read().expect("LOG ERORR: could not acquire lock!");
                lock.log()
            }

            Primitive::U8(i) => Primitive::Double((*i as f64).log10()),
            Primitive::I8(i) => Primitive::Double((*i as f64).log10()),
            Primitive::Int(i) => Primitive::Double((*i as f64).log10()),
            Primitive::Double(d) => Primitive::Double(d.log10()),
            Primitive::Error(e) => panic!("call to log() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to log() => {self}")),
        }
    }
    fn ln(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock = s.read().expect("LN ERORR: could not acquire lock!");
                lock.ln()
            }

            Primitive::U8(i) => Primitive::Double((*i as f64).ln()),
            Primitive::I8(i) => Primitive::Double((*i as f64).ln()),
            Primitive::Int(i) => Primitive::Double((*i as f64).ln()),
            Primitive::Double(d) => Primitive::Double(d.ln()),
            Primitive::Error(e) => panic!("call to ln() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to ln() => {self}")),
        }
    }
}

impl Sqrt for Primitive {
    fn sqrt(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock =
                    s.read().expect("SQRT ERORR: could not acquire lock!");
                lock.sqrt()
            }
            Primitive::U8(i) => Primitive::Double((*i as f64).sqrt()),
            Primitive::I8(i) => Primitive::Double((*i as f64).sqrt()),
            Primitive::Int(i) => Primitive::Double((*i as f64).sqrt()),
            Primitive::Double(d) => Primitive::Double(d.sqrt()),
            Primitive::Error(e) => panic!("call to sqrt() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to sqrt() => {self}")),
        }
    }
}
impl Abs for Primitive {
    fn abs(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock =
                    s.read().expect("ABS ERORR: could not acquire lock!");
                lock.abs()
            }
            Primitive::U8(i) => Primitive::U8(*i),
            Primitive::I8(i) => Primitive::I8(i.abs()),
            Primitive::Int(i) => Primitive::Int(i.abs()),
            Primitive::Double(d) => Primitive::Double(d.abs()),
            Primitive::Error(e) => panic!("call to abs() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to abs() => {self}")),
        }
    }
}

impl Pow for Primitive {
    fn pow(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Primitive::Ref(l), Primitive::Ref(r)) => {
                let l = l.read().expect("POW L ERORR: could not acquire lock!");

                let r = r.read().expect("POW R ERORR: could not acquire lock!");
                l.pow(&r)
            }
            (Primitive::Ref(l), r) => {
                let l = l.read().expect("POW L ERORR: could not acquire lock!");

                l.pow(r)
            }
            (l, Primitive::Ref(r)) => {
                let r = r.read().expect("POW R ERORR: could not acquire lock!");

                l.pow(&r)
            }
            (Primitive::U8(l), Primitive::U8(r)) => {
                Primitive::Int((*l as i128).pow(*r as u32))
            }
            (Primitive::U8(l), Primitive::I8(r)) if r > &0 => {
                Primitive::Int((*l as i128).pow(*r as u32))
            }
            (Primitive::U8(l), Primitive::I8(r)) => {
                Primitive::Double((*l as f64).powf(*r as f64))
            }
            #[allow(clippy::manual_range_contains)]
            (Primitive::U8(l), Primitive::Int(r))
                if r >= &0 && r <= &MAX_U32_AS_I128 =>
            {
                Primitive::Int((*l as i128).pow(*r as u32))
            }

            (Primitive::I8(l), Primitive::I8(r)) if r > &0 => {
                Primitive::Int((*l as i128).pow(*r as u32))
            }
            (Primitive::I8(l), Primitive::I8(r)) => {
                Primitive::Double((*l as f64).powf(*r as f64))
            }

            (Primitive::I8(l), Primitive::U8(r)) => {
                Primitive::Int((*l as i128).pow(*r as u32))
            }
            #[allow(clippy::manual_range_contains)]
            (Primitive::I8(l), Primitive::Int(r))
                if r >= &0 && r <= &MAX_U32_AS_I128 =>
            {
                Primitive::Int((*l as i128).pow(*r as u32))
            }

            (Primitive::U8(l), Primitive::Int(r)) => {
                Primitive::Double((*l as f64).powf(*r as f64))
            }
            (Primitive::I8(l), Primitive::Int(r)) => {
                Primitive::Double((*l as f64).powf(*r as f64))
            }
            (Primitive::U8(l), Primitive::Double(r)) => {
                Primitive::Double((*l as f64).powf(*r))
            }
            (Primitive::I8(l), Primitive::Double(r)) => {
                Primitive::Double((*l as f64).powf(*r))
            }

            #[allow(clippy::manual_range_contains)]
            (Primitive::Int(l), Primitive::Int(r))
                if r >= &0 && r <= &MAX_U32_AS_I128 =>
            {
                Primitive::Int(l.pow(*r as u32))
            }

            (Primitive::Int(l), Primitive::U8(r)) => {
                Primitive::Int(l.pow(*r as u32))
            }
            (Primitive::Int(l), Primitive::I8(r)) if r >= &0 => {
                Primitive::Int(l.pow(*r as u32))
            }
            (Primitive::Int(l), Primitive::I8(r)) => {
                Primitive::Double((*l as f64).powf(*r as f64))
            }
            (Primitive::Int(l), Primitive::Int(r)) => {
                Primitive::Double((*l as f64).powf(*r as f64))
            }
            (Primitive::Int(l), Primitive::Double(r)) => {
                Primitive::Double((*l as f64).powf(*r))
            }
            (Primitive::Double(l), Primitive::Int(r)) => {
                Primitive::Double(l.powf(*r as f64))
            }
            (Primitive::Double(l), Primitive::U8(r)) => {
                Primitive::Double(l.powf(*r as f64))
            }
            (Primitive::Double(l), Primitive::I8(r)) => {
                Primitive::Double(l.powf(*r as f64))
            }
            (Primitive::Double(l), Primitive::Double(r)) => {
                Primitive::Double(l.powf(*r))
            }
            (l, r) => Primitive::Error(format!(
                "illegal call to pow() => left: {l} right: {r}"
            )),
        }
    }
}

impl Add for Primitive {
    fn add(&self, rhs: &Self) -> Self {
        match (self.clone(), rhs.clone()) {
            (Primitive::Ref(l), Primitive::Ref(r)) => {
                let l = l.read().expect("ADD L ERORR: could not acquire lock!");

                let r = r.read().expect("ADD R ERORR: could not acquire lock!");
                l.add(&r)
            }
            (Primitive::Ref(l), r) => {
                let l = l.read().expect("ADD L ERORR: could not acquire lock!");

                l.add(&r)
            }
            (l, Primitive::Ref(r)) => {
                let r = r.read().expect("ADD R ERORR: could not acquire lock!");

                l.add(&r)
            }
            (Primitive::U8(l), Primitive::U8(r)) => {
                Primitive::Int(l as i128 + r as i128)
            }
            (Primitive::U8(l), Primitive::I8(r)) => {
                Primitive::Int(l as i128 + r as i128)
            }
            (Primitive::U8(l), Primitive::Int(r)) => {
                Primitive::Int(l as i128 + r)
            }
            (Primitive::U8(l), Primitive::String(s)) => {
                Primitive::String(format!("{l}{s}"))
            }
            (Primitive::U8(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 + r)
            }

            (Primitive::I8(l), Primitive::I8(r)) => {
                Primitive::Int(l as i128 + r as i128)
            }
            (Primitive::I8(l), Primitive::U8(r)) => {
                Primitive::Int(l as i128 + r as i128)
            }
            (Primitive::I8(l), Primitive::Int(r)) => {
                Primitive::Int(l as i128 + r)
            }
            (Primitive::I8(l), Primitive::String(s)) => {
                Primitive::String(format!("{l}{s}"))
            }
            (Primitive::I8(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 + r)
            }

            (Primitive::Int(l), Primitive::Int(r)) => Primitive::Int(l + r),
            (Primitive::Int(l), Primitive::U8(r)) => {
                Primitive::Int(l + r as i128)
            }
            (Primitive::Int(l), Primitive::I8(r)) => {
                Primitive::Int(l + r as i128)
            }
            (Primitive::Int(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 + r)
            }
            (Primitive::Int(l), Primitive::String(s)) => {
                Primitive::String(format!("{l}{s}"))
            }

            (Primitive::Double(l), Primitive::Int(r)) => {
                Primitive::Double(l + r as f64)
            }
            (Primitive::Double(l), Primitive::U8(r)) => {
                Primitive::Double(l + r as f64)
            }
            (Primitive::Double(l), Primitive::I8(r)) => {
                Primitive::Double(l + r as f64)
            }
            (Primitive::Double(l), Primitive::Double(r)) => {
                Primitive::Double(l + r)
            }

            (Primitive::Array(mut l), Primitive::Array(mut r)) => {
                l.append(&mut r);
                Primitive::Array(l)
            }

            (Primitive::Array(mut l), r) => {
                let r: Primitive = r;
                l.push(r);
                Primitive::Array(l)
            }
            (l, Primitive::Array(mut r)) => {
                r.insert(0, l);
                Primitive::Array(r)
            }
            (l, Primitive::String(s)) => Primitive::String(format!("{l}{s}")),

            (Primitive::String(s), r) => Primitive::String(format!("{s}{r}")),
            (l, r) => Primitive::Error(format!(
                "illegal call to add() => left: {l} right: {r}"
            )),
        }
    }
}

impl Sub for Primitive {
    fn sub(&self, rhs: &Self) -> Self {
        match (self.clone(), rhs.clone()) {
            (Primitive::Ref(l), Primitive::Ref(r)) => {
                let l = l.read().expect("SUB L ERORR: could not acquire lock!");

                let r = r.read().expect("SUB R ERORR: could not acquire lock!");
                l.sub(&r)
            }
            (Primitive::Ref(l), r) => {
                let l = l.read().expect("SUB L ERORR: could not acquire lock!");

                l.sub(&r)
            }
            (l, Primitive::Ref(r)) => {
                let r = r.read().expect("SUB R ERORR: could not acquire lock!");

                l.sub(&r)
            }

            (Primitive::U8(l), Primitive::U8(r)) => {
                if let Some(v) = l.checked_sub(r) {
                    Primitive::U8(v)
                } else {
                    Primitive::Int(l as i128 - r as i128)
                }
            }
            (Primitive::U8(l), Primitive::I8(r)) => {
                Primitive::Int(l as i128 - r as i128)
            }
            (Primitive::U8(l), Primitive::Int(r)) => {
                Primitive::Int(l as i128 - r)
            }
            (Primitive::U8(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 - r)
            }

            (Primitive::I8(l), Primitive::U8(r)) => {
                Primitive::Int(l as i128 - r as i128)
            }
            (Primitive::I8(l), Primitive::I8(r)) => {
                if let Some(v) = l.checked_sub(r) {
                    Primitive::I8(v)
                } else {
                    Primitive::Int(l as i128 - r as i128)
                }
            }
            (Primitive::I8(l), Primitive::Int(r)) => {
                Primitive::Int(l as i128 - r)
            }
            (Primitive::I8(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 - r)
            }

            (Primitive::Int(l), Primitive::Int(r)) => Primitive::Int(l - r),
            (Primitive::Int(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 - r)
            }
            (Primitive::Int(l), Primitive::U8(r)) => {
                Primitive::Int(l - r as i128)
            }
            (Primitive::Int(l), Primitive::I8(r)) => {
                Primitive::Int(l - r as i128)
            }

            (Primitive::Double(l), Primitive::Int(r)) => {
                Primitive::Double(l - r as f64)
            }
            (Primitive::Double(l), Primitive::Double(r)) => {
                Primitive::Double(l - r)
            }
            (Primitive::Double(l), Primitive::U8(r)) => {
                Primitive::Double(l - r as f64)
            }
            (Primitive::Double(l), Primitive::I8(r)) => {
                Primitive::Double(l - r as f64)
            }

            (l, r) => Primitive::Error(format!(
                "illegal call to sub() => left: {l} right: {r}"
            )),
        }
    }
}
impl Rem for Primitive {
    fn rem(&self, rhs: &Self) -> Self {
        match (self.clone(), rhs.clone()) {
            (Primitive::Ref(l), Primitive::Ref(r)) => {
                let l = l.read().expect("REM L ERORR: could not acquire lock!");

                let r = r.read().expect("REM R ERORR: could not acquire lock!");
                l.rem(&r)
            }
            (Primitive::Ref(l), r) => {
                let l = l.read().expect("REM L ERORR: could not acquire lock!");

                l.rem(&r)
            }
            (l, Primitive::Ref(r)) => {
                let r = r.read().expect("REM R ERORR: could not acquire lock!");

                l.rem(&r)
            }

            (Primitive::U8(l), Primitive::U8(r)) if r != 0 => {
                Primitive::U8(l % r)
            }
            (Primitive::U8(l), Primitive::I8(r)) if r != 0 => {
                Primitive::Int(l as i128 % r as i128)
            }
            (Primitive::U8(l), Primitive::Int(r)) if r != 0 => {
                Primitive::Int(l as i128 % r)
            }
            (Primitive::U8(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 % r)
            }
            (Primitive::U8(_), _) => Primitive::Double(f64::NAN),

            (Primitive::I8(l), Primitive::I8(r)) if r != 0 => {
                Primitive::I8(l % r)
            }
            (Primitive::I8(l), Primitive::U8(r)) if r != 0 => {
                Primitive::Int(l as i128 % r as i128)
            }
            (Primitive::I8(l), Primitive::Int(r)) if r != 0 => {
                Primitive::Int(l as i128 % r)
            }
            (Primitive::I8(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 % r)
            }
            (Primitive::I8(_), _) => Primitive::Double(f64::NAN),

            (Primitive::Int(l), Primitive::Int(r)) if r != 0 => {
                Primitive::Int(l % r)
            }
            (Primitive::Int(l), Primitive::U8(r)) if r != 0 => {
                Primitive::Int(l % r as i128)
            }
            (Primitive::Int(l), Primitive::I8(r)) if r != 0 => {
                Primitive::Int(l % r as i128)
            }
            (Primitive::Int(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 % r)
            }
            (Primitive::Int(_), _) => Primitive::Double(f64::NAN),

            (Primitive::Double(l), Primitive::U8(r)) => {
                Primitive::Double(l % r as f64)
            }
            (Primitive::Double(l), Primitive::I8(r)) => {
                Primitive::Double(l % r as f64)
            }
            (Primitive::Double(l), Primitive::Int(r)) => {
                Primitive::Double(l % r as f64)
            }
            (Primitive::Double(l), Primitive::Double(r)) => {
                Primitive::Double(l % r)
            }

            (l, r) => Primitive::Error(format!(
                "illegal call to rem() => left: {l} right: {r}"
            )),
        }
    }
}

impl Mul for Primitive {
    fn mul(&self, rhs: &Self) -> Self {
        fn multiply_array(arr: Vec<Primitive>, n: i128) -> Vec<Primitive> {
            let arr_size = arr.len();
            arr.into_iter().cycle().take(n as usize * arr_size).collect()
        }
        match (self.clone(), rhs.clone()) {
            (Primitive::Ref(l), Primitive::Ref(r)) => {
                let l = l.read().expect("MUL L ERORR: could not acquire lock!");

                let r = r.read().expect("MUL R ERORR: could not acquire lock!");
                l.mul(&r)
            }
            (Primitive::Ref(l), r) => {
                let l = l.read().expect("MUL L ERORR: could not acquire lock!");

                l.mul(&r)
            }
            (l, Primitive::Ref(r)) => {
                let r = r.read().expect("MUL R ERORR: could not acquire lock!");

                l.mul(&r)
            }

            (Primitive::U8(l), Primitive::U8(r)) if r != 0 => {
                Primitive::Int(l as i128 * r as i128)
            }
            (Primitive::U8(l), Primitive::I8(r)) if r != 0 => {
                Primitive::Int(l as i128 * r as i128)
            }
            (Primitive::U8(l), Primitive::Int(r)) if r != 0 => {
                Primitive::Int(l as i128 * r)
            }
            (Primitive::U8(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 * r)
            }
            (Primitive::U8(l), Primitive::Array(r)) => {
                Primitive::Array(multiply_array(r, l as i128))
            }

            (Primitive::I8(l), Primitive::U8(r)) if r != 0 => {
                Primitive::Int(l as i128 * r as i128)
            }
            (Primitive::I8(l), Primitive::I8(r)) if r != 0 => {
                Primitive::Int(l as i128 * r as i128)
            }
            (Primitive::I8(l), Primitive::Int(r)) if r != 0 => {
                Primitive::Int(l as i128 * r)
            }
            (Primitive::I8(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 * r)
            }
            (Primitive::I8(l), Primitive::Array(r)) if l >= 0 => {
                Primitive::Array(multiply_array(r, l as i128))
            }

            (Primitive::Int(l), Primitive::Int(r)) => {
                Primitive::Int(l.wrapping_mul(r))
            }
            (Primitive::Int(l), Primitive::U8(r)) => {
                Primitive::Int(l * r as i128)
            }
            (Primitive::Int(l), Primitive::I8(r)) => {
                Primitive::Int(l * r as i128)
            }
            (Primitive::Int(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 * r)
            }
            (Primitive::Int(l), Primitive::Array(r)) if l >= 0 => {
                Primitive::Array(multiply_array(r, l))
            }

            (Primitive::Double(l), Primitive::Int(r)) => {
                Primitive::Double(l * r as f64)
            }
            (Primitive::Double(l), Primitive::U8(r)) => {
                Primitive::Double(l * r as f64)
            }
            (Primitive::Double(l), Primitive::I8(r)) => {
                Primitive::Double(l * r as f64)
            }
            (Primitive::Double(l), Primitive::Double(r)) => {
                Primitive::Double(l * r)
            }

            (Primitive::String(l), Primitive::Int(r)) if r >= 0 => {
                Primitive::String(l.repeat(r as usize))
            }
            (Primitive::String(l), Primitive::U8(r)) => {
                Primitive::String(l.repeat(r as usize))
            }
            (Primitive::String(l), Primitive::I8(r)) if r >= 0 => {
                Primitive::String(l.repeat(r as usize))
            }

            (Primitive::Array(l), Primitive::Int(n)) if n >= 0 => {
                Primitive::Array(multiply_array(l, n))
            }
            (Primitive::Array(l), Primitive::U8(n)) => {
                Primitive::Array(multiply_array(l, n as i128))
            }
            (Primitive::Array(l), Primitive::I8(n)) if n >= 0 => {
                Primitive::Array(multiply_array(l, n as i128))
            }

            (l, r) => Primitive::Error(format!(
                "illegal call to mul() => left: {l} right: {r}"
            )),
        }
    }
}
impl Div for Primitive {
    fn div(&self, rhs: &Self) -> Self {
        match (self.clone(), rhs.clone()) {
            (Primitive::Ref(l), Primitive::Ref(r)) => {
                let l = l.read().expect("DIV L ERORR: could not acquire lock!");

                let r = r.read().expect("DIV R ERORR: could not acquire lock!");
                l.div(&r)
            }
            (Primitive::Ref(l), r) => {
                let l = l.read().expect("DIV L ERORR: could not acquire lock!");

                l.div(&r)
            }
            (l, Primitive::Ref(r)) => {
                let r = r.read().expect("DIV R ERORR: could not acquire lock!");

                l.div(&r)
            }

            (Primitive::U8(l), Primitive::U8(r)) if r != 0 => {
                Primitive::Int(l as i128 / r as i128)
            }
            (Primitive::U8(l), Primitive::I8(r)) if r != 0 => {
                Primitive::Int(l as i128 / r as i128)
            }
            (Primitive::U8(l), Primitive::Int(r)) if r != 0 => {
                Primitive::Int(l as i128 / r)
            }
            (Primitive::U8(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 / r)
            }
            (Primitive::U8(l), Primitive::Int(_)) if l >= 1 => {
                Primitive::Double(f64::INFINITY)
            }
            (Primitive::U8(l), Primitive::U8(_)) if l >= 1 => {
                Primitive::Double(f64::INFINITY)
            }
            (Primitive::U8(l), Primitive::I8(_)) if l >= 1 => {
                Primitive::Double(f64::INFINITY)
            }
            (Primitive::U8(_), _) => Primitive::Double(f64::NAN),

            (Primitive::I8(l), Primitive::U8(r)) if r != 0 => {
                Primitive::Int(l as i128 / r as i128)
            }
            (Primitive::I8(l), Primitive::I8(r)) if r != 0 => {
                Primitive::Int(l as i128 / r as i128)
            }
            (Primitive::I8(l), Primitive::Int(r)) if r != 0 => {
                Primitive::Int(l as i128 / r)
            }
            (Primitive::I8(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 / r)
            }
            (Primitive::I8(l), Primitive::Int(_)) if l >= 1 => {
                Primitive::Double(f64::INFINITY)
            }
            (Primitive::I8(l), Primitive::U8(_)) if l >= 1 => {
                Primitive::Double(f64::INFINITY)
            }
            (Primitive::I8(l), Primitive::I8(_)) if l >= 1 => {
                Primitive::Double(f64::INFINITY)
            }
            (Primitive::I8(_), _) => Primitive::Double(f64::NAN),

            (Primitive::Int(l), Primitive::Int(r)) if r != 0 => {
                Primitive::Int(l / r)
            }
            (Primitive::Int(l), Primitive::U8(r)) if r != 0 => {
                Primitive::Int(l / r as i128)
            }
            (Primitive::Int(l), Primitive::I8(r)) if r != 0 => {
                Primitive::Int(l / r as i128)
            }
            (Primitive::Int(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 / r)
            }
            (Primitive::Int(l), Primitive::Int(_)) if l >= 1 => {
                Primitive::Double(f64::INFINITY)
            }
            (Primitive::Int(l), Primitive::U8(_)) if l >= 1 => {
                Primitive::Double(f64::INFINITY)
            }
            (Primitive::Int(l), Primitive::I8(_)) if l >= 1 => {
                Primitive::Double(f64::INFINITY)
            }
            (Primitive::Int(_), _) => Primitive::Double(f64::NAN),

            (Primitive::Double(l), Primitive::Int(r)) => {
                Primitive::Double(l / r as f64)
            }
            (Primitive::Double(l), Primitive::U8(r)) => {
                Primitive::Double(l / r as f64)
            }
            (Primitive::Double(l), Primitive::I8(r)) => {
                Primitive::Double(l / r as f64)
            }
            (Primitive::Double(l), Primitive::Double(r)) => {
                Primitive::Double(l / r)
            }

            (l, r) => Primitive::Error(format!(
                "illegal call to div() => left: {l} right: {r}"
            )),
        }
    }
}

impl Neg for Primitive {
    fn neg(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock =
                    s.read().expect("NEG ERORR: could not acquire lock!");
                lock.neg()
            }
            Primitive::U8(n) if *n > i8::MAX as u8 => {
                Primitive::Int(-(*n as i128))
            }
            Primitive::U8(n) => Primitive::I8(-(*n as i8)),
            Primitive::I8(n) => Primitive::I8(-*n),

            Primitive::Int(n) => Primitive::Int(-n),
            Primitive::Double(n) => Primitive::Double(-n),
            _ => Primitive::Error(format!("invalid call to neg() {self}")),
        }
    }
}

impl Not for Primitive {
    fn not(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock =
                    s.read().expect("NOT ERORR: could not acquire lock!");
                lock.not()
            }
            Primitive::Bool(b) => Primitive::Bool(!b),
            _ => Primitive::Error(format!("invalid call to not() {self}")),
        }
    }
    fn bitwise_not(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock =
                    s.read().expect("NOT ERORR: could not acquire lock!");
                lock.not()
            }
            Primitive::U8(b) if *b < i8::MAX as u8 => {
                Primitive::I8(!(*b as i8))
            }
            Primitive::U8(b) => Primitive::Int(!(*b as i128)),
            Primitive::I8(b) => Primitive::I8(!b),
            Primitive::Int(b) => Primitive::Int(!b),
            _ => Primitive::Error(format!(
                "invalid call to bitwise_not() {self}"
            )),
        }
    }
}

impl ToBool for Primitive {
    fn to_bool(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock =
                    s.read().expect("TO_BOOL ERORR: could not acquire lock!");
                lock.to_bool()
            }
            v @ Primitive::Bool(_) => v.clone(),
            Primitive::Double(n) => Primitive::Bool(n > &0.0),
            Primitive::U8(n) => Primitive::Bool(n > &0),
            Primitive::I8(n) => Primitive::Bool(n > &0),
            Primitive::Int(n) => Primitive::Bool(n > &0),
            Primitive::Null => Primitive::Bool(false),
            Primitive::Array(a) => Primitive::Bool(!a.is_empty()),
            Primitive::String(s) => match s.parse::<bool>() {
                Ok(b) => Primitive::Bool(b),
                Err(e) => Primitive::Error(format!(
                    "invalid cast to bool: {self}, {e}"
                )),
            },
            _ => Primitive::Error(format!("invalide cast too bool: {self}")),
        }
    }
}

impl ToNumber for Primitive {
    fn to_int(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock =
                    s.read().expect("TO_INT ERORR: could not acquire lock!");
                lock.to_int()
            }
            v @ Primitive::Int(_) => v.clone(),
            v @ Primitive::U8(_) => v.clone(),
            v @ Primitive::I8(_) => v.clone(),
            Primitive::Bool(false) => Primitive::Int(0),
            Primitive::Bool(true) => Primitive::Int(1),
            Primitive::Double(d) => Primitive::Int(*d as i128),
            Primitive::String(s) => match s.parse::<i128>() {
                Ok(number) => Primitive::Int(number),
                Err(e) => Primitive::Error(format!(
                    "invalid cast to int: {self}, {e}"
                )),
            },
            _ => Primitive::Error(format!("invalid cast to int: {self}")),
        }
    }

    fn to_double(&self) -> Self {
        match self {
            Primitive::Ref(s) => {
                let lock =
                    s.read().expect("TO_DOUBLE ERORR: could not acquire lock!");
                lock.to_double()
            }
            Primitive::U8(d) => Primitive::Double(*d as f64),
            Primitive::I8(d) => Primitive::Double(*d as f64),
            Primitive::Int(d) => Primitive::Double(*d as f64),
            v @ Primitive::Double(_) => v.clone(),
            Primitive::String(s) => match s.parse::<f64>() {
                Ok(number) => Primitive::Double(number),
                Err(e) => Primitive::Error(format!(
                    "invalid cast to double: {self}, {e}"
                )),
            },
            _ => Primitive::Error(format!("invalid cast to double: {self}")),
        }
    }
}

impl BitShift for Primitive {
    fn right_shift(&self, rhs: &Self) -> Self {
        if let (&Primitive::Ref(l), &Primitive::Ref(r)) = (&self, &rhs) {
            let l =
                l.read().expect("B_R_SHIFT L ERROR: could not acquire lock!");
            let r =
                r.read().expect("B_R_SHIFT R ERROR: could not acquire lock!");
            return l.right_shift(&r);
        } else if let &Primitive::Ref(l) = &self {
            let l = l
                .read()
                .expect("B_R_SHIFT SELF ERROR: could not acquire lock!");
            return l.right_shift(rhs);
        } else if let &Primitive::Ref(r) = &rhs {
            let r =
                r.read().expect("B_R_SHIFT RHS ERROR: could not acquire lock!");
            return self.right_shift(&r);
        }

        match (self, rhs) {
            (Primitive::U8(l), Primitive::U8(r)) => {
                Primitive::Int(*l as i128 >> *r as i128)
            }
            (Primitive::U8(l), Primitive::Bool(r)) => {
                Primitive::U8(l >> if r == &true { 1 } else { 0 })
            }
            (Primitive::U8(l), Primitive::Int(r)) => {
                Primitive::Int(*l as i128 >> r)
            }
            (Primitive::U8(l), Primitive::I8(r)) => {
                Primitive::Int(*l as i128 >> *r as i128)
            }
            (Primitive::I8(l), Primitive::I8(r)) => {
                Primitive::Int(*l as i128 >> *r as i128)
            }
            (Primitive::I8(l), Primitive::Int(r)) => {
                Primitive::Int(*l as i128 >> r)
            }
            (Primitive::I8(l), Primitive::U8(r)) => {
                Primitive::Int(*l as i128 >> *r as i128)
            }
            (Primitive::I8(l), Primitive::Bool(r)) => {
                Primitive::I8(l >> if r == &true { 1 } else { 0 })
            }
            (Primitive::Int(l), Primitive::U8(r)) => {
                Primitive::Int(l >> *r as i128)
            }
            (Primitive::Int(l), Primitive::I8(r)) => {
                Primitive::Int(l >> *r as i128)
            }
            (Primitive::Int(l), Primitive::Int(r)) => Primitive::Int(l >> *r),
            (Primitive::Int(l), Primitive::Bool(r)) => {
                Primitive::Int(l >> if r == &true { 1 } else { 0 })
            }
            _ => Primitive::Error(format!(
                "illegal call to 'r_shift' => left: {self} right: {rhs}"
            )),
        }
    }

    fn left_shift(&self, rhs: &Self) -> Self {
        if let (&Primitive::Ref(l), &Primitive::Ref(r)) = (&self, &rhs) {
            let l =
                l.read().expect("B_L_SHIFT L ERROR: could not acquire lock!");
            let r =
                r.read().expect("B_L_SHIFT R ERROR: could not acquire lock!");
            return l.left_shift(&r);
        } else if let &Primitive::Ref(l) = &self {
            let l = l
                .read()
                .expect("B_L_SHIFT SELF ERROR: could not acquire lock!");
            return l.left_shift(rhs);
        } else if let &Primitive::Ref(r) = &rhs {
            let r =
                r.read().expect("B_L_SHIFT RHS ERROR: could not acquire lock!");
            return self.left_shift(&r);
        }

        match (self, rhs) {
            (Primitive::U8(l), Primitive::U8(r)) => {
                Primitive::Int((*l as i128) << *r as i128)
            }
            (Primitive::U8(l), Primitive::Bool(r)) => {
                Primitive::U8(l << if r == &true { 1 } else { 0 })
            }
            (Primitive::U8(l), Primitive::Int(r)) => {
                Primitive::Int((*l as i128) << r)
            }
            (Primitive::U8(l), Primitive::I8(r)) => {
                Primitive::Int((*l as i128) << r)
            }
            (Primitive::I8(l), Primitive::I8(r)) => {
                Primitive::Int((*l as i128) << (*r as i128))
            }
            (Primitive::I8(l), Primitive::Int(r)) => {
                Primitive::Int((*l as i128) << r)
            }
            (Primitive::I8(l), Primitive::U8(r)) => {
                Primitive::Int((*l as i128) << *r as i128)
            }
            (Primitive::I8(l), Primitive::Bool(r)) => {
                Primitive::I8(l << if r == &true { 1 } else { 0 })
            }
            (Primitive::Int(l), Primitive::U8(r)) => {
                Primitive::Int(l << *r as i128)
            }
            (Primitive::Int(l), Primitive::I8(r)) => {
                Primitive::Int(l << *r as i128)
            }
            (Primitive::Int(l), Primitive::Int(r)) => Primitive::Int(l << *r),
            (Primitive::Int(l), Primitive::Bool(r)) => {
                Primitive::Int(l << if r == &true { 1 } else { 0 })
            }
            _ => Primitive::Error(format!(
                "illegal call to 'l_shift' => left: {self} right: {rhs}"
            )),
        }
    }
}

impl Or for Primitive {
    fn or(&self, rhs: &Self) -> Self {
        if let (&Primitive::Ref(l), &Primitive::Ref(r)) = (&self, &rhs) {
            let l = l.read().expect("OR L ERROR: could not acquire lock!");
            let r = r.read().expect("OR R ERROR: could not acquire lock!");
            return l.or(&r);
        } else if let &Primitive::Ref(l) = &self {
            let l = l.read().expect("OR SELF ERROR: could not acquire lock!");
            return l.or(rhs);
        } else if let &Primitive::Ref(r) = &rhs {
            let r = r.read().expect("OR RHS ERROR: could not acquire lock!");
            return self.or(&r);
        }
        if let &Primitive::Bool(true) = &self {
            return Primitive::Bool(true);
        }
        if !matches!((self, &rhs), (Primitive::Bool(_), Primitive::Bool(_))) {
            Primitive::Error(format!(
                "illegal call to 'or' => left: {self} right: {rhs}"
            ));
        }
        rhs.clone()
    }

    fn bitwise_or(&self, rhs: &Self) -> Self {
        if let (&Primitive::Ref(l), &Primitive::Ref(r)) = (&self, &rhs) {
            let l = l.read().expect("B_OR L ERROR: could not acquire lock!");
            let r = r.read().expect("B_OR R ERROR: could not acquire lock!");
            return l.bitwise_or(&r);
        } else if let &Primitive::Ref(l) = &self {
            let l = l.read().expect("B_OR SELF ERROR: could not acquire lock!");
            return l.bitwise_or(rhs);
        } else if let &Primitive::Ref(r) = &rhs {
            let r = r.read().expect("B_OR RHS ERROR: could not acquire lock!");
            return self.bitwise_or(&r);
        }

        match (self, rhs) {
            (Primitive::U8(l), Primitive::U8(r)) => Primitive::U8(l | r),
            (Primitive::U8(l), Primitive::Bool(r)) => {
                Primitive::U8(l | if r == &true { 1 } else { 0 })
            }
            (Primitive::U8(l), Primitive::Int(r)) => {
                Primitive::Int(*l as i128 | r)
            }
            (Primitive::U8(l), Primitive::I8(r)) => {
                Primitive::Int(*l as i128 | *r as i128)
            }
            (Primitive::I8(l), Primitive::I8(r)) => Primitive::I8(l | r),
            (Primitive::I8(l), Primitive::Int(r)) => {
                Primitive::Int(*l as i128 | r)
            }
            (Primitive::I8(l), Primitive::U8(r)) => {
                Primitive::Int(*l as i128 | *r as i128)
            }
            (Primitive::I8(l), Primitive::Bool(r)) => {
                Primitive::I8(l | if r == &true { 1 } else { 0 })
            }
            (Primitive::Int(l), Primitive::U8(r)) => {
                Primitive::Int(l | *r as i128)
            }
            (Primitive::Int(l), Primitive::I8(r)) => {
                Primitive::Int(l | *r as i128)
            }
            (Primitive::Int(l), Primitive::Int(r)) => Primitive::Int(l | *r),
            (Primitive::Int(l), Primitive::Bool(r)) => {
                Primitive::Int(l | if r == &true { 1 } else { 0 })
            }
            _ => Primitive::Error(format!(
                "illegal call to 'bitwise_or' => left: {self} right: {rhs}"
            )),
        }
    }

    fn bitwise_xor(&self, rhs: &Self) -> Self {
        if let (&Primitive::Ref(l), &Primitive::Ref(r)) = (&self, &rhs) {
            let l = l.read().expect("B_XOR L ERROR: could not acquire lock!");
            let r = r.read().expect("B_XOR R ERROR: could not acquire lock!");
            return l.bitwise_xor(&r);
        } else if let &Primitive::Ref(l) = &self {
            let l =
                l.read().expect("B_XOR SELF ERROR: could not acquire lock!");
            return l.bitwise_xor(rhs);
        } else if let &Primitive::Ref(r) = &rhs {
            let r = r.read().expect("B_XOR RHS ERROR: could not acquire lock!");
            return self.bitwise_xor(&r);
        }

        match (self, rhs) {
            (Primitive::U8(l), Primitive::U8(r)) => Primitive::U8(l ^ r),
            (Primitive::U8(l), Primitive::Bool(r)) => {
                Primitive::U8(l ^ if r == &true { 1 } else { 0 })
            }
            (Primitive::U8(l), Primitive::Int(r)) => {
                Primitive::Int(*l as i128 ^ r)
            }
            (Primitive::U8(l), Primitive::I8(r)) => {
                Primitive::Int(*l as i128 ^ *r as i128)
            }
            (Primitive::I8(l), Primitive::I8(r)) => Primitive::I8(l ^ r),
            (Primitive::I8(l), Primitive::Int(r)) => {
                Primitive::Int(*l as i128 ^ r)
            }
            (Primitive::I8(l), Primitive::U8(r)) => {
                Primitive::Int(*l as i128 ^ *r as i128)
            }
            (Primitive::I8(l), Primitive::Bool(r)) => {
                Primitive::I8(l ^ if r == &true { 1 } else { 0 })
            }
            (Primitive::Int(l), Primitive::U8(r)) => {
                Primitive::Int(l ^ *r as i128)
            }
            (Primitive::Int(l), Primitive::I8(r)) => {
                Primitive::Int(l ^ *r as i128)
            }
            (Primitive::Int(l), Primitive::Int(r)) => Primitive::Int(l ^ *r),
            (Primitive::Int(l), Primitive::Bool(r)) => {
                Primitive::Int(l ^ if r == &true { 1 } else { 0 })
            }
            _ => Primitive::Error(format!(
                "illegal call to 'bitwise_xor' => left: {self} right: {rhs}"
            )),
        }
    }
}
impl And for Primitive {
    fn and(&self, rhs: &Self) -> Self {
        if let (&Primitive::Ref(l), &Primitive::Ref(r)) = (&self, &rhs) {
            let l = l.read().expect("AND L ERROR: could not acquire lock!");
            let r = r.read().expect("AND R ERROR: could not acquire lock!");
            return l.and(&r);
        } else if let &Primitive::Ref(l) = &self {
            let l = l.read().expect("AND SELF ERROR: could not acquire lock!");
            return l.and(rhs);
        } else if let &Primitive::Ref(r) = &rhs {
            let r = r.read().expect("AND RHS ERROR: could not acquire lock!");
            return self.and(&r);
        }
        if let &Primitive::Bool(false) = &self {
            return Primitive::Bool(false);
        }

        if !matches!((self, &rhs), (Primitive::Bool(_), Primitive::Bool(_))) {
            return Primitive::Error(format!(
                "illegal call to 'and' => left: {self} right: {rhs}"
            ));
        }

        rhs.clone()
    }
    fn bitwise_and(&self, rhs: &Self) -> Self {
        if let (&Primitive::Ref(l), &Primitive::Ref(r)) = (&self, &rhs) {
            let l = l.read().expect("B_AND L ERROR: could not acquire lock!");
            let r = r.read().expect("B_AND R ERROR: could not acquire lock!");
            return l.bitwise_and(&r);
        } else if let &Primitive::Ref(l) = &self {
            let l =
                l.read().expect("B_AND SELF ERROR: could not acquire lock!");
            return l.bitwise_and(rhs);
        } else if let &Primitive::Ref(r) = &rhs {
            let r = r.read().expect("B_AND RHS ERROR: could not acquire lock!");
            return self.bitwise_and(&r);
        }

        match (self, rhs) {
            (Primitive::U8(l), Primitive::U8(r)) => Primitive::U8(l & r),
            (Primitive::U8(l), Primitive::Bool(r)) => {
                Primitive::U8(l & if r == &true { 1 } else { 0 })
            }
            (Primitive::U8(l), Primitive::Int(r)) => {
                Primitive::Int(*l as i128 & r)
            }
            (Primitive::U8(l), Primitive::I8(r)) => {
                Primitive::Int(*l as i128 & *r as i128)
            }
            (Primitive::I8(l), Primitive::I8(r)) => Primitive::I8(l & r),
            (Primitive::I8(l), Primitive::Int(r)) => {
                Primitive::Int(*l as i128 & r)
            }
            (Primitive::I8(l), Primitive::U8(r)) => {
                Primitive::Int(*l as i128 & *r as i128)
            }
            (Primitive::I8(l), Primitive::Bool(r)) => {
                Primitive::I8(l & if r == &true { 1 } else { 0 })
            }
            (Primitive::Int(l), Primitive::U8(r)) => {
                Primitive::Int(l & *r as i128)
            }
            (Primitive::Int(l), Primitive::I8(r)) => {
                Primitive::Int(l & *r as i128)
            }
            (Primitive::Int(l), Primitive::Int(r)) => Primitive::Int(l & *r),
            (Primitive::Int(l), Primitive::Bool(r)) => {
                Primitive::Int(l & if r == &true { 1 } else { 0 })
            }
            _ => Primitive::Error(format!(
                "illegal call to 'bitwise_and' => left: {self} right: {rhs}"
            )),
        }
    }
}

impl PartialOrd for Primitive {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Primitive::Ref(l), Primitive::Ref(r)) => {
                if Arc::ptr_eq(l, r) {
                    return Some(Ordering::Equal);
                }
                let l = l
                    .read()
                    .expect("PARTIAL_CMP L ERORR: could not acquire lock!");

                let r = r
                    .read()
                    .expect("PARTIAL_CMP R ERORR: could not acquire lock!");
                l.partial_cmp(&r)
            }
            (Primitive::Ref(l), r) => {
                let l = l
                    .read()
                    .expect("PARTIAL_CMP L ERORR: could not acquire lock!");

                l.partial_cmp(r)
            }
            (l, Primitive::Ref(r)) => {
                let r = r
                    .read()
                    .expect("PARTIAL_CMP R ERORR: could not acquire lock!");

                l.partial_cmp(&r)
            }

            (Primitive::U8(l), Primitive::U8(r)) => l.partial_cmp(r),
            (Primitive::U8(l), Primitive::I8(r)) => (*l as i8).partial_cmp(r),
            (Primitive::U8(l), Primitive::Int(r)) => {
                (*l as i128).partial_cmp(r)
            }
            (Primitive::U8(l), Primitive::Double(r)) => {
                (*l as f64).partial_cmp(r)
            }

            (Primitive::I8(l), Primitive::U8(r)) => l.partial_cmp(&(*r as i8)),
            (Primitive::I8(l), Primitive::I8(r)) => (*l).partial_cmp(r),
            (Primitive::I8(l), Primitive::Int(r)) => {
                (*l as i128).partial_cmp(r)
            }
            (Primitive::I8(l), Primitive::Double(r)) => {
                (*l as f64).partial_cmp(r)
            }

            (Primitive::Int(l), Primitive::Int(r)) => l.partial_cmp(r),
            (Primitive::Int(l), Primitive::U8(r)) => {
                l.partial_cmp(&(*r as i128))
            }
            (Primitive::Int(l), Primitive::I8(r)) => {
                l.partial_cmp(&(*r as i128))
            }
            (Primitive::Int(l), Primitive::Double(r)) => {
                (*l as f64).partial_cmp(r)
            }

            (Primitive::Double(l), Primitive::Int(r)) => {
                l.partial_cmp(&(*r as f64))
            }
            (Primitive::Double(l), Primitive::U8(r)) => {
                l.partial_cmp(&(*r as f64))
            }
            (Primitive::Double(l), Primitive::I8(r)) => {
                l.partial_cmp(&(*r as f64))
            }
            (Primitive::Double(l), Primitive::Double(r)) => l.partial_cmp(r),

            (Primitive::Bool(a), Primitive::Bool(b)) => a.partial_cmp(b),
            (l @ Primitive::Bool(_), r) => l.partial_cmp(&(r.to_bool())),

            (Primitive::String(l), Primitive::String(r)) => l.partial_cmp(r),
            (Primitive::Unit, Primitive::Unit) => Some(Ordering::Equal),
            (Primitive::Array(l), Primitive::Array(r)) => l.partial_cmp(r),
            (
                Primitive::Function { parameters: pl, exprs: el },
                Primitive::Function { parameters: pr, exprs: er },
            ) => {
                if pl.eq(pr)
                    && el.iter().zip(er.iter()).filter(|&(a, b)| a != b).count()
                        == 0
                {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
            (Primitive::Error(l), Primitive::Error(r)) => l.partial_cmp(r),
            (Primitive::Null, Primitive::Null) => Some(Ordering::Equal),
            (Primitive::EarlyReturn(l), Primitive::EarlyReturn(r)) => {
                l.partial_cmp(r)
            }
            (Primitive::EarlyReturn(l), a) => l.as_ref().partial_cmp(a),
            (l, Primitive::EarlyReturn(r)) => l.partial_cmp(r),
            (Primitive::Struct(l), Primitive::Struct(r)) => {
                if l.eq(r) {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
            (Primitive::NativeLibrary { .. }, _)
            | (_, Primitive::NativeLibrary { .. }) => None,
            (Primitive::NativeFunction(_, _), _)
            | (_, Primitive::NativeFunction(_, _)) => None,
            (Primitive::Struct(_), _) => None,
            (Primitive::Int(_), _) => None,
            (Primitive::U8(_), _) => None,
            (Primitive::I8(_), _) => None,
            (Primitive::Double(_), _) => None,
            (Primitive::String(_), _) => None,
            (Primitive::NoReturn, _) => None,
            (Primitive::Null, _) => None,
            (Primitive::Array(_), _) => None,
            (Primitive::Error(_), _) => None,
            (Primitive::Unit, _) => None,
            (Primitive::Function { parameters: _, exprs: _ }, _) => None,
        }
    }
}

impl TypeOf for Primitive {
    fn type_of_str(&self) -> &'static str {
        match self {
            Primitive::Ref(l) => {
                let l =
                    l.read().expect("TYPE_OF ERROR: could not acquire lock!");
                l.type_of_str()
            }
            Primitive::U8(_) => TYPE_U8,
            Primitive::I8(_) => TYPE_I8,
            Primitive::Int(_) => TYPE_INT,
            Primitive::Bool(_) => TYPE_BOOL,
            Primitive::Null => TYPE_NULL,
            Primitive::Double(_) => TYPE_DOUBLE,
            Primitive::String(_) => TYPE_STRING,
            Primitive::Array(_) => TYPE_ARRAY,
            Primitive::Error(_) => TYPE_ERROR,
            Primitive::NativeLibrary(_) => TYPE_NATIVE_LIB,
            Primitive::Function { .. } | Primitive::NativeFunction(_, _) => {
                TYPE_FUNCTION
            }
            Primitive::Struct(_) => TYPE_STRUCT,
            Primitive::Unit => TYPE_UNIT,
            Primitive::NoReturn => TYPE_NO_RETURN,
            Primitive::EarlyReturn(v) => v.type_of_str(),
        }
    }

    fn type_of(&self) -> Self {
        Primitive::String(self.type_of_str().to_string())
    }
}

impl Array for Primitive {
    fn index_at(&self, rhs: &Primitive) -> Primitive {
        match (self, rhs) {
            (Primitive::Ref(l), Primitive::Ref(r)) => {
                let l = l
                    .read()
                    .expect("INDEX_AT L ERORR: could not acquire lock!");

                let r = r
                    .read()
                    .expect("INDEX_AT R ERORR: could not acquire lock!");
                l.index_at(&r)
            }
            (Primitive::Ref(l), r) => {
                let l = l
                    .read()
                    .expect("INDEX_AT L ERORR: could not acquire lock!");

                l.index_at(r)
            }
            (l, Primitive::Ref(r)) => {
                let r = r
                    .read()
                    .expect("INDEX_AT R ERORR: could not acquire lock!");

                l.index_at(&r)
            }
            (Primitive::Array(arr), Primitive::Int(idx)) if idx >= &0 => {
                let idx = *idx as usize;
                if idx < arr.len() {
                    arr[idx].clone()
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::Array(arr), Primitive::U8(idx)) => {
                let idx = *idx as usize;
                if idx < arr.len() {
                    arr[idx].clone()
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::Array(arr), Primitive::I8(idx)) if idx >= &0 => {
                let idx = *idx as usize;
                if idx < arr.len() {
                    arr[idx].clone()
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::String(s), Primitive::Int(idx)) if idx >= &0 => {
                let idx = *idx as usize;
                if idx < s.len() {
                    let s: String = s.chars().skip(idx).take(1).collect();
                    Primitive::String(s)
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::String(s), Primitive::U8(idx)) => {
                let idx = *idx as usize;
                if idx < s.len() {
                    let s: String = s.chars().skip(idx).take(1).collect();
                    Primitive::String(s)
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::String(s), Primitive::I8(idx)) if idx >= &0 => {
                let idx = *idx as usize;
                if idx < s.len() {
                    let s: String = s.chars().skip(idx).take(1).collect();
                    Primitive::String(s)
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::Struct(struc), Primitive::String(key)) => {
                if let Some(p) = struc.get(key) {
                    p.clone()
                } else {
                    Primitive::Null
                }
            }
            (key, value) => Primitive::Error(format!(
                "illegal access to array!!!  {key} => {value}"
            )),
        }
    }

    fn len(&self) -> Primitive {
        match self {
            Primitive::Ref(l) => {
                let l = l.read().expect("LEN ERROR: could not acquire lock!");
                l.len()
            }
            Primitive::String(s) => Primitive::Int(s.len() as i128),
            Primitive::Array(a) => Primitive::Int(a.len() as i128),
            Primitive::Struct(s) => Primitive::Int(s.len() as i128),
            _ => Primitive::Error(format!(
                "call to len() on a non array value => {self}"
            )),
        }
    }

    fn swap_mem(
        &mut self,
        rhs: &mut Primitive,
        index: &Primitive,
    ) -> Primitive {
        match (self, index) {
            // FIXME this might be more complex than that
            // RHS could be equal to self
            // using Arc::ptr_eq(&arc1, &arc2) might be safer
            (Primitive::Ref(l), Primitive::Ref(r)) => {
                let mut l = l
                    .write()
                    .expect("SWAP_MEM L ERORR: could not acquire lock!");

                let r = r
                    .read()
                    .expect("SWAP_MEM R ERORR: could not acquire lock!");
                l.swap_mem(rhs, &r)
            }
            (Primitive::Ref(l), _) => {
                let mut l = l
                    .write()
                    .expect("SWAP_MEM L ERORR: could not acquire lock!");

                l.swap_mem(rhs, index)
            }
            (l, Primitive::Ref(index)) => {
                let index = index
                    .read()
                    .expect("SWAP_MEM R ERORR: could not acquire lock!");

                l.swap_mem(rhs, &index)
            }
            (Primitive::Array(arr), Primitive::Int(idx)) if idx >= &0 => {
                let idx = *idx as usize;
                if !matches!(rhs, Primitive::Error(_) | Primitive::Unit)
                    && idx < arr.len()
                {
                    std::mem::swap(&mut arr[idx], rhs);
                    arr[idx].clone()
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }

            (Primitive::Array(arr), Primitive::U8(idx)) => {
                let idx = *idx as usize;
                if !matches!(rhs, Primitive::Error(_) | Primitive::Unit)
                    && idx < arr.len()
                {
                    std::mem::swap(&mut arr[idx], rhs);
                    arr[idx].clone()
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::Array(arr), Primitive::I8(idx)) if idx >= &0 => {
                let idx = *idx as usize;
                if !matches!(rhs, Primitive::Error(_) | Primitive::Unit)
                    && idx < arr.len()
                {
                    std::mem::swap(&mut arr[idx], rhs);
                    arr[idx].clone()
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::Struct(s), Primitive::String(k)) => {
                if s.contains_key(k) {
                    std::mem::swap(s.get_mut(k).unwrap(), rhs);
                } else {
                    s.insert(k.clone(), rhs.clone());
                }
                s[k].clone()
            }
            (Primitive::String(s), Primitive::Int(idx)) if idx >= &0 => {
                let idx = *idx as usize;
                if !matches!(rhs, Primitive::Error(_) | Primitive::Unit)
                    && idx < s.len()
                {
                    s.remove(idx);
                    s.insert_str(idx, &rhs.to_string());
                    rhs.clone()
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::String(s), Primitive::I8(idx)) if idx >= &0 => {
                let idx = *idx as usize;
                if !matches!(rhs, Primitive::Error(_) | Primitive::Unit)
                    && idx < s.len()
                {
                    s.remove(idx);
                    s.insert_str(idx, &rhs.to_string());
                    rhs.clone()
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::String(s), Primitive::U8(idx)) => {
                let idx = *idx as usize;
                if !matches!(rhs, Primitive::Error(_) | Primitive::Unit)
                    && idx < s.len()
                {
                    s.remove(idx);
                    s.insert_str(idx, &rhs.to_string());
                    rhs.clone()
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            _ => Primitive::Error("invalid call to swap_mem()".to_string()),
        }
    }

    fn remove(&mut self, key: &Primitive) -> anyhow::Result<()> {
        match (self, key) {
            // FIXME this might be more complex than that
            // RHS could be equal to self
            // using Arc::ptr_eq(&arc1, &arc2) might be safer
            (Primitive::Ref(l), Primitive::Ref(r)) => {
                let mut l =
                    l.write().expect("REMOVE L ERORR: could not acquire lock!");

                let r =
                    r.read().expect("REMOVE R ERORR: could not acquire lock!");
                l.remove(&r)
            }
            (Primitive::Ref(l), _) => {
                let mut l =
                    l.write().expect("REMOVE L ERORR: could not acquire lock!");

                l.remove(key)
            }
            (l, Primitive::Ref(index)) => {
                let index = index
                    .read()
                    .expect("REMOVE R ERORR: could not acquire lock!");

                l.remove(&index)
            }
            (Primitive::Array(arr), Primitive::Int(idx)) if idx >= &0 => {
                let idx = *idx as usize;
                if idx < arr.len() {
                    arr.remove(idx);
                    Ok(())
                } else {
                    Err(anyhow::Error::msg("index out of range"))
                }
            }
            (Primitive::Array(arr), Primitive::U8(idx)) => {
                let idx = *idx as usize;
                if idx < arr.len() {
                    arr.remove(idx);
                    Ok(())
                } else {
                    Err(anyhow::Error::msg("index out of range"))
                }
            }
            (Primitive::Array(arr), Primitive::I8(idx)) if idx >= &0 => {
                let idx = *idx as usize;
                if idx < arr.len() {
                    arr.remove(idx);
                    Ok(())
                } else {
                    Err(anyhow::Error::msg("index out of range"))
                }
            }

            (Primitive::String(s), Primitive::U8(idx)) => {
                let idx = *idx as usize;
                if idx < s.len() {
                    s.remove(idx);
                    Ok(())
                } else {
                    Err(anyhow::Error::msg("index out of range"))
                }
            }
            (Primitive::String(s), Primitive::I8(idx)) if idx >= &0 => {
                let idx = *idx as usize;
                if idx < s.len() {
                    s.remove(idx);
                    Ok(())
                } else {
                    Err(anyhow::Error::msg("index out of range"))
                }
            }
            (Primitive::String(s), Primitive::Int(idx)) if idx >= &0 => {
                let idx = *idx as usize;
                if idx < s.len() {
                    s.remove(idx);
                    Ok(())
                } else {
                    Err(anyhow::Error::msg("index out of range"))
                }
            }
            (Primitive::Struct(struc), Primitive::String(key)) => {
                if let Some(_p) = struc.remove(key) {
                    Ok(())
                } else {
                    Err(anyhow::Error::msg("key doesn't exist"))
                }
            }
            _ => Err(anyhow::Error::msg("illegal access to array!!!")),
        }
    }

    fn is_empty(&self) -> Primitive {
        match self.len() {
            Primitive::Int(n) => Primitive::Bool(n == 0),
            e => Primitive::Error(format!("err: {e}")),
        }
    }
}
impl PartialEq for Primitive {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ref(l0), Self::Ref(r0)) => {
                if Arc::ptr_eq(l0, r0) {
                    return true;
                }

                let l0 =
                    l0.read().expect("EQ L ERORR: could not acquire lock!");

                let r = r0.read().expect("EQ R ERORR: could not acquire lock!");
                l0.eq(&r)
            }
            (Primitive::Ref(l), _) => {
                let l = l.read().expect("EQ L ERORR: could not acquire lock!");
                l.eq(other)
            }
            (_, Primitive::Ref(r)) => {
                let r = r.read().expect("EQ R ERORR: could not acquire lock!");
                self.eq(&r)
            }
            (Self::U8(l0), Self::U8(r0)) => l0 == r0,
            (Self::I8(l0), Self::I8(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Double(l0), Self::Double(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            (Self::Struct(l0), Self::Struct(r0)) => l0 == r0,
            (Self::Error(l0), Self::Error(r0)) => l0 == r0,
            (
                Self::Function { parameters: l_parameters, exprs: l_exprs },
                Self::Function { parameters: r_parameters, exprs: r_exprs },
            ) => l_parameters == r_parameters && l_exprs == r_exprs,
            (Self::EarlyReturn(l0), Self::EarlyReturn(r0)) => l0 == r0,
            _ => {
                core::mem::discriminant(self) == core::mem::discriminant(other)
            }
        }
    }
}
impl DisplayHex for Primitive {
    fn to_hex(&self) -> Self {
        match self {
            Primitive::U8(u) => Primitive::String(format!("{u:#x}")),
            Primitive::I8(u) => Primitive::String(format!("{u:#x}")),
            Primitive::Int(u) => Primitive::String(format!("{u:#x}")),

            Primitive::Ref(l0) => {
                let l0 =
                    l0.read().expect("EQ L ERORR: could not acquire lock!");
                l0.to_hex()
            }
            Primitive::Double(d) => {
                let bytes = d.to_ne_bytes();
                let u = i64::from_ne_bytes(bytes);
                Primitive::String(format!("{u:#x}"))
            }

            e => Primitive::Error(format!("could not convert to_hex: {e}")),
        }
    }
}
impl DisplayBinary for Primitive {
    fn to_binary(&self) -> Self {
        match self {
            Primitive::U8(u) => Primitive::String(format!("{u:#b}")),
            Primitive::I8(u) => Primitive::String(format!("{u:#b}")),
            Primitive::Int(u) => Primitive::String(format!("{u:#b}")),

            Primitive::Ref(l0) => {
                let l0 =
                    l0.read().expect("EQ L ERORR: could not acquire lock!");
                l0.to_hex()
            }
            Primitive::Double(d) => {
                let bytes = d.to_ne_bytes();
                let u = i64::from_ne_bytes(bytes);
                Primitive::String(format!("{u:#b}"))
            }

            e => Primitive::Error(format!("could not convert to_binary: {e}")),
        }
    }
}
// endregion

#[cfg(test)]
mod test {
    use super::Add;

    use super::Primitive;

    #[test]
    fn test_add_valid() {
        let l = Primitive::Int(1);
        let r = Primitive::Int(2);
        assert_eq!(l.add(&r), Primitive::Int(3));

        let l = Primitive::Int(1);
        let r = Primitive::Double(2.);
        assert_eq!(l.add(&r), Primitive::Double(3.));

        let l = Primitive::Double(1.);
        let r = Primitive::Int(2);
        assert_eq!(l.add(&r), Primitive::Double(3.));

        let l = Primitive::Double(1.);
        let r = Primitive::Double(2.);
        assert_eq!(l.add(&r), Primitive::Double(3.));
    }
}
