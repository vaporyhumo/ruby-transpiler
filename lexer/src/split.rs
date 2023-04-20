use regex::Regex;

pub(crate) fn split(input: &str, pattern: &str) -> Option<(String, String)> {
  let mut new_pattern = pattern.to_string();

  if !pattern.starts_with('^') {
    new_pattern.insert(0, '^');
  }

  let re = Regex::new(&new_pattern);

  if let Some(matched) = re.unwrap().find(input) {
    let matched_str = matched.as_str().to_string();
    let rest = input[matched.end()..].to_string();
    return Some((matched_str, rest));
  }
  None
}
