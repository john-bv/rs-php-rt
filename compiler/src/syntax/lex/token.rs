use std::ops::Range;

use crate::syntax::ast::{
    keyword::Keyword,
    reserved::{ReservedCall, ReservedIdent},
};

#[derive(Debug, Clone)]
pub struct Token(pub TokenType, pub Range<usize>, pub Option<String>);

impl Token {
    pub fn kind(&self) -> TokenType {
        self.0.clone()
    }

    pub fn value(&self) -> Option<String> {
        self.2.clone()
    }

    pub fn range(&self) -> Range<usize> {
        self.1.clone()
    }
}

/// A simple utility macro to create a token from an expression, for example:
///
/// ```rust no_run
/// use crate::syntax::lex::token::token;
///
/// let lf_tk: Token = token!(1, 2, TokenType::LF, Some(LF::CRLF));
/// ```
#[macro_export]
macro_rules! token {
    ($start: expr, $end: expr, $t: expr, $v: expr) => {
        Some(Token($t, $start..$end, $v))
    };
    ($start: expr, $end: expr, $t: expr) => {
        Some(Token($t, $start..$end, None))
    };
}

/// A enum representing a valid php numerical.
/// ! WARNING, THIS COMPILER DIFFERS IN NUMERICAL IMPLMENTATION FROM PHP
/// ! IN PHP INTS ARE TREATED AS FLOATS, THIS COMPILER RESPECTS THE DATA TYPE
#[derive(Debug, Clone)]
pub enum Numeric {
    Float(f64),
    Int(i32),

    /// LInt, similar to `BigInt` in javascript, allows precision on machines that allow it.
    /// This numeric does NOT exist in regular php.
    LInt(i128),
}

#[derive(Debug, Clone)]
pub enum AccessType {
    /// Another term used to refer to this access is "Scoped Resolution".
    /// However for simplicity, the token is named "StaticMember". This is however,
    /// later parsed to the `ScopedResolution` operation.
    ///
    /// EG:
    /// ```php
    /// class MyConstants {
    ///     public const APPLE = 1;
    /// }
    ///
    /// MyConstants::APPLE;
    /// //         ~~ Static Member Access
    ///```
    StaticMember,
    /// EG:
    /// ```php
    /// class MyInstance {
    ///     public int member = 0;
    /// }
    ///
    /// $instance = new MyInstance();
    /// $instance->member;
    /// //       ~~ Referenced Member
    /// ```
    ReferenceMember,
}

#[derive(Debug, Clone)]
pub enum LF {
    CRLF,
    LF,
}

#[derive(Debug, Clone)]
pub enum StringType {
    /// A single qoute string. Allows use for multi-lined strings.
    Single,
    /// Double qouted strings.
    Double,
    /// A here doc is a specification within php that allows formatting for strings by declaring
    /// an identifier used to indicate the beginning and the end of the formatted string.
    ///
    /// For Example:
    /// ```php no_run
    /// <?php
    /// echo <<<END
    ///       a
    ///      b
    ///     c
    /// END;
    ///
    /// You can optionally add indentation by putting the identifier and the end of the body.
    /// However, you can not indent the closing identifier further than any lines of the body.
    ///```
    HereDoc,
    /// Similar to heredoc, in the fact that the body of the string is NOT parsed, meaning
    /// you can NOT use template arguments or any expression within the string using the
    /// php template literal `{}` or the Prociduous variable expression `$var`.
    ///
    /// Usage:
    /// ```php
    /// <?php
    ///
    /// echo <<<'END'
    ///  This is a $string that will not have \x41 any parsing
    ///  done to it
    /// END;
    /// ```
    NowDoc,
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

    NumericalLit(Numeric),

    /// A string is anything that contains text surrounded by any of the supporting PHP string
    /// types.
    StringLit(StringType),

    /// An operator is a char or word that represents an operation.
    ///
    /// ! THIS MAY BE CONFUSED WITH KEYWORDS "and" "or" AND "not" WHICH ARE
    /// ! CONSIDERED KEYWORDS DURING TOKENIZATION
    Operator,

    /// An accessor is a character that is used to access a value based either on inheritance,
    /// a reference or a static access.
    ///
    /// For Example:
    /// - A `::` is considered a "StaticMember" access,
    /// - A `->` is considered a "ReferenceMember" access
    ///
    Accessor(AccessType),

    /// A token that represents a boolean value. Either "true" or "false".
    Boolean,

    /// A character that is not a letter, number or a string and is not visible
    Whitespace,

    /// The end of statement token is referred to as the semi-colon: `;`.
    EOS,

    /// The line feed, or "line break" is used to represent a char that signals the end of the file,
    /// on windows this is `\r\n` (CRLF) while unix uses `\n` (LF)
    LF(LF),

    /// The `[` character that signals a return type, an array, etc.
    LeftBracket,

    /// The `]` character that signals the end of a return type, an array, etc.
    RightBracket,

    /// The `(` character that signals the start of a argument or expression condition.
    LeftParenthesis,

    /// The `)` character that signals the end of a argument or expression condition.
    RightParenthesis,

    /// The `{` character that signals the start of a block.
    LeftBrace,

    /// The `}` character that signals the end of a block.
    RightBrace,

    /// The `,` character that signals the end of a parameter.
    Comma,

    /// The `\` character that signals the start of a string literal.
    Backslash,

    /// The `$` character that signals the definition or calling of a variable.
    /// > - < wtf php!
    Variable,
}
