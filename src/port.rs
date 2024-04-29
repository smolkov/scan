use crate::error::ScanError;

#[derive(Debug)]
pub struct PortInfo {
    pub port: u32,
    pub open: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PortRange {
    start: u32,
    end: u32,
}

impl PortRange {
    pub fn new(start: u32, end: u32) -> PortRange {
        PortRange { start, end }
    }
    pub fn steps(&self) -> u32 {
        self.end - self.start
    }
}

impl Iterator for PortRange {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }
        let pos = self.start;
        self.start += 1;
        Some(pos)
    }
}

impl std::str::FromStr for PortRange {
    type Err = ScanError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once("..") {
            Ok(PortRange::new(start.parse::<u32>()?, end.parse::<u32>()?))
        } else {
            Err(ScanError::InvalidPortRange)
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn port_range() {
        assert_eq!(
            PortRange::new(0, 65535),
            "0..65535".parse::<PortRange>().unwrap()
        );
        assert!("0.65535".parse::<PortRange>().is_err());
        assert!("0-65535".parse::<PortRange>().is_err());
        let range = "1..20".parse::<PortRange>().unwrap();
        let start = range.start;
        assert_eq!(start, range.into_iter().last().unwrap());
    }
}
