extension SecureStorageKey {
	public var identifier: String {
		secureStorageKeyIdentifier(key: self)
	}
}
