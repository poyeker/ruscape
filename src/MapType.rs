use crate::Variable::Variable;
use fxhash::FxBuildHasher;
use indexmap::IndexMap;
pub(crate) type HashMap<K, V> = IndexMap<K, V, FxBuildHasher>;
pub(crate) type VariableMap = std::collections::HashMap<&'static str, Variable, FxBuildHasher>;
