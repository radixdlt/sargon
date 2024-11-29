import Foundation
import SargonUniFFI

#if DEBUG
extension Blob {
	public static let sample = Self(data: BagOfBytes.sampleAced)

	public static let sampleOther = try! Self(
		data: Data(
			hex: String(repeating: "deadbeefabbafadecafe", count: 100)
		)
	)
}
#endif //  DEBUG
