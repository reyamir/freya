use std::sync::{Arc, Mutex};

pub use config::{WindowConfig, *};
use freya_native_core::NodeId;

#[cfg(target_os = "macos")]
pub use macos::renderer::DesktopRenderer;

#[cfg(not(target_os = "macos"))]
pub use macos::other::DesktopRenderer;

mod accessibility;
mod app;
mod config;
pub mod devtools;
mod winit_waker;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(not(target_os = "macos"))]
mod other;

pub type HoveredNode = Option<Arc<Mutex<Option<NodeId>>>>;
