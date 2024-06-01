//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-01.
//

import Foundation
import SwiftUI


public struct Labeled: SwiftUI.View {
	public let title: String
	public let value: String
	public init<V>(_ title: String, _ value: V) where V: CustomStringConvertible {
		self.title = title
		self.value = value.description
	}
	public var body: some SwiftUI.View {
		HStack {
			Text("**\(title)**")
			Text("`\(value)`")
		}
		.multilineTextAlignment(.leading)
	}
}
