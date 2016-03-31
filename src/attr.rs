use std::fmt;
use std::borrow::Cow;

use escape::Escaped;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> Attribute<'a> {
    pub fn new<N, V>(name: N, value: V) -> Attribute<'a>
        where N: Into<Cow<'a, str>>,
              V: Into<Cow<'a, str>>
    {
        Attribute {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl<'a> fmt::Display for Attribute<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO handle invalid attribute names
        write!(f, "{}=\"{}\"", self.name, Escaped(&self.value))
    }
}

#[derive(Clone, Debug)]
pub struct AttributeSet<'a>(pub Cow<'a, [Attribute<'a>]>);

impl<'a> AttributeSet<'a> {
    pub fn empty() -> AttributeSet<'a> {
        AttributeSet(Cow::Borrowed(&[]))
    }
}

impl<'a> fmt::Display for AttributeSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for attr in self.0.iter() {
            try!(write!(f, " {}", attr));
        }
        Ok(())
    }
}
