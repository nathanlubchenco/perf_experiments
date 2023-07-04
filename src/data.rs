use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
pub enum NnImpls {
    Heap,
    ParallelSort,
    KDTree,
    LocalitySensitiveHashing,
    HierarchicalSmallWorld
}

impl FromStr for NnImpls {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Heap" | "heap" | "h" => Ok(NnImpls::Heap),
            "parallel" | "sort" | "parallelSort" | "ps" => Ok(NnImpls::ParallelSort),
            _ => Err("no match"),
        }
    }
}

impl fmt::Display for NnImpls {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NnImpls::Heap => write!(f, "Heap"),
            NnImpls::ParallelSort => write!(f, "ParallelSort"),
            _ => write!(f, "not yet supported approach")
        }
    }
}