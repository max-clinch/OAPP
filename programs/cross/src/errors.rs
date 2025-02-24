use anchor_lang::prelude::*;

#[error_code]
pub enum CounterError {
    #[msg("Unauthorized: Only the admin can perform this action.")]
    UnauthorizedSender,

    #[msg("Invalid message type received.")]
    InvalidMessageType,

    #[msg("Not enough funds to cover LayerZero fees.")]
    InsufficientFunds,

    #[msg("Remote address mismatch.")]
    RemoteAddressMismatch,

    #[msg("Invalid source EID in message.")]
    InvalidSourceEid,

    #[msg("Failed to decode the incoming message.")]
    MessageDecodingFailed,

    #[msg("Swap execution failed.")]
    SwapExecutionFailed,

    #[msg("Remote account not found.")]
    RemoteAccountNotFound,

    #[msg("LayerZero endpoint settings are incorrect.")]
    InvalidEndpointSettings,

    #[msg("Unknown error occurred.")]
    UnknownError,

    #[msg("Failed to clear the message in LayerZero.")]
    ClearFailed, 

    #[msg("Failed to send composed message.")]
    SendComposeFailed, 

    #[msg("Unauthorized recipient.")]  // ✅ Add this error
    UnauthorizedRecipient,
}
