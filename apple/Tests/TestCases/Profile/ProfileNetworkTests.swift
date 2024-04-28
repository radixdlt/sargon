import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class ProfileNetworkTests: Test<ProfileNetwork> {
	func test_auth_details() throws {
		let sut = SUT.sample
		let dapp = try XCTUnwrap(sut.authorizedDapps.first)
		let details = try sut.detailsForAuthorizedDapp(dapp)
		XCTAssertEqual(details.dappDefinitionAddress, dapp.dappDefinitionAddress)
	}
}
