use uuid::Uuid;

pub fn generate_id(context: &str) -> String {
    let uid = Uuid::now_v7().hyphenated();

    format!("{}-{}", context, uid)
}
