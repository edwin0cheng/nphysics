pub use object::private::rigid_body::{RigidBody, Static, Dynamic}; // FIXME: rename to StaticBody, DynamicBody ?
pub use object::private::soft_body::SoftBody;
pub use object::private::body::{Body, RB, SB};

mod private {
    #[path = "../rigid_body.rs"]
    mod rigid_body;

    #[path = "../soft_body.rs"]
    mod soft_body;

    #[path = "../body.rs"]
    mod body;
}

pub mod volumetric;
