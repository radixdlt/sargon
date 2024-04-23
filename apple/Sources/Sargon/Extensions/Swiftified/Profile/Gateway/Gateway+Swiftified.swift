import SargonUniFFI
import Foundation

extension Gateway: SargonModel {}
extension Gateway: CustomStringConvertible {
	public var description: String {
		toString()
	}
}
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
        Self.forNetwork(id: .nebunet)
    }

    public static var kisharnet: Self {
        Self.forNetwork(id: .kisharnet)
    }

    public static var ansharnet: Self {
        Self.forNetwork(id: .ansharnet)
    }

    public static var hammunet: Self {
        Self.forNetwork(id: .hammunet)
    }

    public static var enkinet: Self {
        Self.forNetwork(id: .enkinet)
    }

    public static var mardunet: Self {
        Self.forNetwork(id: .mardunet)
    }

}
