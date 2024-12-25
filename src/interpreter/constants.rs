use std::sync::{Arc, LazyLock};

use crate::{ast::Node, lexer::Token};

use super::{objects::Object, EnvReference, Reference};

pub static NULL: LazyLock<Reference> = LazyLock::new(|| Reference::new(Object::Null));
pub static TRUE: LazyLock<Reference> = LazyLock::new(|| Reference::new(Object::Bool(true)));
pub static FALSE: LazyLock<Reference> = LazyLock::new(|| Reference::new(Object::Bool(false)));
pub static NUMBER: LazyLock<Reference> = LazyLock::new(|| Reference::new(Object::Integer(0)));
pub static STRING: LazyLock<Reference> =
    LazyLock::new(|| Reference::new(Object::String(String::new().into())));
pub static LIST: LazyLock<Reference> = LazyLock::new(|| Reference::new(Object::List(Arc::new([]))));
pub static FUNCTION: LazyLock<Reference> = LazyLock::new(|| {
    Reference::new(Object::Function {
        body: Node::Invalid(Token::default()),
        parameters: Arc::default(),
        env: EnvReference::default(),
    })
});

pub static ALL_TYPES: LazyLock<[Reference; 6]> = LazyLock::new(|| {
    [
        NULL.clone(),
        TRUE.clone(),
        NUMBER.clone(),
        STRING.clone(),
        LIST.clone(),
        FUNCTION.clone(),
    ]
});
