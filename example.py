# Complete working example

from zaru_core import PaymentEngine, version, generate_keypair
import  json

print("=" * 60)
print(" ZARU-CORE PAYMENT ENGINE DEMO")
print("=" * 60)

# 1. Initialize
print("\n Initializing...")
engine = PaymentEngine()
print(f"	Version: {version()}")

# 2. Generate a keypair for the sender
print("\n 	Generating sender keypair...")
sender_pk = generate_keypair()
print(f"	Sender public key: {sender_pk[:20]}...")

# 3. Create wallets
print("\n Creating wallets...")
sender_wallet = engine.create_wallet("crypto", sender_pk)
receiver_wallet = engine.create_wallet("crypto", "0xRecipientAddress123")

print(f"	Sender: {sender_wallet}")
print(f"	Receiver: {receiver_wallet}")

# 4. Create amount
print("\n Creating payment amount...")
amount = engine.create_amount(5000)
print(f"	Amount: {amount}")

# 5.  Create transaction
print("\ Creating transaction...")

tx = engine.create_transaction(
	id ="demo_txn_001",
	from=sender_wallet,
	to=receiver_wallet,
	amount=amount,
	nonce=1
)


print(f"	Transaction ID: {tx.id}")
print(f"	From: {tx.from_wallet}")
print(f"	To: {tx.to_wallet}")
print(f"	Amount: {tx.amount}")

# 6. Verify the transaction

print("\n Verify transaction...")
is_valid = engine.verify_transaction(tx)

print(f"	Signature valid: {is_valid}")

if is_valid: 
	print("		Transaction is VALID and cryptographically verified")
else:
	print("		Transaction is INVALID")

# 7. Get JSON representation

print("\n Transaction JSON:")
json_data = json.loads(tx.to_json())
print(json.dumps(json_data, indent=2))

print("\n" + "=" * 60)
print("		Demo completed successfully!")
print("=" * 60)

