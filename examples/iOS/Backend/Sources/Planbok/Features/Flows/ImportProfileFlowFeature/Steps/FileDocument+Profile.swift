import ComposableArchitecture
import Sargon
import SwiftUI
import UniformTypeIdentifiers

// MARK: - NoJSONDataFound
struct NoJSONDataFound: Error {}

// MARK: - FileContentIsNotProfile
struct FileContentIsNotProfile: LocalizedError {
	var errorDescription: String? {
		"Invalid backup file."
	}
}

// MARK: - ExportableProfileFile
/// An exportable (and thus importable) Profile file, either encrypted or plaintext.
public enum ExportableProfileFile: FileDocument, Sendable, Hashable {
	case plaintext(Profile)
	case encrypted(Data)
}


extension ExportableProfileFile {
	public static let readableContentTypes: [UTType] = [.profile]

	public init(configuration: ReadConfiguration) throws {
		guard let data = configuration.file.regularFileContents
		else {
			throw NoJSONDataFound()
		}
		try self.init(data: data)
	}

	public init(data: Data) throws {
		switch Profile.analyzeContents(data: data) {
		case .encryptedProfile:
			self = .encrypted(data)
		case .notProfile:
			throw FileContentIsNotProfile()
		case let .plaintextProfile(plaintextProfile):
			self = .plaintext(plaintextProfile)
		}
	}

	public func fileWrapper(configuration: WriteConfiguration) throws -> FileWrapper {
		@Dependency(\.encode) var encode

		switch self {
		case let .plaintext(plaintext):
			let jsonData = plaintext.jsonData()
			return FileWrapper(regularFileWithContents: jsonData)
		case let .encrypted(encryptedSnapshot):
			return FileWrapper(regularFileWithContents: encryptedSnapshot)
		}
	}
}
