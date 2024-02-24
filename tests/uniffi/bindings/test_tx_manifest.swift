import Sargon

extension TransactionManifest {
	static let placeholder: Self = newTransactionManifestPlaceholder()
	static let placeholderOther: Self = newTransactionManifestPlaceholderOther()
}
extension TransactionManifest: CustomStringConvertible {
	public var description: String {
		transactionManifestToString(manifest: self)
	}
}
public typealias Blob = BagOfBytes
public typealias Blobs = [Blob]
extension TransactionManifest {
	public init(instructionsString: String, networkID: NetworkId, blobs: Blobs = []) throws {
		self = try newTransactionManifestFromInstructionsStringAndBlobs(
			instructionsString: instructionsString, networkId: networkID, blobs: blobs)
	}
}

func test() throws {
	print("🚀 Test TransactionManifest in Swift start")
	defer { print("✅ Test TransactionManifest in Swift completed ") }
	assert(TransactionManifest.placeholder == TransactionManifest.placeholder)
	assert(TransactionManifest.placeholderOther == TransactionManifest.placeholderOther)
	assert(TransactionManifest.placeholder != TransactionManifest.placeholderOther)

	let instructionsString = """
		CALL_METHOD
		    Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q")
		    "lock_fee"
		    Decimal("500")
		;
		CALL_METHOD
		    Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q")
		    "withdraw"
		    Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
		    Decimal("100")
		;
		CALL_METHOD
		    Address("account_sim1cyzfj6p254jy6lhr237s7pcp8qqz6c8ahq9mn6nkdjxxxat5syrgz9")
		    "try_deposit_batch_or_abort"
		    Expression("ENTIRE_WORKTOP")
		    Enum<0u8>()
		;

		"""
	print("🔮 🔮 🔮 🔮 🔮 ")
	print(TransactionManifest.placeholder.description)
	print("✨ ✨ ✨ ✨ ✨")
	assert(TransactionManifest.placeholder.description == instructionsString)
	let sut = try TransactionManifest(
		instructionsString: instructionsString, networkID: .simulator)
	assert(sut == TransactionManifest.placeholder)

}

try! test()
