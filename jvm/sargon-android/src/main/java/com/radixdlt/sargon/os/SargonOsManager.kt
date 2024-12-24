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
                val os = SargonOs.boot(bios, hostInteractor)
                _sargonState.update { SargonOsState.Booted(os) }
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
    }
}

sealed interface SargonOsState {
    data object Idle : SargonOsState
    data class Booted(
        val os: SargonOs
    ) : SargonOsState
}

class SargonOsNotBooted : IllegalStateException("Sargon OS is not booted")