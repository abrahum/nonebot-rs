use Result as SysResult;

#[derive(Debug)]
pub enum Error {
    #[allow(dead_code)]
    Custom(String),
}

pub type HandlerResult = SysResult<bool, Error>;
pub type AfterMatcherResult = SysResult<bool, Error>;
