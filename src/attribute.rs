use std::fmt;
use std::borrow::Cow;

static EMPTY_ATTRS: &'static [Attribute<'static>] = &[];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> Attribute<'a> {
    pub fn none() -> Cow<'a, [Attribute<'a>]> {
        Cow::Borrowed(EMPTY_ATTRS)
    }
}

impl<'a> fmt::Display for Attribute<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Escaping
        write!(f, "{}=\"{}\"", self.name, self.value)
    }
}

impl<'a> Attribute<'a> {
    pub fn utf8_len(&self) -> usize {
        self.name.len() + self.value.len() + 3
    }
}