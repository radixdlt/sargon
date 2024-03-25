public typealias GetIDPath = GetIdPath

extension GetIDPath: @unchecked Sendable {}
extension GetIDPath {
    public static let `default`: Self = defaultGetIdPath()
}

