import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import Testing

@Suite("ShieldBuilder")
struct ShieldTests {

    @Test("Get/Set name")
    func name() {
        let builder = SecurityShieldBuilder()
        #expect(builder.name == "My Shield")
        builder.name = "S.H.I.E.L.D"
        #expect(builder.name == "S.H.I.E.L.D")
    }
    
    
    @Test("Get/Set Threshold")
    func threshold() {
        let builder = SecurityShieldBuilder()
        #expect(builder.threshold == 0)
        #expect(throws: CommonError.RoleMustHaveAtLeastOneFactor) {
            try builder.setThreshold(threshold: 0)
        }
    }
}
