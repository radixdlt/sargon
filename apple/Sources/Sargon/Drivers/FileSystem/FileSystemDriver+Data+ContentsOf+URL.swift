//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI

extension FileManager: @unchecked Sendable {}

// Makes it possible to type `.shared` on an initalizer/func taking
// `some FileSystemDriver` as parameter.
extension FileSystemDriver where Self == FileSystem {
	
	/// Singleton `FileSystemDriver` of type `FileSystem`being an `actor`  which
	/// uses a `FileManager` for File I/O CRUD operations.
	public static var shared: Self { Self.shared }
}

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
		guard url.startAccessingSecurityScopedResource() else {
			throw CommonError.NotPermissionToAccessFile(path: path)
		}
		defer { url.stopAccessingSecurityScopedResource() }
		return try io(url)
	}
}

extension FileSystem: FileSystemDriver {
	
	public func loadFromFile(path: String) async throws -> BagOfBytes? {
		try with(path: path) {
			try Data(contentsOf: $0)
		}
	}
	
	public func saveToFile(path: String, data: BagOfBytes) async throws {
		try with(path: path) {
			try data.write(to: $0)
		}
	}
	
	public func deleteFile(path: String) async throws {
		try with(path: path) {
			try fileManager.removeItem(at: $0)
		}
	}
}
