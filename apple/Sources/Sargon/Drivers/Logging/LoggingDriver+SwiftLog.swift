//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI
import Logging

public final actor Log {
	fileprivate let logger: Logging.Logger
	private init(label: String = "Sargon") {
		self.logger = Logger(label: label)
	}
	public static let shared = Log()
}

public var log: Log {
	Log.shared
}

extension Log: LoggingDriver {
	nonisolated public func trace(msg: String) {
		logger.trace(.init(stringLiteral: msg))
	}
	
	nonisolated public func debug(msg: String) {
		logger.debug(.init(stringLiteral: msg))
	}
	
	nonisolated public func info(msg: String) {
		logger.info(.init(stringLiteral: msg))
	}
	
	nonisolated public func warning(msg: String) {
		logger.warning(.init(stringLiteral: msg))
	}
	
	nonisolated public func error(msg: String) {
		logger.error(.init(stringLiteral: msg))
	}
	
	
}
