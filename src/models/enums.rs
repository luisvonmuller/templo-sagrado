/* All possible behaviours for a clerk status */
pub enum Status {
    Offline = 0,
    Online = 1,
    Oncall = 2,
}

pub enum ClerkTimeEventType {
    Gone = 0,
    Available = 1,
    Busy = 2,
    GoneForTextChat = 3,
    BackFromTextChat = 4,
    GoneForVoiceChat = 5,
    BackFromVoiceChatt = 6,
}