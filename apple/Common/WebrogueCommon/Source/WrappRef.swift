public struct WrappRef {
    public let path: String
    public let metadata: WrappMetadata

    public init(path: String, metadata: WrappMetadata) {
        self.path = path
        self.metadata = metadata
    }
}
