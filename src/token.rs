#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Token {
    Heading(HeadingLevel, String),
    Bold(String),
    Italic(String),
    Text(String),
    BlockQuotes(String),
    Lists(String),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}
