import Foundation

// MARK: - LabelStyleFlip
public struct LabelStyleFlip: LabelStyle {
	let imageColor: Color
	public func makeBody(configuration: Configuration) -> some View {
		HStack(alignment: .center) {
			configuration.title
			configuration.icon.foregroundStyle(imageColor)
		}
	}
}

extension LabelStyle where Self == LabelStyleFlip {
	public static func flipped(imageColor: Color = .gray) -> Self {
		LabelStyleFlip(imageColor: imageColor)
	}
}
