//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-01.
//

import Foundation
import SwiftUI


public struct FactorSourceCardView: SwiftUI.View {
	public let factorSource: FactorSource
	public var body: some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("Kind", factorSource.kind)
			Labeled("Added", factorSource.addedOn.formatted(.dateTime))
			Labeled("Last Used", factorSource.lastUsedOn.formatted(.dateTime))
			
			factorSource.hintView()
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
		.background(Color.orange)
		.padding()
	}
}

