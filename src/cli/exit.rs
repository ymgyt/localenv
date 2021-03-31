/// Common terminate hook.
pub(crate) fn exit(prefer_code: Option<i32>) {
    let code = prefer_code.unwrap_or(1);

    std::process::exit(code);
}
