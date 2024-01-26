use rquickjs::{Type, Value};

#[inline(always)]
pub fn cast_to_string(value: &Value) -> String {
    match value.type_of() {
        Type::Uninitialized => String::from("<uninitialized>"),
        Type::Undefined => String::from("<undefined>"),
        Type::Null => String::from("<null>"),
        Type::Bool => value.as_bool().unwrap().to_string(),
        Type::Int => value.as_int().unwrap().to_string(),
        Type::Float => value.as_float().unwrap().to_string(),
        Type::String => value.as_string().unwrap().to_string().unwrap(),
        Type::Symbol => format!("{:?}", value.as_symbol().unwrap()).to_string(),
        Type::Array => format!("{:?}", value.as_array().unwrap()).to_string(),
        Type::Constructor => format!("{:?}", value.as_constructor().unwrap()).to_string(),
        Type::Function => format!("{:?}", value.as_function().unwrap()).to_string(),
        Type::Exception => format!("{:?}", value.as_exception().unwrap()).to_string(),
        Type::Object => format!("{:?}", value.as_object().unwrap()).to_string(),
        Type::Module => String::from("<module>"),
        Type::BigInt => format!("{:?}", value.as_big_int().unwrap()).to_string(),
        Type::Unknown => String::from("<unknown>"),
    }
}
