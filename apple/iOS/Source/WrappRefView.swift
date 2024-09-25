//
//  WrappRefView.swift
//  Launcher iOS
//
//  Created by Artem on 25.09.24.
//

import SwiftUI
import WebrogueCommon

struct WrappRefView: View {
    let ref: WrappRef

    var body: some View {
        VStack {
            Text("Path: \(ref.path)")
            Text("SHA256: \(ref.metadata.sha256)")
                .lineLimit(1)
        }
            .navigationBarTitleDisplayMode(.inline)
            .navigationTitle("webrogue")
            .toolbar {
                Button("Run") {
                    WebrogueAppDelegate.shared?.runGame(path: ref.path) { _ in }
                }
            }
    }
}
