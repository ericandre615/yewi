use uuid::Uuid;

pub(crate) fn generate_unique_id() -> String {
    Uuid::new_v4().to_string()
}
