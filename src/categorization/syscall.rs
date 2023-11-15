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
        let per_os_infos: Vec<HashMap<&str, String>> = oses
            .iter()
            .filter(|os| os.syscalls().contains(&self.name))
            .map(|os| {
                HashMap::from([
                    ("os_emoji", os.emoji()),
                    ("manpage_link", os.manpage_link(&self.name)),
                ])
            })
            .collect();

        let mut result = HashMap::from([
            ("name", Value::from(self.name.to_string())),
            ("per_os_infos", Value::from_serializable(&per_os_infos)),
        ]);

        if let Some(desc) = &self.desc {
            result.insert("desc", Value::from(desc.to_string()));
        }

        result
    }
}
