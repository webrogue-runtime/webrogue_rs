package io.github.webrogue_runtime.wrapp

import java.io.File

class WrappRef(val sha: String, val path: String) {
    fun delete() {
        File(path).delete()
    }
}