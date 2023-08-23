pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let test = &self.0;
        true
        // unimplemented!("Determine if the current Luhn struct contains a valid credit card number.");
    }
}

pub trait CheckType {
    fn check_type(&self) -> String;
}

impl CheckType for &str {
    fn check_type(&self) -> String {
        self.chars().filter(|&c| c.is_numeric()).collect::<String>()
    }
}

/*impl CheckType for String {
    fn check_type(&self) -> String {
        self.to_string()
    }
}*/

impl CheckType for u8 {
    fn check_type(&self) -> String {
        self.to_string()
    }
}

impl CheckType for u16 {
    fn check_type(&self) -> String {
        self.to_string()
    }
}

impl CheckType for u32 {
    fn check_type(&self) {
        self.to_string();
    }
}

impl CheckType for u64 {
    fn check_type(&self) {
        self.to_string();
    }
}

impl CheckType for usize {
    fn check_type(&self) {
        self.to_string();
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: CheckType> From<T> for Luhn {
    fn from(input: T) -> Self {
        let convert_var = input.check_type();
        Self(convert_var)
    }
}
