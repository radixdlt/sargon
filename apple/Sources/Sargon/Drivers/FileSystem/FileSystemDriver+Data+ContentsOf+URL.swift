//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI

extension FileManager: @unchecked Sendable {}

extension FileSystemDriver where Self == FileSystem {
	public static var shared: Self { Self.shared }
}

public final actor FileSystem {
	private let fileManager: FileManager
	public init(fileManager: FileManager = .default) {
		self.fileManager = fileManager
	}
	public static let shared = FileSystem(fileManager: .default)
}

extension URL {
	init(file string: String) {
		self.init(filePath: string, directoryHint: .notDirectory)
	}
}

extension FileSystem: FileSystemDriver {
	
	public func loadFromFile(path: String) async throws -> BagOfBytes? {
		let url = URL(file: path)
		return try Data(contentsOf: url)
	}
	
	public func saveToFile(path: String, data: BagOfBytes) async throws {
		let url = URL(file: path)
		try data.write(to: url)
	}
	
	public func deleteFile(path: String) async throws {
		let url = URL(file: path)
		try fileManager.removeItem(at: url)
	}
}
