#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SettlementStatus {

	/// Transaction has been accepted but not yet processd
	Pending,

	/// Transaction has been processed but may still be reversible
	Confirmed,

	/// Transaction is final and irreversible
	Finalized,

	// Transaction failed permanently
	Failed(FailureReason),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FailureReason {

	InsufficientFunds,
	InvalidSignature,
	ReplayDetected,
	NetworkError(String),
	Unknown,
} 
