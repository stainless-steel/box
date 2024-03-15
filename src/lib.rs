//! Storage for unique static strings.

use std::collections::HashSet;
use std::sync::{OnceLock, RwLock};

/// A symbol.
#[derive(Clone, Copy, Ord, PartialOrd)]
pub struct Symbol(&'static str);

#[derive(Default)]
struct State(HashSet<&'static str>);

impl Symbol {
    /// Create a new instance.
    pub fn new<T>(value: T) -> Self
    where
        T: AsRef<str> + Into<String>,
    {
        let mut state = State::instance().write().unwrap();
        if let Some(value) = state.0.get(value.as_ref()) {
            return Self(value);
        }
        let value = value.into().leak();
        state.0.insert(value);
        Self(value)
    }
}

impl AsRef<str> for Symbol {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Default for Symbol {
    #[inline]
    fn default() -> Self {
        Self::new::<&str>(Default::default())
    }
}

impl<T> From<T> for Symbol
where
    T: AsRef<str> + Into<String>,
{
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl std::cmp::Eq for Symbol {}

impl std::cmp::PartialEq for Symbol {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl std::fmt::Debug for Symbol {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.0, formatter)
    }
}

impl std::fmt::Display for Symbol {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.0, formatter)
    }
}

impl std::hash::Hash for Symbol {
    #[inline]
    fn hash<T: std::hash::Hasher>(&self, state: &mut T) {
        self.as_ptr().hash(state)
    }
}

impl std::ops::Deref for Symbol {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl State {
    fn instance() -> &'static RwLock<Self> {
        static STATE: OnceLock<RwLock<State>> = OnceLock::new();
        STATE.get_or_init(|| RwLock::new(Default::default()))
    }
}

#[cfg(feature = "serde")]
mod serialization {
    struct Visitor;

    impl<'l> serde::de::Deserialize<'l> for super::Symbol {
        #[inline]
        fn deserialize<T>(deserializer: T) -> Result<Self, T::Error>
        where
            T: serde::de::Deserializer<'l>,
        {
            deserializer.deserialize_str(Visitor)
        }
    }

    impl serde::ser::Serialize for super::Symbol {
        #[inline]
        fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
        where
            T: serde::ser::Serializer,
        {
            serializer.serialize_str(self.0)
        }
    }

    #[cfg(feature = "serde")]
    impl<'l> serde::de::Visitor<'l> for Visitor {
        type Value = super::Symbol;

        #[inline]
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        #[inline]
        fn visit_str<T>(self, value: &str) -> Result<Self::Value, T>
        where
            T: serde::de::Error,
        {
            Ok(super::Symbol::new(value))
        }

        #[inline]
        fn visit_string<T>(self, value: String) -> Result<Self::Value, T>
        where
            T: serde::de::Error,
        {
            Ok(super::Symbol::new(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Symbol;

    #[test]
    fn format() {
        assert_eq!(format!("{}", Symbol::new("foo")), "foo");
    }
}
