//
//  HDPathComponentModels.swift.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-24.
//

public protocol BaseHDPathComponentProtocol: SargonModel {
    init(globalKeySpace: UInt32) throws
    func indexInLocalKeySpace() -> UInt32
    func indexInGlobalKeySpace() -> UInt32
}

public protocol HDPathComponentProtocol: BaseHDPathComponentProtocol {
	static var globalOffset: UInt32 { get }
    init(localKeySpace: UInt32) throws
}

