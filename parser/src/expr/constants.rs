trait Constant<T> {
  fn get_value(&self) -> T;
}

struct StringConstant {
  value: String,
}

impl StringConstant {
  fn set_value(&mut self, value: String) {
    assert!(value.chars().count() <= 1024, 
            "String exceeds maximum length of 1024 characters; has {} characters", value.len());

    self.value = value;
  }
}

impl Constant<String> for StringConstant {
  fn get_value(&self) -> String {
    self.value.clone()
  }
}

struct TrueConstant;
impl Constant<bool> for TrueConstant {
  fn get_value(&self) -> bool { true }
}

struct FalseConstant;
impl Constant<bool> for FalseConstant {
  fn get_value(&self) -> bool { false }
}
