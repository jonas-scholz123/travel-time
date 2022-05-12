use serde::Serialize;

use super::param_value::ParamValue;

#[derive(Default)]
pub struct ExtraQueryParams {
    pub params: Vec<(String, String)>,
}

impl ExtraQueryParams {
    pub fn new() -> Self {
        ExtraQueryParams::default()
    }

    /*pub fn push<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: Into<String>,
        V: ParamValue,
    {
        self.params.push((key.into(), val.as_value()));
        self
    }
    */

    pub fn push_opt<K, V>(&mut self, key: K, val: Option<V>) -> &mut Self
    where
        K: Into<String>,
        V: ParamValue,
    {
        if let Some(val) = val {
            self.params.push((key.into(), val.as_value()));
        }
        self
    }
}

impl Serialize for ExtraQueryParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.params.serialize(serializer)
    }
}
