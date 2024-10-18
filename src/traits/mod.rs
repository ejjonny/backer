mod nodetrait;
mod scopable;
mod scopable_option;

pub(crate) use nodetrait::NodeTrait;
pub use scopable::NoOpScoper;
pub use scopable::Scopable;
pub use scopable_option::ScopableOption;
