import UniformTypeIdentifiers
import SwiftUI

public struct WrappDocument: FileDocument {
    public let metadata: WrappMetadata

    public static var readableContentTypes: [UTType] { [WrappStorage.wrappType] }

    public init(configuration: ReadConfiguration) throws {
        guard
            let data = configuration.file.regularFileContents,
            let metadata = WrappMetadata(data: data)
        else {
            throw CocoaError(.fileReadCorruptFile)
        }
        self.metadata = metadata
    }

    public func fileWrapper(configuration: WriteConfiguration) throws -> FileWrapper {
        throw CocoaError(.featureUnsupported)
    }
}
