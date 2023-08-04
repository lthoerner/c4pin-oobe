pub trait OobeString {
    fn is_yes(&self) -> bool;
}

impl OobeString for String {
    fn is_yes(&self) -> bool {
        self.as_str().is_yes()
    }
}

impl<'a> OobeString for &'a str {
    fn is_yes(&self) -> bool {
        matches!(*self, "yes" | "y")
    }
}
