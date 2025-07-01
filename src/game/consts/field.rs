pub const FIELD_WIDTH: u16 = 10;
pub const FIELD_HEIGHT: u16 = 20;

pub type Row = u16;
pub const ROW_OFFSET: usize = size_of::<Row>() * 8 - FIELD_WIDTH as usize;
pub const EMPTY_ROW: Row = Row::MAX << FIELD_WIDTH;

pub type Area = [Row; FIELD_HEIGHT as usize];
pub const EMPTY_AREA: Area = [EMPTY_ROW; FIELD_HEIGHT as usize];