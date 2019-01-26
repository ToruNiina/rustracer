#[cfg(target_feature = "sse")]
pub mod x86_64_sse;

#[cfg(target_feature = "sse")]
pub use self::x86_64_sse as vector;

#[cfg(not(target_feature = "sse"))]
pub mod fallback;

#[cfg(not(target_feature = "sse"))]
pub use self::fallback as vector;
