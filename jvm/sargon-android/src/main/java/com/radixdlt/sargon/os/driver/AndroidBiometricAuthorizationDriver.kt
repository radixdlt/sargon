package com.radixdlt.sargon.os.driver

import android.os.Build
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.biometric.BiometricPrompt.AuthenticationError
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.flowWithLifecycle
import androidx.lifecycle.lifecycleScope
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.SecureStorageAccessErrorKind
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.secureStorageKeyIdentifier
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.coroutines.withContext
import timber.log.Timber
import javax.crypto.Cipher
import kotlin.coroutines.resume
import kotlin.coroutines.suspendCoroutine

internal interface BiometricAuthorizationDriver {

    val hasStrongAuthenticator: Boolean

    suspend fun authorize(cipher: Cipher?): Result<Cipher?>

}

class BiometricsFailure(
    @AuthenticationError
    val errorCode: Int,
    val errorMessage: String?
) : Exception("[$errorCode] $errorMessage") {

    fun toCommonException(
        key: SecureStorageKey
    ): CommonException = CommonException.SecureStorageAccessException(
        key = secureStorageKeyIdentifier(key),
        errorKind = when (errorCode) {
            BiometricPrompt.ERROR_CANCELED -> SecureStorageAccessErrorKind.CANCELLED
            BiometricPrompt.ERROR_HW_NOT_PRESENT -> SecureStorageAccessErrorKind.HARDWARE_NOT_PRESENT
            BiometricPrompt.ERROR_HW_UNAVAILABLE -> SecureStorageAccessErrorKind.HARDWARE_UNAVAILABLE
            BiometricPrompt.ERROR_LOCKOUT -> SecureStorageAccessErrorKind.LOCKOUT
            BiometricPrompt.ERROR_LOCKOUT_PERMANENT -> SecureStorageAccessErrorKind.LOCKOUT_PERMANENT
            BiometricPrompt.ERROR_NEGATIVE_BUTTON -> SecureStorageAccessErrorKind.NEGATIVE_BUTTON
            BiometricPrompt.ERROR_NO_BIOMETRICS -> SecureStorageAccessErrorKind.NO_BIOMETRICS
            BiometricPrompt.ERROR_NO_DEVICE_CREDENTIAL -> SecureStorageAccessErrorKind.NO_DEVICE_CREDENTIAL
            BiometricPrompt.ERROR_NO_SPACE -> SecureStorageAccessErrorKind.NO_SPACE
            BiometricPrompt.ERROR_TIMEOUT -> SecureStorageAccessErrorKind.TIMEOUT
            BiometricPrompt.ERROR_UNABLE_TO_PROCESS -> SecureStorageAccessErrorKind.UNABLE_TO_PROCESS
            BiometricPrompt.ERROR_USER_CANCELED -> SecureStorageAccessErrorKind.USER_CANCELLED
            BiometricPrompt.ERROR_VENDOR -> SecureStorageAccessErrorKind.VENDOR
            else -> SecureStorageAccessErrorKind.UNKNOWN
        },
        errorMessage = message.orEmpty()
    )

}

internal class AndroidBiometricAuthorizationDriver(
    private val biometricsHandler: BiometricsHandler
) : BiometricAuthorizationDriver {

    override val hasStrongAuthenticator: Boolean
        get() = biometricsHandler.hasStrongAuthenticator


    override suspend fun authorize(cipher: Cipher?): Result<Cipher?> = biometricsHandler.askForBiometrics(cipher)

}

class BiometricsHandler(
    internal val biometricsSystemDialogTitle: String
) {

    private val biometricRequestsChannel = Channel<Cipher?>()
    private val biometricsResultsChannel = Channel<Result<Cipher?>>()

    private val allowedAuthenticators = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
        BiometricManager.Authenticators.BIOMETRIC_STRONG or
            BiometricManager.Authenticators.DEVICE_CREDENTIAL
    } else {
        BiometricManager.Authenticators.BIOMETRIC_WEAK or
            BiometricManager.Authenticators.DEVICE_CREDENTIAL
    }

    private var _hasStrongAuthenticator: Boolean = false
    val hasStrongAuthenticator: Boolean
        get() = _hasStrongAuthenticator

    fun register(
        activity: FragmentActivity,
        callbacks: OnBiometricsLifecycleCallbacks? = null
    ) {
        _hasStrongAuthenticator = hasStrongAuthenticator(activity)
        biometricRequestsChannel
            .receiveAsFlow()
            .flowWithLifecycle(
                lifecycle = activity.lifecycle,
                minActiveState = Lifecycle.State.STARTED
            )
            .onEach {
                callbacks?.onBeforeBiometricsRequest()
                val result = requestBiometricsAuthorization(activity, it)

                // Send back the result to sargon os
                biometricsResultsChannel.send(result)
                callbacks?.onAfterBiometricsResult()
            }
            .launchIn(activity.lifecycleScope)
    }

    suspend fun askForBiometrics(cipher: Cipher? = null): Result<Cipher?> {
        // Suspend until an activity is subscribed to this channel and is at least started
        biometricRequestsChannel.send(cipher)

        // If an activity is already registered, then we need to wait until the user provides
        // the response from the biometrics prompt
        return biometricsResultsChannel.receive()
    }

    private suspend fun requestBiometricsAuthorization(
        activity: FragmentActivity,
        cipher: Cipher?
    ): Result<Cipher?> = withContext(Dispatchers.Main) {
        suspendCoroutine { continuation ->
            val authCallback = object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                    continuation.resume(Result.success(result.cryptoObject?.cipher))
                }

                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    continuation.resume(
                        Result.failure(BiometricsFailure(errorCode, errString.toString()))
                    )
                }

                override fun onAuthenticationFailed() {
                    Timber.tag("Sargon").w("Biometrics failed.")
                }
            }

            val promptInfo = BiometricPrompt.PromptInfo.Builder()
                .setTitle(biometricsSystemDialogTitle)
                .setAllowedAuthenticators(allowedAuthenticators)
                .build()

            val biometricPrompt = BiometricPrompt(
                activity,
                ContextCompat.getMainExecutor(activity),
                authCallback
            )

            if (cipher != null && hasStrongAuthenticator) {
                // Invokes crypto based authentication
                biometricPrompt.authenticate(
                    promptInfo,
                    BiometricPrompt.CryptoObject(cipher)
                )
            } else {
                // Crypto based authentication is incompatible with Class 2 (formerly Weak) biometrics
                // and (prior to Android 11) device credential.
                biometricPrompt.authenticate(promptInfo)
            }
        }
    }

    private fun hasStrongAuthenticator(activity: FragmentActivity): Boolean {
        val biometricManager = BiometricManager.from(activity)

        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            biometricManager.canAuthenticate(
                BiometricManager.Authenticators.BIOMETRIC_STRONG
            ) == BiometricManager.BIOMETRIC_SUCCESS
        } else {
            false
        }
    }
}

/**
 * Callback functions called to notify the host about the lifecycle
 * of a biometrics request
 */
interface OnBiometricsLifecycleCallbacks {
    /**
     * Called right before a biometrics request is about to be appeared.
     */
    fun onBeforeBiometricsRequest()

    /**
     * Called right after a biometrics request has ended with a result.
     */
    fun onAfterBiometricsResult()
}