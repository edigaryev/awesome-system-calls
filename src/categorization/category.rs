use crate::categorization::syscall::Syscall;
use crate::categorization::Categories;
use crate::os::OS;
use inflector::Inflector;
use minijinja::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub name: Option<String>,
    pub desc: Option<String>,
    pub syscalls: Option<Vec<Syscall>>,
    pub categories: Option<Categories>,
}

impl Category {
    pub(crate) fn transform(
        &self,
        name: &str,
        oses: &[Box<dyn OS>],
    ) -> HashMap<&'static str, Value> {
        if self.categories.is_none() && self.syscalls.is_none() {
            panic!(
                "category {:?} is empty, please either add \"categories:\" or \"syscalls:\"",
                name
            )
        }

        if self.categories.is_some() && self.syscalls.is_some() {
            panic!(
                "category {:?} has both \"categories:\" and \"syscalls:\", please only use one",
                name
            )
        }

        let final_name = self.name.clone().unwrap_or(name.to_sentence_case());

        let mut transformed = HashMap::from([
            ("name", Value::from_serializable(&final_name)),
            ("desc", Value::from_serializable(&self.desc)),
            (
                "slug",
                Value::from_serializable(&github_slugger::slug(&final_name)),
            ),
        ]);

        if let Some(syscalls) = &self.syscalls {
            let modified_syscalls: Vec<HashMap<&str, Value>> = syscalls
                .iter()
                .flat_map(|syscall| syscall.transform(oses))
                .collect();

            transformed.insert("syscalls", Value::from_serializable(&modified_syscalls));
        }

        if let Some(categories) = &self.categories {
            let modified_categories: Vec<HashMap<&str, Value>> = categories
                .iter()
                .map(|(name, category)| category.transform(name, oses))
                .collect();

            transformed.insert("categories", Value::from_serializable(&modified_categories));
        }

        transformed
    }

    pub(crate) fn syscalls(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        if let Some(syscalls) = &self.syscalls {
            for syscall in syscalls {
                result.push(syscall.name.clone());

                if let Some(aliases) = &syscall.aliases {
                    result.extend(aliases.iter().cloned());
                }
            }
        }

        if let Some(categories) = &self.categories {
            for (_, category) in categories {
                result.extend(category.syscalls());
            }
        }

        result
    }
}
