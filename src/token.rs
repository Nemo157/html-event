use std::fmt;
use std::borrow::Cow;

use attr::AttributeList;
use escape::Escaped;

#[derive(Clone, Debug, PartialEq, Eq)]
/// An HTML token. By convention, `Token::Text` should be preferred over
/// `Token::RawText` when a piece of text can be represented by both. For
/// instance, use `Token::Text` when tokenizing whitespaces or line-breaks, but
/// use `Token::RawText` for representing all text inside `<style>` tag.
///
/// When `Display`ing a `Token`, the output stream is assumed to be Unicode, and
/// therefore only five characters are escaped: `&`, `<`, `>`, `"`, and `'`
/// ([ref](http://stackoverflow.com/a/7382028)).
pub enum Token<'a> {
    StartTag {
        name: Cow<'a, str>,
        attrs: AttributeList<'a>,
        /// Marker indicating self-closing tags such as `<br />`
        self_closing: bool,
    },
    EndTag {
        name: Cow<'a, str>,
    },
    /// The text contained will be escaped on `Display`.
    Text(Cow<'a, str>),
    /// The text contained will be `Display`ed as-is.
    RawText(Cow<'a, str>),
    /// Comments contained within `<!--` and `-->`. No validation is done to
    /// ensure that the text does not contain `-->`.
    Comment(Cow<'a, str>),
    /// The HTML5 DOCTYPE declaration
    DOCTYPE,
}

impl<'a> Token<'a> {
    pub fn start_tag<S>(name: S, attrs: AttributeList<'a>) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::StartTag {
            name: name.into(),
            attrs: attrs,
            self_closing: false,
        }
    }

    pub fn end_tag<S>(name: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::EndTag { name: name.into() }
    }

    pub fn text<S>(s: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::Text(s.into())
    }

    pub fn raw_text<S>(s: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::RawText(s.into())
    }

    pub fn comment<S>(s: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::Comment(s.into())
    }

    /// If `self` is a `StartTag`, returns the `Token` after setting
    /// `self_closing` to `true`; otherwise, it is a no-op.
    pub fn closed(self) -> Token<'a> {
        if let Token::StartTag { name, attrs, .. } = self {
            Token::StartTag {
                name: name,
                attrs: attrs,
                self_closing: true,
            }
        } else {
            self
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::StartTag { ref name, ref attrs, self_closing } => {
                try!(write!(f, "<{}", name));
                for attr in attrs.iter() {
                    try!(write!(f, " {}", attr));
                }
                if self_closing {
                    write!(f, " />")
                } else {
                    write!(f, ">")
                }
            }
            Token::EndTag { ref name } => write!(f, "</{}>", name),
            Token::Text(ref text) => write!(f, "{}", Escaped(text)),
            Token::RawText(ref text) => write!(f, "{}", text),
            Token::Comment(ref text) => write!(f, "<!--{}-->", text),
            Token::DOCTYPE => write!(f, "<!DOCTYPE html>"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn token_variants() {
        use Token;
        use attr::AttributeList;

        let start_tag = Token::start_tag("tag", AttributeList::empty());
        assert_eq!(format!("{}", start_tag), "<tag>");

        let end_tag = Token::end_tag("tag");
        assert_eq!(format!("{}", end_tag), "</tag>");

        let text = Token::text("<bomb>");
        assert_eq!(format!("{}", text), "&lt;bomb&gt;");

        let raw_text = Token::raw_text("<bomb>");
        assert_eq!(format!("{}", raw_text), "<bomb>");

        let comment = Token::comment("Multi\nline\ncomment");
        assert_eq!(format!("{}", comment), "<!--Multi\nline\ncomment-->");

        assert_eq!(format!("{}", Token::DOCTYPE), "<!DOCTYPE html>");
    }
}
