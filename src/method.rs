// PAWS methods
pub enum Method {
    Init,
    Register,
    GetSpectrum,
    GetSpectrumBatch,
    NotifySpectrumUse,
    VerifyDevice,
}

impl Method {
    pub fn to_string(&self) -> String {
        match *self {
            Method::Init => "spectrum.paws.init".to_string(),
            Method::Register => "spectrum.paws.register".to_string(),
            Method::GetSpectrum => "spectrum.paws.getSpectrum".to_string(),
            Method::GetSpectrumBatch => "spectrum.paws.getSpectrumBatch".to_string(),
            Method::NotifySpectrumUse => "spectrum.paws.notifySpectrumUse".to_string(),
            Method::VerifyDevice => "spectrum.paws.verifyDevice".to_string(),
        }
    }
}