struct Formatter {
    code: String,
}

impl Formatter {
    fn format(self) -> String {
        self.code
    }

    pub fn new(code: String, _args: FormatterArguments) -> Self {
        Formatter { code }
    }
}

pub trait Sexp {
    fn is_sexp(&self) -> bool;
}

impl Sexp for String {
    /// is_sexp function returns true if line's first charactor is left bracket.
    /// ```
    /// use lisp_fmt_rs::Sexp;
    ///
    /// assert!(String::from("(print 'Alice')").is_sexp());
    /// assert!(!String::from("print 'This line is missing a left bracket')").is_sexp());
    /// assert!(!String::from("# This is emacs-lisp's comment").is_sexp());
    /// assert!(!String::from(";; This is commonlisp's comment").is_sexp());
    /// ```
    fn is_sexp(&self) -> bool {
        self.chars().next() == Some('(')
    }
}


struct FormatterArguments {}

impl FormatterArguments {
    fn new() -> Self {
        FormatterArguments {}
    }
}

#[cfg(test)]
mod tests {
    use super::Sexp;
    use super::{Formatter, FormatterArguments};

    #[test]
    fn nothing_to_change() {
        let code: String = String::from("(print 'Hello')");
        let formatter_args: FormatterArguments = FormatterArguments::new();
        let formatter: Formatter = Formatter::new(code, formatter_args);
        let result: String = formatter.format();
        assert_eq!(result, "(print 'Hello')");
    }
}
