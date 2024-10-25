//
//  HDPathComponentTests.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-24.
//

import Sargon
import Testing

@Suite
struct HDPathComponentTests {
    typealias Sut = HdPathComponent
   
    @Test("Local roundtrip securified")
    func local_roundtrip_securified() throws {
        for local in UInt32(0)...3 {
            let sut = try Sut(indexInLocalKeySpace: local, keySpace: .securified)
            #expect(local == sut.indexInLocalKeySpace())
        }
    }
    

    
    
   
}
