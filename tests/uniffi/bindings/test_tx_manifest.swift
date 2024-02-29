import Sargon

extension TransactionManifest {
	static let sample: Self = newTransactionManifestSample()
	static let sampleOther: Self = newTransactionManifestSampleOther()
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
	assert(TransactionManifest.sample == TransactionManifest.sample)
	assert(TransactionManifest.sampleOther == TransactionManifest.sampleOther)
	assert(TransactionManifest.sample != TransactionManifest.sampleOther)

	let instructionsString = """
		CALL_METHOD
		    Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
		    "lock_fee"
		    Decimal("0.61")
		;
		CALL_METHOD
		    Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
		    "withdraw"
		    Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
		    Decimal("1337")
		;
		TAKE_FROM_WORKTOP
		    Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
		    Decimal("1337")
		    Bucket("bucket1")
		;
		CALL_METHOD
		    Address("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
		    "try_deposit_or_abort"
		    Bucket("bucket1")
		    Enum<0u8>()
		;

		"""
	print("🔮 🔮 🔮 🔮 🔮 ")
	print(TransactionManifest.sample.description)
	print("✨ ✨ ✨ ✨ ✨")
	assert(TransactionManifest.sample.description == instructionsString)
	let sut = try TransactionManifest(
		instructionsString: instructionsString, 
		networkID: .mainnet
	)
	assert(sut == TransactionManifest.sample)

}

try! test()
