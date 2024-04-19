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
	public static let sampleMainnet: Self = newPersonaSampleMainnetSatoshi()
	public static let sampleMainnetOther: Self = newPersonaSampleMainnetBatman()
	public static let sampleMainnetThird: Self = newPersonaSampleMainnetRipley()
	
	public static let sampleStokenet: Self = newPersonaSampleStokenetLeiaSkywalker()
	public static let sampleStokenetOther: Self = newPersonaSampleStokenetHermione()
	public static let sampleStokenetThird: Self = newPersonaSampleStokenetConnor()
}
#endif // DEBUG
