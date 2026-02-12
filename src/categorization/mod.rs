mod category;
mod syscall;

use crate::categorization::category::Category;
use crate::os::OS;
use crate::Result;
use indexmap::{IndexMap, IndexSet};
use minijinja::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::ops::Sub;

type Categories = IndexMap<String, Category>;

pub struct Categorization {
    pub categories: Categories,
}

impl Categorization {
    pub fn load(path: &str) -> Result<Self> {
        let file = File::open(path)?;

        Ok(Categorization {
            categories: serde_yaml::from_reader(file)?,
        })
    }

    pub fn render_known_syscalls(
        &self,
        env: &minijinja::Environment,
        oses: &[Box<dyn OS>],
        template_path: &str,
        output_path: &str,
    ) -> Result<()> {
        let mut env = env.clone();

        let os_infos: Vec<HashMap<&str, String>> =
            oses.iter().map(|os| os_info(os.as_ref())).collect();
        env.add_global("os_infos", os_infos);

        let categories: Vec<HashMap<&str, Value>> = self
            .categories
            .iter()
            .map(|(name, category)| category.transform(name, oses))
            .collect();
        env.add_global("categories", Value::from_serialize(&categories));

        let template = fs::read_to_string(template_path)?;
        let rendered = minijinja::render!(in env, &template);
        fs::write(output_path, rendered).map_err(|err| err.into())
    }

    pub fn render_unknown_syscalls(
        &self,
        env: &minijinja::Environment,
        os: &dyn OS,
        template_path: &str,
        output_path: &str,
    ) -> Result<()> {
        let mut env = env.clone();

        env.add_global("os_info", os_info(os));

        let known_syscalls: IndexSet<String> = self
            .categories
            .iter()
            .flat_map(|(_, category)| category.syscalls())
            .collect();
        let unknown_syscalls: IndexSet<String> =
            IndexSet::from_iter(os.syscalls()).sub(&known_syscalls);
        env.add_global("syscalls", Value::from_serialize(&unknown_syscalls));

        let template = fs::read_to_string(template_path)?;
        let rendered = minijinja::render!(in env, &template);
        fs::write(output_path, rendered).map_err(|err| err.into())
    }
}

fn os_info(os: &dyn OS) -> HashMap<&'static str, String> {
    HashMap::from([
        ("name", os.name()),
        ("emoji", os.emoji()),
        ("count", os.syscalls().len().to_string()),
    ])
}
