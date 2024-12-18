use std::sync::{Arc, LazyLock};

use super::{objects::Object, Reference};

pub static NULL: LazyLock<Reference> = LazyLock::new(|| Reference::new(Object::Null));
pub static TRUE: LazyLock<Reference> = LazyLock::new(|| Reference::new(Object::Bool(true)));
pub static FALSE: LazyLock<Reference> = LazyLock::new(|| Reference::new(Object::Bool(false)));
pub static NUMBER: LazyLock<Reference> = LazyLock::new(|| Reference::new(Object::Integer(0)));
pub static STRING: LazyLock<Reference> =
    LazyLock::new(|| Reference::new(Object::String(String::new())));
pub static LIST: LazyLock<Reference> = LazyLock::new(|| Reference::new(Object::List(Arc::new([]))));
