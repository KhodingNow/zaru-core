use zaru_core::prelude::*;
use zaru_core::ledger::in_memory::InMemorySettlement;

#[tokio::test]
async fn test_full_settlement_flow() {
	let settlement = InMemorySettlement::new();

	// ---Setup---
	let from = WalletId::Bank("Andile".into());
	let to = WalletId::Bank("Linda".into());

	let amount = Amount::new(100).unwrap();

	let tx = Transaction::<Unsigned>::new(
		TxId("tx-1".into()),
		from,
		to,
		amount,
		1,
	);

	// ---State transitions---

	let signed = tx.sign(Signature::new(vec![1, 2, 3]));
	let verified = signed.verify().unwrap();

	// ---Submit---
	let tx_id = settlement.submit(verified).await.unwrap();

	// ---Track---
	let final_status = settlement.await_finality(&tx_id).await
	.unwrap();

	//--- Assert---
	assert_eq!(final_status, SettlementStatus::Finalized);

	
}

//---- Idempotency---

#[tokio::test]
async fn test_idempotent_submission() {
	let settlement = InMemorySettlement::new();

	let tx = Transaction::<Unsigned>::new(
		TxId("tx-dup".into()),
		WalletId::Bank("Andile".into()),
		WalletId::Bank("Linda".into()),
		Amount::new(50).unwrap(),
		1,
	);
	
	let verified = tx
		.sign(Signature::new(vec![1]))
		.verify()
		.unwrap();

	let id1 = settlement.submit(verified.clone()).await.unwrap();
	let id2 = settlement.submit(verified).await.unwrap();

	assert_eq!(id1, id2);
}
