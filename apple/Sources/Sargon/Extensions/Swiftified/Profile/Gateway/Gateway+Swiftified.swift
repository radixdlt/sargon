import Foundation
import SargonUniFFI

// MARK: - Gateway + SargonModel
extension Gateway: SargonModel {}

// MARK: - Gateway + CustomStringConvertible
extension Gateway: CustomStringConvertible {
	public var description: String {
		toString()
	}
}

// MARK: - Gateway + Identifiable
extension Gateway: Identifiable {
	public typealias ID = URL
	public var id: ID {
		getID()
	}
}

extension Gateway {
	public var networkID: NetworkID {
		network.id
	}
}

extension Gateway {
	public static var nebunet: Self {
		forNetwork(id: .nebunet)
	}

	public static var kisharnet: Self {
		forNetwork(id: .kisharnet)
	}

	public static var ansharnet: Self {
		forNetwork(id: .ansharnet)
	}

	public static var hammunet: Self {
		forNetwork(id: .hammunet)
	}

	public static var enkinet: Self {
		forNetwork(id: .enkinet)
	}

	public static var mardunet: Self {
		forNetwork(id: .mardunet)
	}
}
