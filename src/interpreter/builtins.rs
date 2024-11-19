use std::rc::Rc;

use super::{objects::Object, Env};

pub fn add_builtins(env: &mut Env) {
    env.insert(
        "+".to_string(),
        Rc::new(Object::Builtin(|args| {
            let [f, s] = &args[..2] else {
                return Object::Error(format!("Invalid args len {} expected:2", args.len()));
            };

            match (f, s) {
                (Object::Integer(l), Object::Integer(r)) => return Object::Integer(l + r),
                _ => Object::Error(format!(
                    "Invalid args type to function should be both integers",
                )),
            }
        })),
    );
}
