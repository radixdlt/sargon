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
	nonisolated fileprivate let rustLogger: Logger
	
	// internal for testing purposes
	nonisolated internal let swiftLogger: Logger

	private init(
		subsystem: String = "Sargon",
		rustCategory: String = "Rust",
		swiftCategory: String = "Swift"
	) {
		self.rustLogger = Logger(
			subsystem: subsystem,
			category: rustCategory
		)
		self.swiftLogger = Logger(
			subsystem: subsystem,
			category: swiftCategory
		)
	}
	
	public static let shared = Log()

}

extension Log: LoggingDriver {
	nonisolated public func log(
		level: LogLevel,
		msg: String
	) {
		rustLogger.log(
			level: .init(sargonLogLevel: level),
			"\(msg)"
		)
	}
}

/// Makes it possible for iOS Wallet to later change the log level in Rust land
/// (remember, the Rust logger **uses the Swift logger**
/// but might suppress logging invocation if its logging facade's log level is too low.)
public func setRustLogLevel(_ level: Sargon.LogFilter) {
	rustLoggerSetLevel(level: level)
}

/// Makes it possible for iOS Wallet to later change the log level in Rust land
/// (remember, the Rust logger **uses the Swift logger**
/// but might suppress logging invocation if its logging facade's log level is too low.)
public func getRustLogLevel() -> Sargon.LogFilter {
	rustLoggerGetLevel()
}

public func logSystemDiagnosis() {
	let levels = LogLevel.allCases
	print("logSystemDiagnosis - RUST")
	rustLoggerLogAtEveryLevel()
	print("logSystemDiagnosis - Swift")
	levels.forEach { level in
		log.log(level: .init(sargonLogLevel: level), "Swift test: '\(String(describing: level))'")
	}
}

public var log: Logger {
	Log.shared.swiftLogger
}

extension LogFilter: CaseIterable {
	public static let allCases: [Self] = rustLoggerGetAllFilters()
}

extension LogLevel: CaseIterable {
	public static let allCases: [Self] = rustLoggerGetAllLevels()
}

extension Logger: @unchecked Sendable {}

extension OSLogType {
	
	/// Rust has 5 log levels, so does Swift.
	///
	/// The mapping might look a bit strange since we do NOT map `error` -> `error`,
	/// neither do we map `debug` -> `debug`, instead we map the most serious Rust
	/// log level to the most serious Swift log level, and the least serious Rust to least
	/// serious to Swift.
	init(sargonLogLevel sargon: Sargon.LogLevel) {
		switch sargon {
		case .error: 
			// yes this is correct we dont map `error` -> `error`.
			self = .fault
		case .warn:
			// Swift does not have warn, we use error, and we use Swifts fault for Rust error.
			self = .error
		case .info: self = .info
		case .debug: 
			// yes this is correct we dont map `debug` -> `debug`.
			self = .default
		case .trace:
			// debug is Swifts least serious, and `trace` is Rust least serious.
			self = .debug
		}
	}
}


extension OSLogType {
	
	init(sargonFilterLevel sargon: Sargon.LogFilter) {
		if let level = LogLevel(rawValue: sargon.rawValue) {
			self.init(sargonLogLevel: level)
		} else {
			self = .fault
		}
	}
}

