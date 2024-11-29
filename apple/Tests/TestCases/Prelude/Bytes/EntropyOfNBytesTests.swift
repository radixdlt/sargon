import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

// MARK: - Entropy16BytesTests
final class Entropy16BytesTests: ExactlyNBytesTest<Entropy16Bytes> {}

// MARK: - Entropy20BytesTests
final class Entropy20BytesTests: ExactlyNBytesTest<Entropy20Bytes> {}

// MARK: - Entropy24BytesTests
final class Entropy24BytesTests: ExactlyNBytesTest<Entropy24Bytes> {}

// MARK: - Entropy28BytesTests
final class Entropy28BytesTests: ExactlyNBytesTest<Entropy28Bytes> {}

// MARK: - Entropy32BytesTests
final class Entropy32BytesTests: ExactlyNBytesTest<Entropy32Bytes> {}
