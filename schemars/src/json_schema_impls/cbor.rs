use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use serde_cbor::{ObjectKey, Value};
use std::borrow::Cow;

impl JsonSchema for Value {
    no_ref_schema!();

    fn schema_name() -> String {
        "AnyValue".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("AnyValue")
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        Schema::Bool(true)
    }
}

forward_impl!(ObjectKey => Value);
