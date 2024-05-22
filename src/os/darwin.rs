use crate::os::{fetch_url, OS};
use crate::Result;
use serde::Deserialize;

pub struct Darwin {
    syscalls: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RepositoryContents {
    tree: Vec<RepositoryTree>,
}

#[derive(Debug, Deserialize)]
struct RepositoryTree {
    path: String,
}

impl Darwin {
    pub fn new() -> Result<Self> {
        // Figure out the SDK directory and a URL to fetch the system calls header from
        let sdk_directory = &Self::sdk_directory()?;
        let url = format!("https://raw.githubusercontent.com/alexey-lysiuk/macos-sdk/main/{sdk_directory}/usr/include/sys/syscall.h");

        let syscall_header = fetch_url(&url)?;

        Ok(Darwin {
            syscalls: Self::extract_syscalls(syscall_header)?,
        })
    }

    fn sdk_directory() -> Result<String> {
        let repository_contents_json =
            fetch_url("https://api.github.com/repos/alexey-lysiuk/macos-sdk/git/trees/main")?;

        let repository_contents: RepositoryContents =
            serde_json::from_str(&repository_contents_json)?;

        let sdk_directory = repository_contents
            .tree
            .last()
            .ok_or("macOS SDK repository has no tree entries, weird!")?;

        Ok(sdk_directory.path.clone())
    }

    fn extract_syscalls(syscall_header: String) -> Result<Vec<String>> {
        let mut result: Vec<String> = Vec::new();

        for cmacro in cmacros::extract_macros(syscall_header.as_str())? {
            // BSD system call definitions start with "SYS_" prefix
            let name = if let Some(name_without_prefix) = cmacro.name.strip_prefix("SYS_") {
                name_without_prefix
            } else {
                continue;
            };

            // Exclude the special "MAXSYSCALL" guard value
            if name == "MAXSYSCALL" {
                continue;
            }

            result.push(name.to_string());
        }

        Ok(result)
    }
}

impl OS for Darwin {
    fn name(&self) -> String {
        "Darwin".into()
    }

    fn emoji(&self) -> String {
        "🍏".into()
    }

    fn syscalls(&self) -> Vec<String> {
        self.syscalls.clone()
    }

    fn manpage_link(&self, syscall_name: &str) -> String {
        format!("https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/{}.2.html", syscall_name)
    }
}
