package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues

@UsesSampleValues
interface Sample<T> {

    val all: List<T>
        get() = listOf(invoke(), other())

    operator fun invoke(): T

    fun other(): T

}

@UsesSampleValues
interface SampleWithRandomValues<T>: Sample<T> {

    override val all: List<T>
        get() = listOf(invoke(), other(), random(), random())

    fun random(): T

}