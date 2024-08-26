#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub enum Region {
    ASH,
    DAL,
    LAX,
    AMS,
}

impl Region {
    pub fn get() -> Region {
        let region = std::env::var("REGION").unwrap();

        match region.as_str() {
            "ASH" => Region::ASH,
            "DAL" => Region::DAL,
            "LAX" => Region::LAX,
            "AMS" => Region::AMS,
            _ => panic!("Invalid region"),
        }
    }

    pub fn get_distance(&self, region: &Region) -> u32 {
        match self {
            Region::ASH => match region {
                Region::ASH => 0,
                Region::DAL => 1,
                Region::LAX => 3,
                Region::AMS => 2,
            },
            Region::DAL => match region {
                Region::ASH => 1,
                Region::DAL => 0,
                Region::LAX => 1,
                Region::AMS => 2,
            },
            Region::LAX => match region {
                Region::ASH => 2,
                Region::DAL => 1,
                Region::LAX => 0,
                Region::AMS => 3,
            },
            Region::AMS => match region {
                Region::ASH => 1,
                Region::DAL => 2,
                Region::LAX => 3,
                Region::AMS => 0,
            },
        }
    }
}
