//! Storage for unique static strings.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// A symbol.
#[derive(Clone, Copy, Hash)]
pub struct Symbol(usize);

#[derive(Default)]
struct State {
    mapping: HashMap<&'static str, usize>,
    values: Vec<String>,
}

impl Symbol {
    /// Create a new instance.
    pub fn new<T>(value: T) -> Self
    where
        T: AsRef<str> + Into<String>,
    {
        let mut state = State::instance().write().unwrap();
        if let Some(index) = state.mapping.get(value.as_ref()) {
            return Self(*index);
        }
        let index = state.values.len();
        state.values.push(value.into());
        // String internally contains a buffer allocated on the heap, and borrowing it as a str
        // references that buffer, not the String. That means that references remain valid when
        // State grows, and since State can only increase in size, references remain valid more
        // generally until the program terminates.
        let value = unsafe { std::mem::transmute(state.values[index].as_str()) };
        state.mapping.insert(value, index);
        Self(index)
    }
}

impl AsRef<str> for Symbol {
    #[inline]
    fn as_ref(&self) -> &str {
        let state = State::instance().read().unwrap();
        // See the note above.
        unsafe { std::mem::transmute(state.values[self.0].as_str()) }
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

impl std::fmt::Debug for Symbol {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.as_ref(), formatter)
    }
}

impl std::fmt::Display for Symbol {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.as_ref(), formatter)
    }
}

impl std::ops::Deref for Symbol {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
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
            serializer.serialize_str(self.as_ref())
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
