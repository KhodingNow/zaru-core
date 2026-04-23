use tokio::task;
use zaru_core::prelude::*;
use zaru_core::ledger::in_memory::InMemorySettlement;
use std::sync::Arc;

#[tokio::test]
async fn test_concurrent_double_spend_attack() {
    let settlement = Arc::new(InMemorySettlement::new());

    let attacker = WalletId::Bank("Attacker".into());
    let victim1 = WalletId::Bank("Victim1".into());
    let victim2 = WalletId::Bank("Victim2".into());

    // ✅ FUND attacker
    settlement.deposit(attacker.clone(), Amount::new(100).unwrap());

    let tx1 = Transaction::<Unsigned>::new(
        TxId("tx-1".into()),
        attacker.clone(),
        victim1,
        Amount::new(100).unwrap(),
        1,
    );

    let tx2 = Transaction::<Unsigned>::new(
        TxId("tx-2".into()),
        attacker.clone(),
        victim2,
        Amount::new(100).unwrap(),
        2,
    );

    let verified1 = tx1.sign(Signature::new(vec![1])).verify().unwrap();
    let verified2 = tx2.sign(Signature::new(vec![2])).verify().unwrap();

    let s1 = Arc::clone(&settlement);
    let s2 = Arc::clone(&settlement);

    let handle1 = task::spawn(async move {
        s1.submit(verified1).await
    });

    let handle2 = task::spawn(async move {
        s2.submit(verified2).await
    });

    let r1 = handle1.await.unwrap();
    let r2 = handle2.await.unwrap();

    let success_count = [r1.is_ok(), r2.is_ok()]
        .iter()
        .filter(|&&x| x)
        .count();

    assert_eq!(success_count, 1, "Double spend occurred");
}

#[tokio::test]
async fn test_replay_attack_same_nonce_different_txid() {
    let settlement = InMemorySettlement::new();

    let from = WalletId::Bank("Attacker".into());
    let to = WalletId::Bank("Victim".into());

    // ✅ FUND attacker
    settlement.deposit(from.clone(), Amount::new(200).unwrap());

    let amount = Amount::new(100).unwrap();

    let tx1 = Transaction::<Unsigned>::new(
        TxId("tx-1".into()),
        from.clone(),
        to.clone(),
        amount.clone(),
        42,
    );

    let tx2 = Transaction::<Unsigned>::new(
        TxId("tx-2".into()),
        from,
        to,
        amount,
        42,
    );

    let verified1 = tx1.sign(Signature::new(vec![1])).verify().unwrap();
    let verified2 = tx2.sign(Signature::new(vec![2])).verify().unwrap();

    let _ = settlement.submit(verified1).await.unwrap();

    let result = settlement.submit(verified2).await;

    assert!(result.is_err(), "Replay attack via nonce was not prevented");
}

#[tokio::test]
async fn test_replay_attack_same_transaction() {
    let settlement = InMemorySettlement::new();

    let from = WalletId::Bank("Attacker".into());
    let to = WalletId::Bank("Victim".into());

    // ✅ FUND attacker
    settlement.deposit(from.clone(), Amount::new(100).unwrap());

    let tx = Transaction::<Unsigned>::new(
        TxId("tx-replay".into()),
        from,
        to,
        Amount::new(100).unwrap(),
        1,
    );

    let verified = tx
        .sign(Signature::new(vec![9, 9, 9]))
        .verify()
        .unwrap();

    let id1 = settlement.submit(verified.clone()).await.unwrap();
    let id2 = settlement.submit(verified.clone()).await.unwrap();
    let id3 = settlement.submit(verified).await.unwrap();

    assert_eq!(id1, id2);
    assert_eq!(id2, id3);

    let final_status = settlement.await_finality(&id1).await.unwrap();

    assert_eq!(final_status, SettlementStatus::Finalized);
}

#[tokio::test]
async fn test_full_settlement_flow() {
    let settlement = InMemorySettlement::new();

    let from = WalletId::Bank("Andile".into());
    let to = WalletId::Bank("Linda".into());

    // ✅ FUND sender
    settlement.deposit(from.clone(), Amount::new(100).unwrap());

    let tx = Transaction::<Unsigned>::new(
        TxId("tx-1".into()),
        from,
        to,
        Amount::new(100).unwrap(),
        1,
    );

    let signed = tx.sign(Signature::new(vec![1, 2, 3]));
    let verified = signed.verify().unwrap();

    let tx_id = settlement.submit(verified).await.unwrap();

    let final_status = settlement.await_finality(&tx_id).await.unwrap();

    assert_eq!(final_status, SettlementStatus::Finalized);
}

#[tokio::test]
async fn test_idempotent_submission() {
    let settlement = InMemorySettlement::new();

    let from = WalletId::Bank("Andile".into());
    let to = WalletId::Bank("Linda".into());

    // ✅ FUND sender
    settlement.deposit(from.clone(), Amount::new(100).unwrap());

    let tx = Transaction::<Unsigned>::new(
        TxId("tx-dup".into()),
        from,
        to,
        Amount::new(100).unwrap(),
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
