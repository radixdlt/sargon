package com.radixdlt.sargon.android

import com.radixdlt.sargon.Bios
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.SargonOs
import com.radixdlt.sargon.android.di.ApplicationScope
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.filterIsInstance
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class SargonOsManager @Inject constructor(
    private val bios: Bios,
    @ApplicationScope private val applicationScope: CoroutineScope
) {

    private val _state = MutableStateFlow<SargonState>(SargonState.Idle)

    val sargonState: Flow<SargonState>
        get() = _state.asStateFlow()

    val sargon: Flow<SargonOs> = _state
        .filterIsInstance<SargonState.Booted>()
        .map { it.os }


    init {
        boot()
    }

    fun boot() = applicationScope.launch {
        if (_state.value is SargonState.Booted) {
            return@launch
        }

        withContext(Dispatchers.Default) {
            runCatching {
                SargonOs.boot(bios)
            }.onSuccess { os ->
                _state.update { SargonState.Booted(os) }
            }.onFailure { error ->
                if (error is CommonException) {
                    _state.update { SargonState.BootError(error) }
                } else {
                    throw error
                }
            }
        }
    }

    sealed interface SargonState {
        data object Idle: SargonState
        data class BootError(
            val error: CommonException
        ): SargonState
        data class Booted(
            val os: SargonOs
        ): SargonState
    }

}