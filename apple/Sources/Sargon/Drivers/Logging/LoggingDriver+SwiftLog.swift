//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI
import os

extension LoggingDriver where Self == Log {
	public static var shared: Self { Self.shared }
}

public final actor Log {
	nonisolated fileprivate let logger: Logger

	private init(
		subsystem: String = "Sargon",
		category: String = ""
	) {
		self.logger = Logger(
			subsystem: subsystem,
			category: category
		)
	}
	
	public static let shared = Log()

}

extension Log: LoggingDriver {
	
	nonisolated public func log(level: LogLevel, msg: String) {
		logger.log(
			level: .init(sargonLogLevel: level),
			"\(msg)"
		)
	}
}

/// Makes it possible for iOS Wallet to later change the log level in Rust land
/// (remember, the Rust logger **uses the Swift logger**
/// but might suppress logging invocation if its logging facade's log level is too low.)
public func setLogLevel(_ level: Sargon.LogLevel) {
	rustLoggerSetLevel(level: level)
}

public var log: Logger {
	Log.shared.logger
}

extension Logger: @unchecked Sendable {}

extension OSLogType {
	init(sargonLogLevel sargon: Sargon.LogLevel) {
		switch sargon {
		case .trace, .debug: self = .debug
		case .info: self = .info
		case .warn: self = .debug
		case .error: self = .error
		}
	}
}
