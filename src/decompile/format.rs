
#[derive(Clone, Debug)]
enum FMVersion {
    FMP7 = 7,
    FMP12 = 12,
}


#[derive(Clone, Debug)]
pub struct Header {
    valid: bool,
    version: FMVersion,
    creator: String,
}
