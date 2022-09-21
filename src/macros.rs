#[macro_export]
macro_rules! create_punctuation_kind {
    (Separator) => {
        PunctuationKind::Separator
    };
    (Open $Depth:expr) => {
        PunctuationKind::Open($Depth)
    };
    (Close $Depth:expr) => {
        PunctuationKind::Close($Depth)
    };
    (Start) => {
        PunctuationKind::Start
    };
}

#[macro_export]
macro_rules! token {
    (EOT) => {
        hello_world::TokenType::EOT
    };
    // tt = token tree
    (Char $raw:tt) => {
        hello_world::TokenType::Char($raw)
    };
    (Punctu $raw:tt ($($inner:tt)+)) => {
        hello_world::TokenType::Punctu {raw: $raw, kind: create_punctuation_kind!($($inner)+)}
    };
}