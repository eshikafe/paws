// Copyright 2021 TVWS-Project

// PAWS methods
pub enum Method {
    Init,              // spectrum.paws.init
    Register,          // spectrum.paws.register
    GetSpectrum,       // spectrum.paws.getSpectrum
    GetSpectrumBatch,  // spectrum.paws.getSpectrumBatch
    NotifySpectrumUse, // spectrum.paws.notifySpectrumUse
    VerifyDevice,      // spectrum.paws.verifyDevice
}
