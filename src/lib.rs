pub mod k8s;
pub mod recipes;

pub mod prelude {
    pub use crate::k8s::container::*;
    pub use crate::k8s::deployment::*;
    pub use crate::k8s::environment::*;
    pub use crate::k8s::metadata::*;
    pub use crate::k8s::secret::*;
    pub use crate::k8s::selector::*;
    pub use crate::k8s::service::*;
    pub use crate::k8s::stateful_set::*;
    pub use crate::k8s::volume::*;
}