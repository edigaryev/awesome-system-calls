use crate::os::{fetch_url, OS};
use crate::Result;

pub struct Linux {
    syscalls: Vec<String>,
}

impl Linux {
    pub fn new() -> Result<Self> {
        let syscall_header = fetch_url("https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/include/uapi/asm-generic/unistd.h")?;

        Ok(Linux {
            syscalls: Self::extract_syscalls(syscall_header)?,
        })
    }

    fn extract_syscalls(syscall_header: String) -> Result<Vec<String>> {
        let mut result: Vec<String> = Vec::new();

        for cmacro in cmacros::extract_macros(syscall_header.as_str())? {
            // Linux system call definitions start with "__NR_" prefix
            let name = if let Some(name_without_prefix) = cmacro.name.strip_prefix("__NR_") {
                name_without_prefix
            } else {
                continue;
            };

            // Filter out guard value
            if name == "syscalls" {
                continue;
            }

            result.push(name.to_string());
        }

        Ok(result)
    }
}

impl OS for Linux {
    fn name(&self) -> String {
        "Linux".into()
    }

    fn emoji(&self) -> String {
        "🐧".into()
    }

    fn syscalls(&self) -> Vec<String> {
        self.syscalls.clone()
    }

    fn manpage_link(&self, syscall_name: &str) -> String {
        format!(
            "https://man7.org/linux/man-pages/man2/{}.2.html",
            syscall_name
        )
    }
}
