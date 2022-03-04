use chrono::{DateTime, TimeZone};
use serde::{self, Deserialize, Serialize};
use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

const EPOCH_0_TIMESTAMP: i64 = -2_208_988_800;
const WRAP: i64 = 2_i64.pow(32);
/// approximately 68 years
const MIDDLE: u32 = 2_u32.pow(16);

#[derive(Debug, Deserialize, Clone, Serialize, Eq, PartialEq, Copy)]
#[serde(from = "u32", into = "u32")]
pub struct NtpTime {
    seconds: u32,
}

impl NtpTime {
    #[must_use]
    pub fn now() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before Unix epoch")
            .as_secs();
        Self::from_timestamp(i64::try_from(timestamp).expect("system time impossibly large"))
    }

    #[must_use]
    pub fn from_timestamp(timestamp: i64) -> Self {
        let seconds = (timestamp - EPOCH_0_TIMESTAMP).rem_euclid(WRAP);
        let seconds = u32::try_from(seconds).unwrap();
        Self { seconds }
    }

    #[must_use]
    pub fn add_seconds(self, seconds: u32) -> Self {
        let seconds = self.seconds.wrapping_add(seconds);
        Self { seconds }
    }

    #[must_use]
    pub fn sub_seconds(self, seconds: u32) -> Self {
        let seconds = self.seconds.wrapping_sub(seconds);
        Self { seconds }
    }
}

impl From<u32> for NtpTime {
    fn from(seconds: u32) -> Self {
        NtpTime { seconds }
    }
}

impl From<NtpTime> for u32 {
    fn from(ntp: NtpTime) -> Self {
        ntp.seconds
    }
}

impl<Tz: TimeZone> From<DateTime<Tz>> for NtpTime {
    fn from(date: DateTime<Tz>) -> Self {
        Self::from_timestamp(date.timestamp())
    }
}

impl Ord for NtpTime {
    fn cmp(&self, other: &Self) -> Ordering {
        let res = other.seconds.wrapping_sub(self.seconds);
        if res == 0 {
            Ordering::Equal
        } else if res > MIDDLE {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for NtpTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]

    fn historic_ntp_times() {
        let ymd = |y, m, d| u32::from(NtpTime::from(Utc.ymd(y, m, d).and_hms(0, 0, 0)));
        // 1 CE TODO: this does not work
        // assert_eq!(ymd(1, 1, 1), 202_939_144);
        // Last day Julian
        assert_eq!(ymd(1582, 10, 4), 2_873_647_488);
        // Last day NTP Era -1
        assert_eq!(ymd(1899, 12, 31), 4_294_880_896);
        // First day NTP Era 0
        assert_eq!(ymd(1900, 1, 1), 0);
        // First day UNIX
        assert_eq!(ymd(1970, 1, 1), 2_208_988_800);
        // Fist day NTP Era 1
        assert_eq!(ymd(2036, 2, 8), 63_104);
    }

    #[test]
    fn compare() {
        let cmp = |a, b| NtpTime::from(a).cmp(&NtpTime::from(b));
        assert_eq!(cmp(10, 10), Ordering::Equal);
        assert_eq!(cmp(10, 11), Ordering::Less);
        assert_eq!(cmp(11, 10), Ordering::Greater);
        assert_eq!(cmp(u32::MAX - 10, 10), Ordering::Less);
    }
}
