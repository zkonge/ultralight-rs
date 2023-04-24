mod string;

pub mod buffer;
pub mod config;
pub mod filesystem;
pub mod logger;
pub mod platform;
pub mod renderer;
pub mod session;
pub mod surface;
pub mod view;

pub trait AsULRawPtr<P> {
    fn as_raw_ptr(&self) -> P;
}
