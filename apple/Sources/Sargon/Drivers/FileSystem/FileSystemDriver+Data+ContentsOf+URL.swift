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
         #if os(macOS)
		 guard url.startAccessingSecurityScopedResource() else {
		 	throw CommonError.NotPermissionToAccessFile(path: path)
		 }
		 defer { url.stopAccessingSecurityScopedResource() }
         #endif
		return try io(url)
	}
}

extension FileSystem: FileSystemDriver {
    public func writableAppDirPath() async throws -> String {
#if os(iOS)
        try fileManager.url(
            for: .cachesDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: true
        ).absoluteString
#elseif os(macOS)
        URL.temporaryDirectory.path()
#else
        fatalError("Unsupported OS")
#endif
    }
    
	public func loadFromFile(path: String) async throws -> BagOfBytes? {
        return try with(path: path) {
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
            if fileManager.fileExists(atPath: $0.path()) {
                if !isAllowedToOverwrite {
                    throw CommonError.FileAlreadyExists(path: path)
                }
            } else {
                fileManager.createFile(atPath: $0.path(), contents: nil)
            }
            try data.write(to: $0)
        }
    }
	
	public func deleteFile(path: String) async throws {
		try with(path: path) {
			try fileManager.removeItem(at: $0)
		}
	}
}
