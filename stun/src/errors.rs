use util::Error;

lazy_static! {
    // ErrAttributeNotFound means that attribute with provided attribute
    // type does not exist in message.
    pub static ref ERR_ATTRIBUTE_NOT_FOUND: Error = Error::new("attribute not found".to_owned());
    // ErrTransactionStopped indicates that transaction was manually stopped.
    pub static ref ERR_TRANSACTION_STOPPED: Error = Error::new("transaction is stopped".to_owned());
    // ErrTransactionNotExists indicates that agent failed to find transaction.
    pub static ref ERR_TRANSACTION_NOT_EXISTS: Error = Error::new("transaction not exists".to_owned());
    // ErrTransactionExists indicates that transaction with same id is already
    // registered.
    pub static ref ERR_TRANSACTION_EXISTS: Error = Error::new("transaction exists with same id".to_owned());
    // ErrAgentClosed indicates that agent is in closed state and is unable
    // to handle transactions.
    pub static ref ERR_AGENT_CLOSED: Error = Error::new("agent is closed".to_owned());
    // ErrTransactionTimeOut indicates that transaction has reached deadline.
    pub static ref ERR_TRANSACTION_TIME_OUT: Error = Error::new("transaction is timed out".to_owned());
    // ErrNoDefaultReason means that default reason for provided error code
    // is not defined in RFC.
    pub static ref ERR_NO_DEFAULT_REASON: Error = Error::new("no default reason for ErrorCode".to_owned());
    pub static ref ERR_UNEXPECTED_EOF: Error = Error::new("unexpected EOF".to_owned());
    // ErrAttributeSizeInvalid means that decoded attribute size is invalid.
    pub static ref ERR_ATTRIBUTE_SIZE_INVALID: Error = Error::new("attribute size is invalid".to_owned());
    // ErrAttributeSizeOverflow means that decoded attribute size is too big.
    pub static ref ERR_ATTRIBUTE_SIZE_OVERFLOW: Error = Error::new("attribute size overflow".to_owned());
    // ErrDecodeToNil occurs on Decode(data, nil) call.
    pub static ref ERR_DECODE_TO_NIL: Error = Error::new("attempt to decode to nil message".to_owned());
    // ErrUnexpectedHeaderEOF means that there were not enough bytes in Raw to read header.
    pub static ref ERR_UNEXPECTED_HEADER_EOF: Error = Error::new("unexpected EOF: not enough bytes to read header".to_owned());
    // ErrIntegrityMismatch means that computed HMAC differs from expected.
    pub static ref ERR_INTEGRITY_MISMATCH: Error = Error::new("integrity check failed".to_owned());
    // ErrFingerprintMismatch means that computed fingerprint differs from expected.
    pub static ref ERR_FINGERPRINT_MISMATCH: Error = Error::new("fingerprint check failed".to_owned());
    // ErrFingerprintBeforeIntegrity means that FINGERPRINT attribute is already in
    // message, so MESSAGE-INTEGRITY attribute cannot be added.
    pub static ref ERR_FINGERPRINT_BEFORE_INTEGRITY: Error = Error::new("FINGERPRINT before MESSAGE-INTEGRITY attribute".to_owned());
    // ErrIntegrityMismatch means that computed HMAC differs from expected.

}