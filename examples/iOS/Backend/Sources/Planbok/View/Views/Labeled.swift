import Foundation
import SwiftUI

public struct Labeled: SwiftUI.View {
	public let title: String
	public let value: String
	public let axis: Axis

	public init(
		_ title: String,
		_ value: some CustomStringConvertible,
		axis: Axis = .horizontal
	) {
		self.axis = axis
		self.title = title
		self.value = value.description
	}

	public var textTitle: some SwiftUI.View {
		Text("**\(title)**")
	}

	public var textValue: some SwiftUI.View {
		Text("`\(value)`")
	}

	public var body: some SwiftUI.View {
		Group {
			if axis == .horizontal {
				HStack {
					textTitle
					textValue
					Spacer()
				}
			} else {
				VStack(alignment: .leading) {
					HStack {
						textTitle
						Spacer()
					}
					.frame(maxWidth: .infinity)
					HStack {
						textValue
						Spacer()
					}
					.frame(maxWidth: .infinity)
				}
			}
		}
		.multilineTextAlignment(.leading)
		.padding(.horizontal, 10)
	}
}
