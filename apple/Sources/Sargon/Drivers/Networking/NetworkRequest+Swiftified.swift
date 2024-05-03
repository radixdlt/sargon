//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI

extension URLRequest {
	init(sargon: NetworkRequest) {
		var request = URLRequest(url: sargon.url)
		switch sargon.method {
		case .post:
			request.httpMethod = "POST"  // FIXME: embed in sargon
		case .get:
			request.httpMethod = "GET"
		}

		request.httpBody = sargon.body
		request.allHTTPHeaderFields = sargon.headers
		self = request
	}
}


