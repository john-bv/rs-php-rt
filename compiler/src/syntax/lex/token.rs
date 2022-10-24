use crate::syntax::ast::{
    keyword::Keyword,
    reserved::{ReservedCall, ReservedLabel},
};

/// Please note that this lexical structure deviates from the original PHP implmentation
/// in that keywords are nested wthin a enum, meaning they are referred to as a byte variant
/// at compile time.
///
/// Other deviations of this lexical structure include:
/// - visibility is PARSED not lexed, during lexing, visibility is a "keyword"
/// - default functions, like echo, and print are not
///   represented by their own token, but rather a value wrapped within a
///   `ReservedCall`
pub enum TokenType {
    /// End of File
    /// Used internally to represent the end of lexing.
    ///
    /// NOT USED TO REPRESENT MAGIC CONSTANTS
    Constant,

    /// A keyword, a word that is reserved and can not be used as a label.
    Keyword(Keyword),

    /// A function that is implemented within the language itself, such as:
    /// - `print`
    /// or
    /// - `eval`
    ReservedCall(ReservedCall),

    /// A label or identifier that is reserved by PHP, for example, a magic constant
    ReservedLabel(ReservedLabel),
}
