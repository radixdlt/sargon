import Foundation
import SargonUniFFI

// MARK: - FileManager + @unchecked @retroactive Sendable
extension FileManager: @unchecked @retroactive Sendable {}

/// Makes it possible to type `.shared` on an initalizer/func taking
/// `some FileSystemDriver` as parameter.
extension FileSystemDriver where Self == FileSystem {
	/// Singleton `FileSystemDriver` of type `FileSystem`being an `actor`  which
	/// uses a `FileManager` for File I/O CRUD operations.
	public static var shared: Self {
		Self.shared
	}
}

// MARK: - FileSystem
/// `FileSystemDriver` being an `actor`  which
/// uses a `FileManager` for File I/O CRUD operations.
public final actor FileSystem {
	private let fileManager: FileManager
	public init(fileManager: FileManager = .default) {
		self.fileManager = fileManager
	}

	/// Singleton `FileSystemDriver` of type `FileSystem`being an `actor`  which
	/// uses a `FileManager` for File I/O CRUD operations.
	public static let shared = FileSystem(fileManager: .default)
}

extension URL {
	init(file string: String) {
		self.init(filePath: string, directoryHint: .notDirectory)
	}
}

extension FileSystem {
	private func with<T: Sendable>(
		path: String,
		_ io: @Sendable (URL) throws -> T
	) throws -> T {
		let url = URL(file: path)
		#if os(macOS)
		guard url.startAccessingSecurityScopedResource() else {
			throw CommonError.NotPermissionToAccessFile(path: path)
		}
		defer { url.stopAccessingSecurityScopedResource() }
		#endif
		return try io(url)
	}
}

extension FileSystem {
	private static func appDirPathNotNecessarilyExisting(fileManager: FileManager) throws -> String {
		#if os(iOS)
		fileManager.urls(
			for: .cachesDirectory,
			in: .userDomainMask
		).first!.path()
		#elseif os(macOS)
		URL.temporaryDirectory.path()
		#else
		fatalError("Unsupported OS")
		#endif
	}
}

#if DEBUG
extension FileSystem {
	public func deleteFactorInstancesCache() async throws {
		try with(path: Self.appDirPathNotNecessarilyExisting(fileManager: fileManager)) {
			let path = $0.appending(component: "radix_babylon_wallet_pre_derived_public_keys_cache.json")
			try self.fileManager.removeItem(at: path)
		}
	}
}
#endif

// MARK: - FileSystem + FileSystemDriver
extension FileSystem: FileSystemDriver {
	public func writableAppDirPath() async throws -> String {
		try with(path: Self.appDirPathNotNecessarilyExisting(fileManager: fileManager)) {
			let directoryExists = fileManager.fileExists(atPath: $0.path())
			if !directoryExists {
				try fileManager.createDirectory(at: $0, withIntermediateDirectories: true)
			}
			return $0.path()
		}
	}

	public func loadFromFile(path: String) async throws -> BagOfBytes? {
		try with(path: path) {
			let fileExists = fileManager.fileExists(atPath: $0.path())
			do {
				return try Data(contentsOf: $0)
			} catch {
				if fileExists {
					throw error
				} else {
					return nil
				}
			}
		}
	}

	public func saveToFile(path: String, data: BagOfBytes, isAllowedToOverwrite: Bool) async throws {
		try with(path: path) {
			try data.write(to: $0, options: isAllowedToOverwrite ? [] : [.withoutOverwriting])
		}
	}

	public func deleteFile(path: String) async throws {
		try with(path: path) {
			try fileManager.removeItem(at: $0)
		}
	}
}
