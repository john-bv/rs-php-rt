use crate::syntax::ast::{
    keyword::Keyword,
    reserved::{ReservedCall, ReservedIdent},
};

/// A enum representing a valid php numerical.
/// ! WARNING, THIS COMPILER DIFFERS IN NUMERICAL IMPLMENTATION FROM PHP
/// ! IN PHP INTS ARE TREATED AS FLOATS, THIS COMPILER RESPECTS THE DATA TYPE
#[derive(Debug, Clone)]
pub enum Numeric {
    Float(f64),
    Int(i32),

    /// LInt, similar to `BigInt` in javascript, allows precision on machines that allow it.
    /// This numeric does NOT exist in regular php.
    LInt(i128)
}

/// Please note that this lexical structure deviates from the original PHP implmentation
/// in that keywords are nested wthin a enum, meaning they are referred to as a byte variant
/// at compile time.
///
/// Other deviations of this lexical structure include:
/// - visibility is PARSED not lexed, during lexing, visibility is a "keyword"
/// - default functions, like echo, and print are not
///   represented by their own token, but rather a value wrapped within a
///   `ReservedCall`
#[derive(Debug, Clone)]
pub enum TokenType {
    /// End of File
    /// Used internally to represent the end of lexing.
    ///
    /// NOT USED TO REPRESENT MAGIC CONSTANTS
    Constant,

    /// A keyword, a word that is reserved and can not be used as a identifier.
    Keyword(Keyword),

    /// A function that is implemented within the language itself, such as:
    /// - `print`
    /// or
    /// - `eval`
    ReservedCall(ReservedCall),

    /// A identifier that is reserved by PHP, for example, a magic constant,
    /// a class, or another word.
    ReservedIdent(ReservedIdent),

    /// An identifier, anything that is considered text, that could be unknown that is not
    /// any of the matching types above.
    ///
    /// For example:
    /// - `dog`
    /// - `foo`
    Identifier,

    Numerical(Numeric)
}
