use crate::os::OS;
use minijinja::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Syscall {
    pub name: String,
    pub desc: Option<String>,
}

impl Syscall {
    pub(crate) fn transform(&self, oses: &[Box<dyn OS>]) -> HashMap<&'static str, Value> {
        let emojis: Vec<String> = oses
            .iter()
            .filter(|os| os.syscalls().contains(&self.name))
            .map(|os| os.emoji())
            .collect();

        let mut result = HashMap::from([
            ("name", Value::from(self.name.to_string())),
            ("emojis", Value::from(emojis.join(", "))),
        ]);

        if let Some(desc) = &self.desc {
            result.insert("desc", Value::from(desc.to_string()));
        }

        result
    }
}
