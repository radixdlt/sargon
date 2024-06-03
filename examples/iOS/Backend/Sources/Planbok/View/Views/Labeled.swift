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
    public let axis: Axis
 
    public init<V>(
        _ title: String,
        _ value: V,
        axis: Axis = .horizontal
    ) where V: CustomStringConvertible {
        self.axis = axis
        self.title = title
		self.value = value.description
	}
    
	public var body: some SwiftUI.View {
        if axis == .horizontal {
            HStack {
                content()
            }
        } else {
            VStack(alignment: .leading) {
                content()
            }
        }
    }
    
    @ViewBuilder
    private func content() -> some SwiftUI.View {
        Text("**\(title)**")
        Text("`\(value)`")
            .multilineTextAlignment(.leading)
    }
}
