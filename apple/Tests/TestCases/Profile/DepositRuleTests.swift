import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class DepositRuleTests: Test<DepositRule> {
    
    
    func test_low_level_to_json_string() {
        let sut = SUT.sample
        let jsonString = depositRuleToJsonString(rule: sut)
        XCTAssertEqual(jsonString, "acceptKnown")
    }
    
    

    func test_codable() throws {
        let raw = "\"denyAll\"".data(using: .utf8)!
        
        // test decoding
        let sut = try JSONDecoder().decode(SUT.self, from: raw)
        XCTAssertEqual(sut, SUT.denyAll)
        
        // test encoding
        let encoded = try JSONEncoder().encode(sut)
        try XCTAssertEqual(JSONDecoder().decode(SUT.self, from: encoded), sut)
    }
  
    func test_wrapped_in_obj() throws {
        struct Wrapper: Codable, Equatable {
            let myString: String
            let rule: DepositRule
        }
        let json = """
        {
            "myString": "Foo",
            "rule": "acceptAll"
        }
        """.data(using: .utf8)!
        
        let decoded = try JSONDecoder().decode(Wrapper.self, from: json)
        XCTAssertEqual(decoded, Wrapper.init(myString: "Foo", rule: .acceptAll))
    }
}
