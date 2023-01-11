use crate::Environment;

#[test]
pub fn test() {
  let env = Environment::from_file("./src/tests/files/.env.testing");
  let some_string: String = env.require("SOME_STRING");
  let some_bool: bool = env.require("SOME_BOOLEAN");
  let some_number: i32 = env.require("SOME_NUMBER");
  assert_eq!(some_string, "asdffdsa".to_string());
  assert_eq!(some_bool, true);
  assert_eq!(some_number, 12345);
}
