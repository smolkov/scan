
pub mod cli;
pub mod scanner;
pub mod error;
use error::ScannError;

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct PortRange {
    start: u16,
    end: u16,
}

impl PortRange {
    pub fn new(start: u16, end: u16) -> PortRange {
        PortRange { start, end }
    }
}

impl Iterator for PortRange {
	type Item = u16;

	fn next(&mut self) -> Option<Self::Item> {
		if self.start > self.end {
			return None
		}
		let pos: u16 = self.start;	
		self.start +=1;
		Some(pos)
	}
}

impl std::str::FromStr for PortRange {
	type Err = ScannError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some((start,end)) = s.split_once("..") {
			Ok(PortRange::new(start.parse::<u16>()?,end.parse::<u16>()?))
		}else {
			Err(ScannError::InvalidPortRange)
		}
	}
}




#[cfg(test)] 
mod test{

	use super::*;

	#[test]
	fn port_range() {
		assert_eq!(PortRange::new(0,65535),"0..65535".parse::<PortRange>().unwrap());
		assert!("0.65535".parse::<PortRange>().is_err());
		assert!("0-65535".parse::<PortRange>().is_err());
		let range = "1..20".parse::<PortRange>().unwrap();
		let start = range.start;
		assert_eq!(start, range.into_iter().last().unwrap());
	}
}