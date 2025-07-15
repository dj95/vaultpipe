use miette::Result;

pub mod associate;
pub mod change_public_key;
pub mod get_logins;
pub mod test_association;

pub trait Action {
    fn action(&self) -> String;
    fn payload(&self) -> Result<String>;
    fn needs_encryption(&self) -> bool;
}
