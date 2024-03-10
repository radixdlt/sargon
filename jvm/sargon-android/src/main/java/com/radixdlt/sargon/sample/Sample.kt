package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting

@VisibleForTesting(otherwise = VisibleForTesting.PACKAGE_PRIVATE)
interface Sample<T> {

    val all: List<T>
        get() = listOf(invoke(), other())

    operator fun invoke(): T

    fun other(): T

}