// Describes how nametable mirroring works, what is shown on reads of the bottom/right of the
// current nametable.
#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    SingleScreen,
    FourScreen,
}
