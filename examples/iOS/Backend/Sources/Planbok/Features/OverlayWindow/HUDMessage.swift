import Foundation

public struct HUDMessage: Sendable, Hashable, Identifiable {
	public let id = UUID()
	public let text: String
	public let icon: Icon?

	public struct Icon: Hashable, Sendable {
		public let systemName: String
		public let foregroundColor: Color

		public init(
			systemName: String,
			foregroundColor: Color
		) {
			self.systemName = systemName
			self.foregroundColor = foregroundColor
		}

		public static let success = Self(
			systemName: "checkmark.circle.fill",
			foregroundColor: Color.app.green1
		)

		public static let fail = Self(
			systemName: "exclamationmark.triangle.fill",
			foregroundColor: Color.app.red1
		)
	}

	public init(
		text: String,
		icon: Icon?
	) {
		self.text = text
		self.icon = icon
	}

	public static func success(text: String, icon: Icon? = .success) -> Self {
		Self(text: text, icon: icon)
	}

	public static func failure(text: String, icon: Icon? = .fail) -> Self {
		Self(text: text, icon: icon)
	}
}
