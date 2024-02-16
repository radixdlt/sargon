extension AppearanceID {
	public var gradient: LinearGradient {
		LinearGradient(appearanceID: self)
	}
}


extension LinearGradient {
	public init(appearanceID: AppearanceID) {
		self.init(
			gradient: Gradient(appearanceID: appearanceID),
			startPoint: .leading,
			endPoint: .trailing
		)
	}
}




extension LinearGradient {
	/// Namespace only
	public struct App { fileprivate init() {} }
	/// Namespace containing app-specific linear gradients
	public static let app = App()
}

extension Gradient {
	public init(appearanceID: AppearanceID) {
		self.init(colors: Self.colors(for: appearanceID.value))
	}

	private static func colors(for index: UInt8) -> [Color] {
		colors[Int(index) % colors.count]
	}

	private static let colors: [[Color]] = [
		[.app.blue2, .app.account0green],
		[.app.blue2, .app.account1pink],
		[.app.blue2, .app.blue3],
		[.app.green1, .app.blue2],
		[.app.account4pink, .app.blue2],
		[.app.account5blue, .app.blue2],
		[.app.gray1, .app.account6green],
		[.app.gray1, .app.account7pink],
		[.app.blue2, .app.gray1],
		[.app.account9green1, .app.account9green2],
		[.app.account10pink1, .app.account10pink2],
		[.app.account11green, .app.account11blue1, .app.account11pink],
	]
}
