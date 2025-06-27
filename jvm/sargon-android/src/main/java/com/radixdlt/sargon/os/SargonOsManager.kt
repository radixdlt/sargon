package com.radixdlt.sargon.os

import com.radixdlt.sargon.Bios
import com.radixdlt.sargon.HostInteractor
import com.radixdlt.sargon.SargonOs
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import org.jetbrains.annotations.VisibleForTesting

class SargonOsManager internal constructor(
    bios: Bios,
    hostInteractor: HostInteractor,
    applicationScope: CoroutineScope,
    defaultDispatcher: CoroutineDispatcher
) {
    private val _sargonState = MutableStateFlow<SargonOsState>(SargonOsState.Idle)

    val sargonState: StateFlow<SargonOsState> = _sargonState.asStateFlow()
    val sargonOs: SargonOs
        get() = (sargonState.value as? SargonOsState.Booted)?.os ?: throw SargonOsNotBooted()

    init {
        applicationScope.launch {
            withContext(defaultDispatcher) {
                runCatching {
                    SargonOs.boot(bios, hostInteractor)
                }.onSuccess { os ->
                    _sargonState.update { SargonOsState.Booted(os) }
                }.onFailure { error ->
                    _sargonState.update { SargonOsState.BootError(error) }
                }
            }
        }
    }

    companion object {
        @Volatile
        private var instance: SargonOsManager? = null

        fun factory(
            bios: Bios,
            hostInteractor: HostInteractor,
            applicationScope: CoroutineScope,
            defaultDispatcher: CoroutineDispatcher
        ): SargonOsManager = instance ?: synchronized(this) {
            instance ?: SargonOsManager(
                bios = bios,
                hostInteractor = hostInteractor,
                applicationScope = applicationScope,
                defaultDispatcher = defaultDispatcher
            ).also {
                instance = it
            }
        }

        // Only used for testing
        @VisibleForTesting
        internal fun tearDown() = synchronized(this) {
            instance = null
        }
    }
}

sealed interface SargonOsState {
    data object Idle : SargonOsState
    data class Booted(
        val os: SargonOs
    ) : SargonOsState
    data class BootError(
        val error: Throwable
    ) : SargonOsState
}

class SargonOsNotBooted : IllegalStateException("Sargon OS is not booted")