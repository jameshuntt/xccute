//! Composite command contract re-exports.
//!
//! The concrete contract traits live in `xccute_contract` so proc macros and
//! the runtime can share one stable target surface.

pub use xccute_contract::{
    CommandApproval,
    CommandApprovalError,
    CommandApprovalReceipt,
    CommandExecutionError,
    CommandPolicyError,
    CommandPolicyResult,
    CommandPreview,
    CommandPreviewPolicy,
    CommandValidationError,
    CommandValidationErrorKind,
    CommandValidationResult,
    CompositeArgvPart,
    CompositeShellCommand,
    CompositeShellRoot,
    XccuteArrangedCommand,
    XccuteCommandArrangement,
    XccuteCommandPolicyMetadata,
    XccuteCommandSensitivity,
    XccutePolicyMetadata,
    RootableCompositeSurface,
    RootableSubCommand,
    ValidatedCommand,
    ValidatedCompositeShellCommand,
};
