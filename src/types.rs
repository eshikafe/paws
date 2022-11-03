pub type Float = f32;
pub type Int = i32;

pub enum RequestType {
    InitReq,
    InitResp,
    RegistrationReq,
    RegistrationResp,
}

impl RequestType {
    pub fn to_string(&self) -> String {
        match *self {
            RequestType::InitReq => "INIT_REQ".to_string(),
            RequestType::InitResp => "INIT_RESP".to_string(),
            RequestType::RegistrationReq => "REGISTRATION_REQ".to_string(),
            RequestType::RegistrationResp => "REGISTRATION_RESP".to_string()
        }
    }
}