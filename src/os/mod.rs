mod darwin;
pub use darwin::Darwin;

mod freebsd;
pub use freebsd::FreeBSD;

mod linux;
pub use linux::Linux;

mod openbsd;
pub use openbsd::OpenBSD;

pub trait OS {
    fn name(&self) -> String;
    fn emoji(&self) -> String;
    fn syscalls(&self) -> Vec<String>;
    fn manpage_link(&self, syscall_name: &str) -> String;
}

fn fetch_url(url: &str) -> crate::Result<String> {
    ureq::get(url)
        .call()?
        .into_string()
        .map_err(|err| err.into())
}
