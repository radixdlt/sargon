//
//  HDPathComponentModels.swift.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-24.
//


extension HdPathComponent: CustomDebugStringConvertible {
	public var debugDescription: String {
		toBIP32String()
	}
}

extension HdPathComponent: CustomStringConvertible {
	public var description: String {
		toBIP32StringDebug()
	}
}
extension HdPathComponent {
	public func toBIP32String() -> String {
		hdPathComponentToBip32String(component: self)
	}
	public func toBIP32StringDebug() -> String {
		hdPathComponentToBip32StringDebug(component: self)
	}

	public init(globalKeySpace: UInt32) {
		self = newHdPathComponentFromGlobalKeySpace(value: globalKeySpace)
	}
	
	public init(localKeySpace: UInt32, keySpace: KeySpace) throws {
		self = try newHdPathComponentFromLocalKeySpace(value: localKeySpace, keySpace: keySpace)
	}
	
	public var keySpace: KeySpace {
		hdPathComponentGetKeySpace(component: self)
	}
	
	public func indexInGlobalKeySpace() -> UInt32 {
		hdPathComponentIndexInGlobalKeySpace(component: self)
	}
	
	public func indexInLocalKeySpace() -> UInt32 {
		hdPathComponentIndexInLocalKeySpace(component: self)
	}
}


extension U31 {
    public init(value: UInt32) throws {
        self = try newU31(value: value)
    }
    public var value: UInt32 {
        u31GetValue(u31: self)
    }
}


extension U30 {
    public init(value: UInt32) throws {
        self = try newU30(value: value)
    }
    public var value: UInt32 {
        u30GetValue(u30: self)
    }
}

extension UnsecurifiedHardened {
    public init(u30: U30) {
        self = newUnsecurifiedHardened(u30: u30)
    }
    public init(localKeySpace: UInt32) throws {
        self = try newUnsecurifiedHardenedFromLocalKeySpace(value: localKeySpace)
    }
    
    public init(globalKeySpace: UInt32) throws {
        self = try newUnsecurifiedHardenedFromGlobalKeySpace(value: globalKeySpace)
    }
    public func indexInLocalKeySpace() -> UInt32 {
        unsecurifiedHardenedIndexInLocalKeySpace(unsecurifiedHardened: self)
    }
    public func indexInGlobalKeySpace() -> UInt32 {
        unsecurifiedHardenedIndexInGlobalKeySpace(unsecurifiedHardened: self)
    }
}
extension SecurifiedU30 {
    public init(u30: U30) {
        self = newSecurified(u30: u30)
    }
    public init(localKeySpace: UInt32) throws {
        self = try newSecurifiedFromLocalKeySpace(value: localKeySpace)
    }
    
    public init(globalKeySpace: UInt32) throws {
        self = try newSecurifiedFromGlobalKeySpace(value: globalKeySpace)
    }
    public func indexInLocalKeySpace() -> UInt32 {
        securifiedIndexInLocalKeySpace(securified: self)
    }
    public func indexInGlobalKeySpace() -> UInt32 {
        securifiedIndexInGlobalKeySpace(securified: self)
    }
}
extension Unhardened {
    public init(u31: U31) {
        self = newUnhardened(u31: u31)
    }
    public init(localKeySpace: UInt32) throws {
        self = try newUnhardenedFromLocalKeySpace(value: localKeySpace)
    }
    
    public init(globalKeySpace: UInt32) throws {
        self = try newUnhardenedFromGlobalKeySpace(value: globalKeySpace)
    }
    public func indexInLocalKeySpace() -> UInt32 {
        unhardenedIndexInLocalKeySpace(unhardened: self)
    }
    public func indexInGlobalKeySpace() -> UInt32 {
        unhardenedIndexInGlobalKeySpace(unhardened: self)
    }
}
