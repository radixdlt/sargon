import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class BlobsTests: Test<Blobs> {
    func test_blobs() {
        XCTAssertEqual(
            SUT.sample.blobs,
            [
                Blob(data: Data.sampleAced),
                Blob(data: Data.sampleBabe),
                Blob(data: Data.sampleCafe),
                Blob(data: Data.sampleDead)
            ])
    }
}
