// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "ServiceManagementBridge",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .library(
            name: "ServiceManagementBridge",
            type: .static,
            targets: ["ServiceManagementBridge"])
    ],
    targets: [
        .target(
            name: "ServiceManagementBridge",
            path: "Sources/ServiceManagementBridge",
            publicHeadersPath: "include")
    ]
)
