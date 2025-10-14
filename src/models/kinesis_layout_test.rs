use super::kinesis_layout::{KeyAction, KinesisLayout};

#[test]
fn test_parse_simple_remap() {
    let content = "[2]>[7]\n[1]>[8]";
    let layout: KinesisLayout = content.parse().unwrap();

    assert_eq!(layout.mappings.len(), 2);
    assert_eq!(
        layout.mappings[0],
        KeyAction::SimpleRemap {
            source: "2".to_string(),
            target: "7".to_string()
        }
    );
}

#[test]
fn test_parse_macro() {
    let content = "{hyphen}>{speed5}{-lalt}{tab}{+lalt}";
    let layout: KinesisLayout = content.parse().unwrap();

    assert_eq!(layout.mappings.len(), 1);
    match &layout.mappings[0] {
        KeyAction::Macro { trigger, actions } => {
            assert_eq!(trigger, "hyphen");
            assert_eq!(actions, "{speed5}{-lalt}{tab}{+lalt}");
        }
        _ => panic!("Expected macro"),
    }
}

#[test]
fn test_display_trait() {
    let mut layout = KinesisLayout::new();
    layout.add_remap("a".to_string(), "b".to_string());
    layout.add_macro("test".to_string(), "{speed5}".to_string());

    let output = format!("{}", layout);

    assert!(output.contains("[a]>[b]"));
    assert!(output.contains("{test}>{speed5}"));
}

#[test]
fn test_empty_layout() {
    let content = "";
    let layout: KinesisLayout = content.parse().unwrap();
    assert_eq!(layout.mappings.len(), 0);
}

#[test]
fn test_comments() {
    let content = "# This is a comment\n[a]>[b]\n# Another comment";
    let layout: KinesisLayout = content.parse().unwrap();
    assert_eq!(layout.mappings.len(), 1);
}

#[test]
fn test_error_missing_operator() {
    let content = "[a][b]";
    let result = content.parse::<KinesisLayout>();
    assert!(result.is_err());
}

#[test]
fn test_find_by_source() {
    let mut layout = KinesisLayout::new();
    layout.add_remap("a".to_string(), "b".to_string());
    layout.add_remap("c".to_string(), "d".to_string());

    let found = layout.find_by_source("a");
    assert_eq!(found.len(), 1);
}

#[test]
fn test_remove_by_source() {
    let mut layout = KinesisLayout::new();
    layout.add_remap("a".to_string(), "b".to_string());
    layout.add_remap("c".to_string(), "d".to_string());

    layout.remove_by_source("a");
    assert_eq!(layout.mappings.len(), 1);
    assert_eq!(layout.find_by_source("a").len(), 0);
}

#[test]
fn test_invalid_bracket_combinations() {
    // Should fail: starts with [ but doesn't end with ]
    let content1 = "[a>>[b]";
    assert!(content1.parse::<KinesisLayout>().is_err());

    // Should fail: ends with ] but doesn't start with [
    let content2 = "a]>[b]";
    assert!(content2.parse::<KinesisLayout>().is_err());

    // Should fail: only one bracket
    let content3 = "[a>[b]";
    assert!(content3.parse::<KinesisLayout>().is_err());
}

#[test]
fn test_invalid_curly_bracket_combinations() {
    // Should fail: starts with { but doesn't end with }
    let content1 = "{trigger>>{actions}";
    assert!(content1.parse::<KinesisLayout>().is_err());

    // Should fail: ends with } but doesn't start with {
    let content2 = "trigger}>{actions}";
    assert!(content2.parse::<KinesisLayout>().is_err());

    // Should fail: only one curly bracket
    let content3 = "{trigger>{actions}";
    assert!(content3.parse::<KinesisLayout>().is_err());

    // Should fail: curly on source, square on target (mixed)
    let content4 = "{trigger}>[target]";
    assert!(content4.parse::<KinesisLayout>().is_err());
}

#[test]
fn test_mixed_bracket_types() {
    // Should fail: square bracket source, but formatted like macro target
    let content1 = "[key]>{actions}";
    assert!(content1.parse::<KinesisLayout>().is_err());

    // Should fail: curly bracket source, square bracket target
    let content2 = "{key}>[target]";
    assert!(content2.parse::<KinesisLayout>().is_err());
}
