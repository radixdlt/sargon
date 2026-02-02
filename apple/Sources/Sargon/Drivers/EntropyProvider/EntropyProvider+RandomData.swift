import Foundation
import SargonUniFFI

/// Makes it possible to type `.shared` on an initalizer/func taking
/// `some EntropyProviderDriver` as parameter.
extension EntropyProviderDriver where Self == EntropyProvider {
	/// Singleton `EntropyProviderDriver` of type `EntropyProvider`,
	/// being an `actor` that uses CSRNG `SystemRandomNumberGenerator`
	public static var shared: Self {
		Self.shared
	}
}

// MARK: - EntropyProvider
/// An `EntropyProviderDriver` actor which uses CSRNG `SystemRandomNumberGenerator`
/// to generate 32 bytes.
public final actor EntropyProvider {
	init() {}

	/// Singleton `EntropyProviderDriver` of type `EntropyProvider`,
	/// being an `actor` that uses CSRNG `SystemRandomNumberGenerator`
	public static let shared = EntropyProvider()
}

// MARK: EntropyProviderDriver
extension EntropyProvider: EntropyProviderDriver {
	/// Generates 32 bytes using CSRNG `SystemRandomNumberGenerator`
	public nonisolated func generateSecureRandomBytes() -> Entropy32Bytes {
		Entropy32Bytes.generate()
	}
}
