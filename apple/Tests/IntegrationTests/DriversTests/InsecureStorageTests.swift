import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class InsecureStorageDriverTests: DriverTest<Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage> {
    
    func test() async throws {
        let sut = SUT.init(keychainService: "test")
        let data = Data.sampleAced
        let key = SUT.Key.activeProfileId
        try await sut.saveData(key: key, data: data)
        let loaded = try await sut.loadData(key: key)
        XCTAssertEqual(loaded, data)
        try await sut.deleteDataForKey(key: key)
        let loadedAfterDelete = try await sut.loadData(key: key)
        XCTAssertNil(loadedAfterDelete)
    }
}
