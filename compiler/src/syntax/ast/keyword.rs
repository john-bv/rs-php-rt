/// Keywords that are valid in PHP.
/// These are words that are reserved by the compiler as they serve a certain purpose.
/// ! Please note that this compiler treats language constructs or "constant functions" as
/// ! as reserved calls!
///
/// For more information regarding reserved keywords, visit: https://www.php.net/manual/en/reserved.keywords.php
#[derive(Debug, Clone)]
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

    /// Else, a control word, used as the negating clause to the `if` expression represented.
    /// For instance, if you have a clause where `$a > $b` and `$a = 1` and `$b = 0`, the else
    /// statement will be executed.
    Else,

    /// ElseIf, Similar to else, but allows another truthy expression to be evaluated to continue
    /// walking into scope.
    Elseif,

    /// This is used primarily in a WebApp, where you would have a rendered scope and declare the end of the scope
    /// For example:
    /// ```php
    /// <?php declare(something=1): ?>
    /// ... html ...
    /// <?php enddeclare ?>
    /// ```
    EndDeclare,

    /// Functions identically to EndDeclare.
    EndFor,

    /// Functions identically to EndDeclare.
    EndForEach,

    /// Functions identically to EndDeclare.
    EndIf,

    /// Functions identically to EndDeclare.
    EndSwitch,

    /// Functions identically to EndDeclare.
    EndWhile,

    /// Extends, A keyword used to "extend", "derive", or "inherit" the methods, constants and properties
    /// from the label following this keyword. For instance, `Foo` would extend `Bar` if `Foo extends Bar`
    /// is written.
    Extends,

    /// Final, A permissive word used to prevent overriding of a method or constant by prefixing the label with the word
    /// "final". This keyword can be used with classes, methods, and constants within classes.
    Final,

    /// Finally, is used to execute code regardless of whether an exception was thrown or not. Finally will be executed after either
    /// the `try` or `catch` block has been executed.
    Finally,

    /// Fn, Used to introduce arrow functions. The Fn keyword creates a function using the built-in `\Closure` class. Arrow functions
    /// will have the basic form of `fn (args) => expr`. Arrow functions function similarly to anonymous functions, except that they
    /// use variables from the parent scope by default.
    Fn,

    /// For, a control structure used to loop over and execute an instruction after each loop.
    /// Syntax:
    /// ```php
    /// for (beginning; condition; end_iteration_expression)
    /// ```
    For,

    /// ForEach, a control structure specifically designed to iterate over an array or object.
    /// For information regarding this control word please visit: https://www.php.net/manual/en/control-structures.foreach.php
    ForEach,

    /// Function, a keyword that allows a user to define a block of code that can be called repeatedly using it's label.
    Function,

    /// Global, A cursed keyword used to expose a variable to the entire execution tree. Variables exposed by global can be accessed
    /// using the reserved `$GLOBALS` object.
    Global,

    /// GoTo, another dumb keyword used for jumping to a label a user defines. A label is defined by an identfier following a colon.
    /// You should definitely use functions instead of this, however this keyword operates identically to a single `jmp` during instruction.
    ///
    /// Example:
    /// ```php
    /// dog:
    /// echo "hi";
    ///
    /// goto dog;
    /// ```
    GoTo,

    /// If, a control structure that evaluates a given expression to a boolean, if truthy the block following the `if` keyword and condition
    /// will be executed, otherwise the `else` block is executed.
    If,

    /// Implements, used to specify that a class must implement the given interface.
    Implements,

    Include,

    IncludeOnce,

    InstanceOf,

    InsteadOf,

    /// Interface, used to specify code that a class must implement without having to define how these methods are implemented. Dumbed down,
    /// an interface is a collection of methods that you promise to define later using the `implements` keyword on the class.
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

    /// Use in `yield from` for generators.
    From,
}
