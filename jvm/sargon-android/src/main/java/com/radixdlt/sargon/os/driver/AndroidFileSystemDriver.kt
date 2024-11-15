package com.radixdlt.sargon.os.driver

import android.content.Context
import android.net.Uri
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.FileSystemDriver
import com.radixdlt.sargon.extensions.logFailure
import com.radixdlt.sargon.extensions.toBagOfBytes
import com.radixdlt.sargon.extensions.toByteArray
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import okio.buffer
import okio.source
import java.io.File

class AndroidFileSystemDriver(
    private val context: Context,
    private val dispatcher: CoroutineDispatcher = Dispatchers.IO
): FileSystemDriver {

    override suspend fun loadFromFile(path: String): BagOfBytes? = withContext(dispatcher) {
        runCatching {
            context.contentResolver.openInputStream(path.toUri())?.use { stream ->
                stream.source().buffer().readByteArray()
            }
        }.logFailure().getOrNull()?.toBagOfBytes()
    }

    override suspend fun writableAppDirPath(): String = directory.absolutePath

    override suspend fun saveToFile(path: String, data: BagOfBytes, isAllowedToOverwrite: Boolean) {
        withContext(dispatcher) {
            runCatching {
                val fileToSave = path.toFile()

                if (!fileToSave.exists()) {
                    if (fileToSave.parentFile?.exists() == false) {
                        fileToSave.parentFile?.mkdirs()
                    }
                    fileToSave.createNewFile()
                } else if (!isAllowedToOverwrite) {
                    throw CommonException.FileAlreadyExists(
                        path = path
                    )
                }

                context.contentResolver.openOutputStream(
                    path.toUri(),
                    "wt" // Stream opened with write and truncate mode
                )?.use { stream ->
                    stream.write(data.toByteArray())
                }
            }.logFailure()
        }
    }

    override suspend fun deleteFile(path: String) {
        withContext(Dispatchers.IO) {
            runCatching {
                path.toFile().delete()
            }.logFailure()
        }
    }

    private val directory: File
        get() = File(context.filesDir, BASE_DIR)

    private fun String.toFile() = File(directory, this)
    private fun File.toUri() = Uri.fromFile(this)
    private fun String.toUri(): Uri = toFile().toUri()

    internal companion object {
        const val BASE_DIR = "sargon"
    }
}