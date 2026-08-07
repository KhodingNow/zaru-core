import pytest
from zaru_core import PaymentEngine, version, generate_keypair

class TestZaruCore:
	def test_version(self):
		assert version() == "0.1.0"


	def test_keypair_generation(self):
		pk = generate_keypair()
		assert len(pk) > 0
		assert isinstance(pk, str)

	def test_amount_creation(self):

		engine = PaymentEngine()
		amount = engine.create_amount(1000)
		assert amount.value == 1000

		with pytest.raises(ValueError):
			engine.create_amount(-100)

	def test_wallet_creation(self):
		
		engine = PaymentEngine()


		bank_wallet = engine.create_wallet("bank", "Bank of South Africa")
		assert str(bank_wallet) == "Bank of South Africa"
		assert bank_wallet.is_bank()

		
		crypto_wallet = engine.create_wallet("crypto", "0x123")
		assert "0x123" in str(crypto_wallet)
		assert crypto_wallet.is_crypto()

		with pytest.raises(ValueError):
			engine.create_wallet("invalid", "test")

		
	def test_transaction__creation(self):
		engine = PaymentEngine()

		from_wallet = engine.create_wallet("bank", "Sender Bank")
		to_wallet = engine.create_wallet("crypto", "0xRecipient")
		amount = engine.create_amount(500)

		tx = engine.create_transaction(
			
			"txn_001",
			from_wallet,
			to_wallet,
			amount,
			1
		)


		assert tx.id == "tx_001"
		assert str(tx.from_wallet) == "Sender Bank"
		assert str(tx.to_wallet) == "0xRecipient"
		assert tx.amount.value == 500
		assert tx.nonce == 1


		# Verify signature

		assert tx.verify_signature() is True

	def test_transaction_json(self):
		engine = PaymentEngine()

		from_wallet  = engine.create_wallet("bank", "Bank A")
		to_wallet = engine.create_wallet("crypto", "0xB")
		amount = engine.create_amount(1000)

		tx = engine.create_transaction(
			
			"txn_002",
			from_wallet,
			to_wallet,
			amount,

			2
		)

		import json
		
		data = json.loads(tx.to_json())


		assert data["id"] == "txn_002"
		assert data["from"] == "Bank A"
		assert data["to"] == "0xB"
		assert data["amount"] == 1000
		assert data["nonce"] == 2
		assert  "state" in data
