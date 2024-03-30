package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting

@VisibleForTesting(otherwise = VisibleForTesting.PACKAGE_PRIVATE)
interface Sample<T> {

    val all: List<T>
        get() = listOf(invoke(), other())

    operator fun invoke(): T

    fun other(): T

}

interface SampleWithRandomValues<T>: Sample<T> {

    override val all: List<T>
        get() = listOf(invoke(), other(), random(), random())

    fun random(): T

}