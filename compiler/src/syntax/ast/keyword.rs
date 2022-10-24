/// Keywords that are valid in PHP.
/// These are words that are reserved by the compiler as they serve a certain purpose.
/// ! Please note that this compiler treats language constructs or "embeded functions" as
/// ! as reserved expressions!
///
/// For more information regarding reserved keywords, visit: https://www.php.net/manual/en/reserved.keywords.php
pub enum Keyword {
    /// Abstract, used to define "interface" classes and
    /// are not stand-alone initializable.
    Abstract,

    /// And, used for logical OP. While you should avoid this
    /// the language supports it so it's implemented
    And,

    /// As, used for iterating over arrays or objects.
    /// Usage:
    /// ```php no_run
    /// $arr = [1, 3, 44];
    /// foreach ($arr as $v) {
    ///     echo "$v";
    /// }
    /// ```
    As,

    /// Break, a control operator,
    /// used to break out of the statements:
    /// for, foreach, while, do-while, or switch.
    ///
    /// Optionally accepts a numerical argument to break
    /// out of the nested call stack. Defaults to 1, which
    /// breaks out of the immediate execution stack.
    Break,

    /// Case, operates as an "if" operation inside an embeded switch statement.
    /// A logical compairson comparing the right hand to the expression within the
    /// switch statement.
    ///
    /// Usage:
    /// ```php
    /// switch ($i) {
    ///     case 0:
    ///         echo "i is 0";
    ///         break;
    /// }
    /// ```
    Case,

    /// Catch, Used to handle exceptions in the current call stack. If no catch block is found,
    /// the executor will continue up the execution tree until one is found, executing all
    /// `finally` blocks it encouters along the way. If the call stack is traversed until the initial
    /// call and no catch block is encountered, the program will terminate unless a global exception
    /// handler has been set.
    Catch,

    /// Class, A class is an instantiable structure with fields and methods that are constant within each
    /// instance of the class.
    ///
    /// Class labels must follow the regex: `^[a-zA-Z_\x80-\xff][a-zA-Z0-9_\x80-\xff]*$.` which in turn
    /// sets a class label to be defined by one of the following conditions:
    /// - It must contain a valid letter or underscore
    /// - May contain numbers or letters AFTER the first condition is met
    /// - Must be within the charset of ASCII
    Class,

    /// Clone, an operation to copy an object, by default this follows PHP Zend's behavior, where
    /// and object will copy it's references. For a deep copy, use the PHP-RST standard library function
    /// `deepcopy()`
    ///
    /// ! THIS BEHAVIOR MAY VARY IN THE FUTURE
    Clone,

    /// Const, Defines a immutable (non-changable) variable within a class.
    ///
    /// By default, this will follow PHP's implementation where constants are only allowed
    /// within classes.
    ///
    /// The alternative behavior, where constants can be created outside
    /// a statement is possible with an experiements flag.
    Const,

    /// Continue, a control word to skip the current execution tree if within a loop iteration and continue
    /// the execution condition evaluation at the beginning of the next iteration.
    Continue,

    /// Declare, a cursed keyword, that is used to set up directives for the block of code. Any statement following
    /// a declaration can be used by its label as an alias to the statement.
    ///
    /// Usage:
    /// ```php
    /// declare(apple="pair");
    ///
    /// // string(4) "pair"
    /// var_dump(apple);
    /// ```
    Declare,

    /// Default, a control word only allowed in switch statements, provides the default implementation for a condition
    /// not matching any previous case clause.
    Default,

    /// Do, a keyword used to change the underlying behavior of a `while` loop. Do will execute the expression for the
    /// while loop *after* each iteration rather than before. This means that the code within the while loop will be called
    /// before the expression of the while loop is executed.
    ///
    /// Essentially equivelant to:
    /// ```php
    /// do_something();
    /// while (false) {
    ///     do_something();
    /// }
    /// ```
    Do,

    Else,

    Elseif,

    EndDeclare,

    EndFor,

    EndForEach,

    EndIf,

    EndSwitch,

    EndWhile,

    Extends,

    Final,

    Finally,

    Fn,

    For,

    ForEach,

    Function,

    Global,

    GoTo,

    If,

    Implements,

    Include,

    IncludeOnce,

    InstanceOf,

    InsteadOf,

    Interface,

    Match,

    Namespace,

    New,

    Or,

    Private,

    Protected,

    Public,

    ReadOnly,

    Require,

    RequireOnce,

    Return,

    Static,

    Switch,

    Throw,

    Trait,

    Try,

    Use,

    Var,

    While,

    Yield,

    YieldFrom,
}
