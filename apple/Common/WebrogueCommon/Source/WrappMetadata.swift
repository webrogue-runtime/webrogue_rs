import CryptoKit
import Foundation

public class WrappMetadata {
    public let sha256: String

    public init?(data: Data) {
        guard data.subdata(in: 0..<6) == "WRAPP\0".data(using: .ascii) else {
            return nil
        }
        let sha = SHA256.hash(data: data)
        self.sha256 = sha.map { i64 in
            String(i64, radix: 16)
        }.joined()
    }
}
