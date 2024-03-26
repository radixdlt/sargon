extension Blobs {
	public init(_ blobs: [Blob]) {
		self = newBlobsFromBlobList(blobs: Array(blobs))
	}
    
    public var blobs: [Blob] {
        blobsListOfBlobs(blobs: self)
    }
}
