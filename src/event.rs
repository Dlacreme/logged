
pub struct Event<'a> {
    kind: &'a str,
    at: chrono::NaiveDateTime,
    code: String,
    metadata: &'a str,
}