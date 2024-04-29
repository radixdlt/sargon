//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension Persona {
	
	public static let sampleMainnetSatoshi: Self = newPersonaSampleMainnetSatoshi()
	public static let sampleMainnetBatman: Self = newPersonaSampleMainnetBatman()
	public static let sampleMainnetRipley: Self = newPersonaSampleMainnetRipley()
	public static let sampleMainnetTuring: Self = newPersonaSampleMainnetTuring()
	
	public static let sampleMainnet: Self = .sampleMainnetSatoshi
	public static let sampleMainnetOther: Self = .sampleMainnetBatman
	public static let sampleMainnetThird: Self = .sampleMainnetRipley
	public static let sampleMainnetForth: Self = .sampleMainnetTuring
	
	public static let sampleStokenetSkywalker: Self = newPersonaSampleStokenetLeiaSkywalker()
	public static let sampleStokenetGranger: Self = newPersonaSampleStokenetHermione()
	public static let sampleStokenetConnor: Self = newPersonaSampleStokenetConnor()
	
	public static let sampleStokenet: Self = .sampleStokenetSkywalker
	public static let sampleStokenetOther: Self = .sampleStokenetGranger
	public static let sampleStokenetThird: Self = .sampleStokenetConnor
}
#endif // DEBUG
