#[derive(Clone, Copy, Debug)]
pub enum Mode {
    ReadOnly,
    ReadWrite,
}

#[derive(Clone, Copy, Debug)]
pub enum DatabaseFlags {
    Create,
    Open,
}