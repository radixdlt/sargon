//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-01.
//

import Foundation
import SargonUniFFI

extension BagOfBytes: SargonModel {
	public static let sample: Self = newBagOfBytesSampleAced()
	public static let sampleOther: Self = newBagOfBytesSampleBabe()
}

extension RefBytes: SargonReferenceType {
	public typealias Inner = BagOfBytes
	
	public static func from(inner: Inner) -> Self {
		RefBytes(inner: inner) as! Self
	}
}
