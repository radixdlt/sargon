import Sargon
import SargonUniFFI

// MARK: - AddressView
public struct AddressView<Address: AddressProtocol>: SwiftUI.View {
	public let address: Address
	public let format: AddressFormat
	public init(_ address: Address, format: AddressFormat = .default) {
		self.address = address
		self.format = format
	}

	public var body: some View {
		Text("\(address.formatted(format))")
			.lineLimit(nil)
			.multilineTextAlignment(.leading)
			.minimumScaleFactor(0.5)
	}
}
