/// The reserved constants or identifiers that php does NOT allow users to define themselves.
/// Please not that NONE of the `E_*` constants are supported with this compiler.
/// To retrieve these constants use the `\ErrorConstants::E_*` respectively.
#[derive(Debug, Clone)]
pub enum ReservedIdent {
    PhpVersion,
    PhpMajorVersion,
    PhpMinorVersion,
    PhpReleaseVersion,
    PhpVersionId,
    PhpExtraVersion,
    PhpZts,
    PhpDebug,
    PhpMaxPathLen,
    PhpOs,
    PhpOsFamily,
    PhpSapi,
    PhpEol,
    PhpIntMax,
    PhpIntMin,
    PhpFloatDig,
    PhpFloatEpsilon,
    PhpFloatMin,
    PhpFloatMax,
    DefaultIncludePath,
    PearInstallDir,
    PearExtensionDir,
    PhpExtensionDir,
    PhpPrefix,
    PhpBinDir,
    PhpBinary,
    PhpManDir,
    PhpLibDir,
    PhpDataDir,
    PhpLocaleStateDir,
    PhpConfigFilePath,
    PhpConfigFileScanDir,
    PhpShLibSuffix,
    PhpFdSetSize,
    /// A compile time constant, used to get the name of the class
    MagicClass,
    /// A magic constant used to get the cwd directory
    MagicDir,
    /// A magic constant used to get the location of the file running
    MagicFile,
    MagicFunction,
    MagicLine,
    MagicMethod,
    MagicNamespace,
    MagicTrait,
}

#[derive(Debug, Clone)]
pub enum ReservedCall {
    /// Represents the `__halt_compiler()` function
    HaltCompiler,
    /// `array()`
    Array,
    Die,
    Empty,
    Eval,
    Exit,
    IsSet,
    List,
    Unset,
}
