use crate::os::{fetch_url, OS};
use crate::Result;
use regex::Regex;

pub struct FreeBSD {
    syscalls: Vec<String>,
}

impl FreeBSD {
    pub fn new() -> Result<Self> {
        let syscall_header = fetch_url("https://raw.githubusercontent.com/freebsd/freebsd-src/refs/heads/main/sys/sys/syscall.h")?;

        Ok(FreeBSD {
            syscalls: Self::extract_syscalls(syscall_header)?,
        })
    }

    fn extract_syscalls(syscall_header: String) -> Result<Vec<String>> {
        let compat_re = Regex::new("^freebsd[0-9]+_.*$")?;
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

            // Exclude forward/backward compatibility system calls (e.g. freebsd11_mknod)
            if compat_re.is_match(name) {
                continue;
            }

            result.push(name.to_string());
        }

        Ok(result)
    }
}

impl OS for FreeBSD {
    fn name(&self) -> String {
        "FreeBSD".into()
    }

    fn emoji(&self) -> String {
        "😈".into()
    }

    fn syscalls(&self) -> Vec<String> {
        self.syscalls.clone()
    }

    fn manpage_link(&self, syscall_name: &str) -> String {
        format!(
            "https://www.freebsd.org/cgi/man.cgi?query={}&sektion=2",
            syscall_name
        )
    }
}
