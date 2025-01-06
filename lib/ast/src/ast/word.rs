use crate::{
    one_of,
    parser_v2::Parser,
    repeat,
    tokens::{Quote, Token},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    raw: String,
    quoted: Option<Quote>,
}

impl Word {
    pub fn new(raw: String, quoted: Option<Quote>) -> Self {
        Self { raw, quoted }
    }

    pub fn raw(&self) -> &String {
        &self.raw
    }
}

impl From<String> for Word {
    fn from(raw: String) -> Self {
        Self { raw, quoted: None }
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.quoted {
            Some(q) => write!(f, "{q}{}{q}", self.raw),
            None => write!(f, "{}", self.raw),
        }
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self))]
    pub fn word(&mut self) -> Option<Word> {
        self.transaction(|parser| {
            let res = one_of!(
                parser,
                || parser.quoted(Quote::Single),
                || parser.quoted(Quote::Double),
                || parser.quoted(Quote::Backtick),
                || parser.choice(|tok| tok.as_word().map(Word::from))
            )?;

            Some(res)
        })
    }

    #[tracing::instrument(skip(self))]
    pub fn non_reserved_word(&mut self) -> Option<Word> {
        self.transaction(|parser| {
            let w = parser.word()?;
            match w.raw().as_str() {
                "case" | "do" | "done" | "elif" | "else" | "fi" | "for" | "if" | "in" | "then"
                | "until" | "while" => None,
                _ => Some(w),
            }
        })
    }

    #[tracing::instrument(skip(self))]
    fn quoted(&mut self, quote: Quote) -> Option<Word> {
        self.transaction(|parser| {
            parser.consume(quote)?;
            parser.set_quoting(Some(quote));

            let buf = repeat!(|| parser.choice(|tok| match tok {
                Token::Quote(q) if q == &quote => None,
                Token::NewLine => None,
                t => Some(t.as_str()),
            }))
            .join("");
            parser.consume(quote)?;
            parser.set_quoting(None);

            let res = Word::new(buf, Some(quote));
            Some(res)
        })
    }
}
