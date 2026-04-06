use solabi::{abi::EventDescriptor, value::Value};

/// Checks whether a new `prepare_event` call is consistent with an already-prepared event.
///
/// Returns `true` if the event already exists with the same descriptor (caller should early-return
/// `Ok(())`), `false` if the event does not yet exist, or an error if it exists with a different
/// signature.
pub fn ensure_event_consistent(
    existing: Option<&EventDescriptor>,
    name: &str,
    event: &EventDescriptor,
) -> anyhow::Result<bool> {
    if let Some(existing_descriptor) = existing {
        if event != existing_descriptor {
            return Err(anyhow::anyhow!(
                "event {} (database name {name}) already exists with different signature",
                event.name
            ));
        }
        return Ok(true);
    }
    Ok(false)
}

/// Validates that `fields` match the descriptor's input types.
pub fn validate_log_fields(descriptor: &EventDescriptor, fields: &[Value]) -> anyhow::Result<()> {
    let expected_len = descriptor.inputs.len();
    let len = fields.len();
    if len != expected_len {
        return Err(anyhow::anyhow!(
            "event value has {len} fields but should have {expected_len}"
        ));
    }
    for (i, (value, kind)) in fields.iter().zip(&descriptor.inputs).enumerate() {
        if value.kind() != kind.field.kind {
            return Err(anyhow::anyhow!(
                "event field {i} doesn't match event descriptor"
            ));
        }
    }
    Ok(())
}

/// Pushes `value` into the current active table in `sql_values`.
/// When `in_array` is true, appends to the last (innermost array) table; otherwise to the first.
pub fn push_sql_value<T>(sql_values: &mut [(Option<usize>, Vec<T>)], in_array: bool, value: T) {
    (if in_array {
        <[_]>::last_mut
    } else {
        <[_]>::first_mut
    })(sql_values)
    .unwrap()
    .1
    .push(value);
}
