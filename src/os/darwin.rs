use crate::os::{fetch_url, OS};
use crate::Result;

pub struct Darwin {
    syscalls: Vec<String>,
}

impl Darwin {
    pub fn new() -> Result<Self> {
        let syscall_header = fetch_url("https://raw.githubusercontent.com/alexey-lysiuk/macos-sdk/main/MacOSX14.4.sdk/usr/include/sys/syscall.h")?;

        Ok(Darwin {
            syscalls: Self::extract_syscalls(syscall_header)?,
        })
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
