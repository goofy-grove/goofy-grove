use super::*;

#[test]
fn user_agent_trims_value() {
    assert_eq!(UserAgent::new("  firefox ".to_string()).inner(), "firefox");
}
