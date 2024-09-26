import Foundation
import SargonUniFFI

extension URL {
	public func isVectorImage(type: VectorImageType) -> Bool {
		imageUrlUtilsIsVectorImage(url: self.absoluteString, imageType: type)
	}

	public func imageURL(imageServiceURL: URL, size: CGSize) throws -> URL {
		try imageUrlUtilsMakeImageUrl(
			url: self.absoluteString,
			imageServiceUrl: imageServiceURL.absoluteString,
			width: UInt32(size.width),
			height: UInt32(size.height)
		)
	}
}
