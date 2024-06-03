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
			Labeled("Kind",  factorSource.kind)
			Labeled("ID", factorSource.id.description.suffix(8))
			Labeled("Added", factorSource.addedOn.formatted(.dateTime))
			Labeled("Last Used", factorSource.lastUsedOn.formatted(.dateTime))
			Labeled("Main?", factorSource.common.flags.contains(.main) ? "TRUE" : "FALSE")
			
			factorSource.hintView()
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
		.background(Color.orange)
		.padding()
	}
}

