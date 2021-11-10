use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoraDatarate {
    SF5,
    SF6,
    SF7,
    SF8,
    SF9,
    SF10,
    SF11,
    SF12,
}

impl TryFrom<u8> for LoraDatarate {
    type Error = String;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            5 => Ok(LoraDatarate::SF5),
            6 => Ok(LoraDatarate::SF6),
            7 => Ok(LoraDatarate::SF7),
            8 => Ok(LoraDatarate::SF8),
            9 => Ok(LoraDatarate::SF9),
            10 => Ok(LoraDatarate::SF10),
            11 => Ok(LoraDatarate::SF11),
            12 => Ok(LoraDatarate::SF12),
            n => Err(format!("unknown datarate: {}", n))
        }
    }
}

impl fmt::Display for LoraDatarate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoraDatarate::SF5 => write!(f, "SF5"),
            LoraDatarate::SF6 => write!(f, "SF6"),
            LoraDatarate::SF7 => write!(f, "SF7"),
            LoraDatarate::SF8 => write!(f, "SF8"),
            LoraDatarate::SF9 => write!(f, "SF9"),
            LoraDatarate::SF10 => write!(f, "SF10"),
            LoraDatarate::SF11 => write!(f, "SF11"),
            LoraDatarate::SF12 => write!(f, "SF12"),
        }
        
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoraBandwidth {
    BW_125KHZ,
    BW_250KHZ,
    BW_500KHZ,
}

impl TryFrom<u8> for LoraBandwidth {
    type Error = String;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            0x04 => Ok(LoraBandwidth::BW_125KHZ),
            0x05 => Ok(LoraBandwidth::BW_250KHZ),
            0x06 => Ok(LoraBandwidth::BW_500KHZ),
            n => Err(format!("unknown bandwith: {}", n))
        }
    }
}

impl fmt::Display for LoraBandwidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoraBandwidth::BW_125KHZ => write!(f, "BW125"),
            LoraBandwidth::BW_250KHZ => write!(f, "BW250"),
            LoraBandwidth::BW_500KHZ => write!(f, "BW500"),
        }
        
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Datr {
    datarate: LoraDatarate,
    bandwidth: LoraBandwidth,
}

impl Datr {
    pub fn new(dr: LoraDatarate, bw: LoraBandwidth) -> Self {
        Datr { datarate: dr, bandwidth: bw }
    }
}

impl fmt::Display for Datr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.datarate, self.bandwidth)
    }
}

mod test {
    use super::*;

    #[test]
    fn datarate_from() {
        let dr = LoraDatarate::try_from(6);
        assert_eq!(Ok(LoraDatarate::SF6), dr);
    }

    #[test]
    fn bandwidth_from() {
        let bw = LoraBandwidth::try_from(0x04);
        assert_eq!(Ok(LoraBandwidth::BW_125KHZ), bw);
    }

    #[test]
    fn display_datr() {
        let dr = Datr::new(LoraDatarate::SF10, LoraBandwidth::BW_125KHZ);
        assert_eq!("SF10BW125", dr.to_string());
    }
}

