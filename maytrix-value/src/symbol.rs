/// A validated identifier following the pattern `^[a-z][a-z0-9_]*$`.
///
/// `Symbol` ensures its inner value is a well-formed, lowercase ASCII identifier
/// commonly used for names, keys, or codes. It provides efficient comparison
/// and map/set usage by implementing `Eq`, `Ord`, and `Hash` and supports
/// borrowing as `&str`.
///
/// # Examples
///
/// Creating a valid `Symbol`:
///
/// ```
/// use maytrix_value::Symbol;
/// let sym = Symbol::try_new("alpha_1").unwrap();
/// assert_eq!(sym.as_str(), "alpha_1");
/// ```
///
/// Invalid values yield an error:
///
/// ```
/// use maytrix_value::Symbol;
/// assert!(Symbol::try_new("Bad-Name").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    value: String,
}

impl Symbol {
    /// Attempts to construct a `Symbol` from a string-like value.
    ///
    /// The input must match the regex `^[a-z][a-z0-9_]*$`.
    ///
    /// # Examples
    ///
    /// Successful creation:
    /// ```
    /// use maytrix_value::Symbol;
    /// let s = Symbol::try_new("task1").unwrap();
    /// assert_eq!(s, "task1");
    /// ```
    ///
    /// Failure on invalid input:
    /// ```
    /// use maytrix_value::Symbol;
    /// assert!(Symbol::try_new("1bad").is_err());
    /// ```
    pub fn try_new<S: Into<String>>(value: S) -> Result<Self, SymbolError> {
        let s = value.into();
        if Self::is_valid(&s) {
            Ok(Self { value: s })
        } else {
            Err(SymbolError)
        }
    }

    /// Returns the inner string slice.
    ///
    /// This is equivalent to dereferencing `Symbol` to `&str`.
    ///
    /// ```
    /// use maytrix_value::Symbol;
    /// let s = Symbol::try_new("ok").unwrap();
    /// assert_eq!(s.as_str(), "ok");
    /// assert_eq!(&*s, "ok"); // Deref to str
    /// ```
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns true if the provided string matches `^[a-z][a-z0-9_]*$`.
    ///
    /// This is a pure validator that does not allocate.
    ///
    /// ```
    /// use maytrix_value::Symbol;
    /// assert!(Symbol::is_valid("a"));
    /// assert!(Symbol::is_valid("a0_b"));
    /// assert!(!Symbol::is_valid("_bad"));
    /// assert!(!Symbol::is_valid("Nope"));
    /// ```
    pub fn is_valid(s: &str) -> bool {
        let mut chars = s.chars();
        match chars.next() {
            Some(first) if first.is_ascii_lowercase() => {}
            _ => return false,
        }
        for c in chars {
            if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
                return false;
            }
        }
        true
    }
}

impl core::fmt::Display for Symbol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl core::ops::Deref for Symbol {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl core::str::FromStr for Symbol {
    type Err = SymbolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Symbol::try_new(s)
    }
}

impl TryFrom<&str> for Symbol {
    type Error = SymbolError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Symbol::try_new(value)
    }
}

impl TryFrom<String> for Symbol {
    type Error = SymbolError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Symbol::try_new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolError;

impl core::fmt::Display for SymbolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "value must match ^[a-z][a-z0-9_]*$")
    }
}

