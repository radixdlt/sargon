

public protocol AddressFormatProtocol: Sendable {
	static var `default`: Self { get }
}
public protocol AddressProtocol: Sendable {
	associatedtype Format: AddressFormatProtocol
	func formatted(_ format: Format) -> String
}

// MARK: - AddressView
public struct AddressView<Address: AddressProtocol>: SwiftUI.View {
	public let address: Address
	public let format: Address.Format
	public init(_ address: Address, format: Address.Format = .default) {
		self.address = address
		self.format = format
	}

	@ViewBuilder
	public var body: some View {
		Text("\(address.formatted(format))")
			.lineLimit(nil)
			.multilineTextAlignment(.leading)
			.minimumScaleFactor(0.5)
	}
	
}
