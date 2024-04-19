import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class Exactly29BytesTests: ExactlyNBytesTest<Exactly29Bytes> {}
import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class Exactly32BytesTests: ExactlyNBytesTest<Exactly32Bytes> {
	func test_from_array_literal() {
		let sut: SUT = [0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad]
		XCTAssertNoDifference(sut, SUT.sample)
	}
}
import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class Exactly33BytesTests: ExactlyNBytesTest<Exactly33Bytes> {}
import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class Exactly64BytesTests: ExactlyNBytesTest<Exactly64Bytes> {}
import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class Exactly65BytesTests: ExactlyNBytesTest<Exactly65Bytes> {}
