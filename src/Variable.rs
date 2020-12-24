#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    Boolean(bool),
    Float(f64),
    Integer(i64),
    String(String),
}

macro_rules! impl_eq {
    ($dest:ident,$src:ty) => {
        impl PartialEq<$src> for Variable {
            fn eq(&self, other: &$src) -> bool {
                if let Self::$dest(item) = self {
                    item == other
                } else {
                    panic!("eq not implemented")
                }
            }
        }
    };
}

impl_eq!(Boolean, bool);
impl_eq!(Float, f64);
impl_eq!(Integer, i64);
impl_eq!(String, String);
impl_eq!(String, &'static str);

impl Default for Variable {
    fn default() -> Self {
        Self::Float(0.0)
    }
}

impl From<bool> for Variable {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<f64> for Variable {
    fn from(num: f64) -> Self {
        Self::Float(num)
    }
}

impl From<i64> for Variable {
    fn from(num: i64) -> Self {
        Self::Integer(num)
    }
}

impl From<&'static str> for Variable {
    fn from(s: &'static str) -> Self {
        Self::String(s.into())
    }
}

impl From<String> for Variable {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}
