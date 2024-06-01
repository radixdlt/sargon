//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-01.
//

import Foundation
import SwiftUI

public protocol FactorSourceHint {
	func display() -> any SwiftUI.View
}

public protocol DisplayableFactorSource: FactorSourceProtocol & Identifiable where ID: Sendable {
	associatedtype Hint: FactorSourceHint
	var hint: Hint { get }
}

public struct FactorSourceCardView<F: DisplayableFactorSource>: SwiftUI.View {
	public let factorSource: F
	public var body: some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("Kind", factorSource.kind)
			Labeled("Added", factorSource.addedOn.formatted(.dateTime))
			Labeled("Last Used", factorSource.lastUsedOn.formatted(.dateTime))
			
			AnyView(factorSource.hint.display())
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
		.background(Color.orange)
		.padding()
	}
}

