// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for creating custom types.

pub mod renderer;

/// Traits intended for blanket imports.
pub mod prelude {
    #[doc(hidden)]
    pub use glib::subclass::prelude::*;

    pub use super::renderer::{RendererImpl, RendererImplExt};
}
