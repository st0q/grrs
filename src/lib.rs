pub fn find_matches(content: &Vec<std::string::String>, pattern: &str, mut writer: impl std::io::Write) {
  for line in content {
      if line.contains(pattern) {
          if let Err(e) = writeln!(writer, "{}", line) {
              eprintln!("Couldn't write to writer: {}", e);
          }
      }
  }
}

#[test]
fn find_a_match() {
  let mut result = Vec::new();
  let content = vec!["lorem ipsum".to_string(), "dolor sit amet".to_string()];
  find_matches(&content, "lorem", &mut result);
  assert_eq!(result, b"lorem ipsum\n");
}
