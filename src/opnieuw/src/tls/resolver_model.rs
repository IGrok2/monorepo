pub struct Ticketer {}

unsafe impl Send for Ticketer {}
unsafe impl Sync for Ticketer {}

pub struct SessionStorage {}

unsafe impl Send for SessionStorage {}
unsafe impl Sync for SessionStorage {}

pub struct CertResolver {}

unsafe impl Send for CertResolver {}
unsafe impl Sync for CertResolver {}

pub struct ChallengeResponse {
    pub token: String,
    pub response: String,
    pub epoch: u64,
}
