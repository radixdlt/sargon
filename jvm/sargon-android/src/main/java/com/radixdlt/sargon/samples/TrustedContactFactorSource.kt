//package com.radixdlt.sargon.samples
//
//import com.radixdlt.sargon.TrustedContactFactorSource
//import com.radixdlt.sargon.annotation.UsesSampleValues
//import com.radixdlt.sargon.newTrustedContactFactorSourceSample
//import com.radixdlt.sargon.newTrustedContactFactorSourceSampleOther
//
//@UsesSampleValues
//val TrustedContactFactorSource.Companion.sample: Sample<TrustedContactFactorSource>
//    get() = object : Sample<TrustedContactFactorSource> {
//        override fun invoke(): TrustedContactFactorSource = newTrustedContactFactorSourceSample()
//
//        override fun other(): TrustedContactFactorSource = newTrustedContactFactorSourceSampleOther()
//    }