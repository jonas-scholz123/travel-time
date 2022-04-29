use super::param_value::ParamValue;

#[derive(Default)]
pub struct QueryParameters {
    pub params: Vec<(String, String)>,
}

impl QueryParameters {
    pub fn new() -> Self {
        QueryParameters::default()
    }

    pub fn push<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: Into<String>,
        V: ParamValue,
    {
        self.params.push((key.into(), val.as_value()));
        self
    }

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
