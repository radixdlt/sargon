//package com.radixdlt.sargon.samples
//
//import com.radixdlt.sargon.SecurityQuestionsNotProductionReadyFactorSource
//import com.radixdlt.sargon.annotation.UsesSampleValues
//import com.radixdlt.sargon.newSecurityQuestionsFactorSourceSample
//import com.radixdlt.sargon.newSecurityQuestionsFactorSourceSampleOther
//
//@UsesSampleValues
//val SecurityQuestionsNotProductionReadyFactorSource.Companion.sample: Sample<SecurityQuestionsNotProductionReadyFactorSource>
//    get() = object : Sample<SecurityQuestionsNotProductionReadyFactorSource> {
//        override fun invoke(): SecurityQuestionsNotProductionReadyFactorSource
//            = newSecurityQuestionsFactorSourceSample()
//
//        override fun other(): SecurityQuestionsNotProductionReadyFactorSource
//            = newSecurityQuestionsFactorSourceSampleOther()
//
//    }