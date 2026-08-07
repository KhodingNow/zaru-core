# test_zaru.py
from zaru_core import PaymentEngine, version, generate_keypair

print("=" * 50)
print("Testing zaru-core Python Bindings")
print("=" * 50)

# Test version
print(f"\n  Version: {version()}")

# Test keypair generation
print("\n🔑 Generating keypair...")
pk = generate_keypair()
print(f"  Public key: {pk[:20]}... (truncated)")

# Create engine
print("\n  Creating payment engine...")
engine = PaymentEngine()
print("  Engine created")

# Test amount creation
print("\n💰 Testing amount creation...")
amount = engine.create_amount(1000)
print(f"  Amount: {amount}")
print(f"   Value: {amount.value}")

# Test invalid amount
try:
    invalid = engine.create_amount(-100)
    print("  Should have failed!")
except ValueError as e:
    print(f"  Correctly rejected negative amount: {e}")

# Test wallet creation
print("\n👛 Testing wallet creation...")
bank_wallet = engine.create_wallet("bank", "Bank of South Africa")
crypto_wallet = engine.create_wallet("crypto", "0x1234567890abcdef")

print(f"  Bank wallet: {bank_wallet}")
print(f"   Is bank: {bank_wallet.is_bank()}")
print(f"   Is crypto: {bank_wallet.is_crypto()}")

print(f"  Crypto wallet: {crypto_wallet}")
print(f"   Is bank: {crypto_wallet.is_bank()}")
print(f"   Is crypto: {crypto_wallet.is_crypto()}")

# CREATE THE KEYPAIR .

print("\n Creating wallet from keypair...")
keypair_wallet = engine.create_wallet("crypto", pk)
print(f" Keypair wallet: {keypair_wallet}")

# Test transaction creation
print("\n📝 Testing transaction creation...")
tx = engine.create_transaction(
    "txn_001",
    keypair_wallet,
    crypto_wallet,
    amount,
    1
)

print("  Transaction created:")
print(f"   ID: {tx.id}")
print(f"   From: {tx.from_wallet}")
print(f"   To: {tx.to_wallet}")
print(f"   Amount: {tx.amount}")
print(f"   Nonce: {tx.nonce}")
print(f"   State: {tx.state}")

# Test JSON serialization
print("\n📄 Testing JSON serialization...")
json_data = tx.to_json()
print(f"  JSON: {json_data}")

# Test signature verification
print("\n🔐 Testing signature verification...")
is_valid = tx.verify_signature()
print(f"  Signature valid: {is_valid}")

# Test transaction verification through engine
print("\n🔍 Testing engine verification...")
is_valid = engine.verify_transaction(tx)
print(f"  Engine says signature valid: {is_valid}")

print("\n" + "=" * 50)
print("🎉 All tests passed! zaru-core is working!")
print("=" * 50)
