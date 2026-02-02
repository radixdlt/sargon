import Foundation
import os
import SargonUniFFI

/// A public globally accessible `Logger` with `category` "Swift" which
/// Swift Sargon uses, and which iOS wallet can use too, it uses the
/// `Log.shared.swiftLogger`.
public var log: Logger {
	Log.shared.swiftLogger
}

/// Makes it possible to type `.shared` on an initalizer/func taking
/// `some LoggingDriver` as parameter.
extension LoggingDriver where Self == Log {
	public static var shared: Self {
		Self.shared
	}
}

// MARK: - Log
/// A `LoggingDriver` actor capable of logging on behalf of
/// Rust Sargon core, that is, when we write e.g. `debug!("hey Swift from Rust");`
/// in Rust code, that message will in fact be logged by a `os.Logger` held by this `Log`
/// actor.
public final actor Log {
	/// The `Logger` to which Rust delegates logged messages.
	private nonisolated let rustLogger: Logger

	/// The `Logger` Swift Sargon uses to log messages, accessed
	/// using global variable `log` (aliasa for `Log.shared.swiftLogger`).
	nonisolated let swiftLogger: Logger

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

	/// LoggingDriver singleton, a shared actor.
	public static let shared = Log()
}

// MARK: LoggingDriver
extension Log: LoggingDriver {
	public nonisolated func log(
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

/// This logging diagnos will tell Rust to log messages at every log level,
/// then it will log at each level using the "Swift logger" (`log`) as well,
/// this is useful from DEBUG menus to ensure logging works properly.
///
/// You can adjust the used Log Level in Rust by calling
/// `setRustLogLevel` and then call this method again.
public func logSystemDiagnosis() {
	let levels = LogLevel.allCases
	print("logSystemDiagnosis - RUST")
	rustLoggerLogAtEveryLevel()
	print("logSystemDiagnosis - Swift")
	for level in levels {
		log.log(level: .init(sargonLogLevel: level), "Swift test: '\(String(describing: level))'")
	}
}

// MARK: - LogFilter + CaseIterable
extension LogFilter: CaseIterable {
	public static let allCases: [Self] = rustLoggerGetAllFilters()
}

// MARK: - LogLevel + CaseIterable
extension LogLevel: CaseIterable {
	public static let allCases: [Self] = rustLoggerGetAllLevels()
}

// MARK: - Logger + @unchecked Sendable
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