impl std::error::Error for SymbolError {}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl core::borrow::Borrow<str> for Symbol {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl core::cmp::PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

impl core::cmp::Ord for Symbol {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl From<Symbol> for String {
    fn from(s: Symbol) -> Self {
        s.value
    }
}

impl From<Symbol> for Box<str> {
    fn from(s: Symbol) -> Self {
        s.value.into_boxed_str()
    }
}

// Optional ergonomic cross-type equality
impl PartialEq<&str> for Symbol {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl PartialEq<Symbol> for &str {
    fn eq(&self, other: &Symbol) -> bool {
        *self == other.as_str()
    }
}
impl PartialEq<String> for Symbol {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}
impl PartialEq<Symbol> for String {
    fn eq(&self, other: &Symbol) -> bool {
        self.as_str() == other.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::str::FromStr;

    #[test]
    fn is_valid_accepts_simple_lowercase() {
        assert!(Symbol::is_valid("a"));
        assert!(Symbol::is_valid("abc"));
        assert!(Symbol::is_valid("z"));
    }

    #[test]
    fn is_valid_accepts_digits_and_underscores_after_first() {
        assert!(Symbol::is_valid("a1"));
        assert!(Symbol::is_valid("a_b"));
        assert!(Symbol::is_valid("a1_b2_c3"));
        assert!(Symbol::is_valid("a0_9"));
        assert!(Symbol::is_valid("a__"));
    }

    #[test]
    fn is_valid_rejects_empty_and_bad_first_char() {
        assert!(!Symbol::is_valid(""));
        assert!(!Symbol::is_valid("1abc"));
        assert!(!Symbol::is_valid("_abc"));
        assert!(!Symbol::is_valid("A"));
    }

    #[test]
    fn is_valid_rejects_invalid_tail_chars() {
        assert!(!Symbol::is_valid("a-"));
        assert!(!Symbol::is_valid("a-1"));
        assert!(!Symbol::is_valid("a b"));
        assert!(!Symbol::is_valid("a$"));
        assert!(!Symbol::is_valid("aB")); // uppercase after first not allowed either
        assert!(!Symbol::is_valid("aÄ")); // non-ascii letter
    }

    #[test]
    fn try_new_constructs_for_valid_and_errors_for_invalid() {
        let ok = Symbol::try_new("abc_123");
        assert!(ok.is_ok());
        assert_eq!(ok.unwrap().as_str(), "abc_123");

        let err = Symbol::try_new("-bad");
        assert!(err.is_err());
    }

    #[test]
    fn display_and_deref_expose_inner() {
        let s = Symbol::try_new("abc_123").unwrap();
        assert_eq!(&*s, "abc_123"); // Deref<str>
        assert_eq!(s.as_str(), "abc_123");
        assert_eq!(s.to_string(), "abc_123");
    }

    #[test]
    fn from_str_and_try_from_work() {
        let s1 = Symbol::from_str("name1").unwrap();
        assert_eq!(s1, "name1");

        let s2: Result<Symbol, _> = "x_y".try_into();
        assert_eq!(s2.unwrap(), "x_y");

        let s3: Result<Symbol, _> = String::from("ok_2").try_into();
        assert_eq!(s3.unwrap(), "ok_2");

        let bad: Result<Symbol, _> = "Nope".try_into();
        assert!(bad.is_err());
    }

    #[test]
    fn error_display_message_matches_spec() {
        let err = Symbol::try_new("Bad-Name").unwrap_err();
        assert_eq!(err.to_string(), "value must match ^[a-z][a-z0-9_]*$");
    }

    #[test]
    fn equality_and_hash_semantics() {
        use std::collections::HashSet;
        let a = Symbol::try_new("abc").unwrap();
        let b = Symbol::try_new("abc").unwrap();
        let c = Symbol::try_new("abd").unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut set = HashSet::new();
        set.insert(a);
        assert!(set.contains(&b));
        assert!(!set.contains(&c));
        // Borrow<str> enables contains lookup by &str in HashSet as well
        assert!(set.contains("abc"));
        assert!(!set.contains("abd"));
    }

    #[test]
    fn as_ref_borrow_and_hashmap_lookup() {
        use std::collections::HashMap;
        let key = Symbol::try_new("alpha").unwrap();
        let mut map = HashMap::new();
        map.insert(key.clone(), 42);
        // Lookup by &str thanks to Borrow<str>
        assert_eq!(map.get("alpha"), Some(&42));

        // AsRef<str>
        fn takes_as_ref<S: AsRef<str>>(s: S) -> usize { s.as_ref().len() }
        assert_eq!(takes_as_ref(&key), 5);
    }

    #[test]
    fn ordering_and_btreeset() {
        use std::collections::BTreeSet;
        let inputs = ["beta", "alpha", "alpha_1", "alpha0"];
        let mut syms: Vec<Symbol> = inputs.iter().map(|s| Symbol::try_new(*s).unwrap()).collect();
        syms.sort(); // requires PartialOrd/Ord
        let sorted: Vec<&str> = syms.iter().map(|s| s.as_str()).collect();
        assert_eq!(sorted, vec!["alpha", "alpha0", "alpha_1", "beta"]);

        let set: BTreeSet<Symbol> = syms.into_iter().collect();
        let ordered: Vec<&str> = set.iter().map(|s| s.as_str()).collect();
        assert_eq!(ordered, vec!["alpha", "alpha0", "alpha_1", "beta"]);
    }

    #[test]
    fn into_string_and_boxed_str() {
        let s = Symbol::try_new("gamma").unwrap();
        let owned: String = s.clone().into();
        assert_eq!(owned, "gamma");
        let boxed: Box<str> = s.clone().into();
        assert_eq!(&*boxed, "gamma");
    }

    #[test]
    fn cross_type_equality() {
        let s = Symbol::try_new("delta_1").unwrap();
        assert!(s == "delta_1");
        assert!("delta_1" == s);
        assert!(String::from("delta_1") == s);
        assert!(s == String::from("delta_1"));
        assert!(s != "delta2");
    }
}
