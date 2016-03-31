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
        write!(f, "{}=\"{}\"", self.name.as_ref(), Escaped(&self.value))
    }
}

#[derive(Clone, Debug)]
/// Prefer `attr_set!` macro over manual construction
pub struct AttributeSet<'a>(pub Cow<'a, [Attribute<'a>]>);

impl<'a> AttributeSet<'a> {
    fn search_name<S>(&self, name: S) -> Option<usize>
        where S: AsRef<str>
    {
        for (i, attr) in self.0.iter().enumerate() {
            if attr.name == name.as_ref() {
                return Some(i);
            }
        }
        None
    }

    pub fn get_attr<S>(&self, name: S) -> Option<&Attribute<'a>>
        where S: AsRef<str>
    {
        self.search_name(name).map(|index| &self.0.as_ref()[index])
    }

    pub fn get_attr_mut<S>(&mut self, name: S) -> Option<&mut Attribute<'a>>
        where S: AsRef<str>
    {
        self.search_name(name).map(move |index| &mut self.0.to_mut()[index])
    }

    pub fn set_attr<N, V>(&mut self, name: N, value: V)
        where N: Into<Cow<'a, str>>,
              V: Into<Cow<'a, str>>
    {
        let name = name.into();
        if let Some(index) = self.search_name(name.as_ref()) {
            self.0.to_mut()[index].value = value.into();
        } else {
            self.0.to_mut().push(Attribute::new(name, value));
        }
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
