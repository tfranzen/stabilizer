use miniconf::Tree;
use serde::{Deserialize, Serialize};



#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum LockState {
    SCANNING = 0,
    LOCKED = 1,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Tree)]
pub struct LockSettings {


    pub enable: bool,

    pub scan_frequency: f32,

    pub state_request: LockState,

    pub state: LockState,

    pub lock_point: f32,


}


impl Default for LockSettings {
    fn default() -> Self {
        Self {
             enable: true,
             scan_frequency: 10.,
             state_request: LockState::SCANNING,
             state: LockState::SCANNING,
             lock_point: 0.,
        }
    }
}


#[derive(Debug)]
pub struct LockBox {
    pub settings: LockSettings,
    pub state: LockState,
}