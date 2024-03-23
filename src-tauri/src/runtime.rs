use once_cell::sync::Lazy;
use tokio::runtime::Handle;

pub static RUNTIME_HANDLE: Lazy<Handle> = Lazy::new(|| {
    tokio::runtime::Handle::current()
});