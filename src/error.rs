//inside error.rs
use thiserror::Error; //https://docs.rs/thiserror/1.0.25/thiserror/
use solana_program::program_error::ProgramError;



// derive basically implicates traits onto structures, so now you can do EscrowError::Error
// Remember an enum{A,B,C} is either A,B or C. What #[error(..)] does is display a message depending onw aht type it is
// furthermore:
// The messages support a shorthand for interpolating fields from the error.

// #[error("{var}")] ⟶ write!("{}", self.var)
// #[error("{0}")] ⟶ write!("{}", self.0)
// #[error("{var:?}")] ⟶ write!("{:?}", self.var) as in write what is in the {} and return self.var
// #[error("{0:?}")] ⟶ write!("{:?}", self.0)


	#[derive(Error,Debug,Copy,Clone)]
	pub enum EscrowError{
		//Invalid instruction escrow error type
		
		#[error("Invalid Instruction")]
		InvalidInstruction,
		//Not rent exempt error type
		#[error("Not rent exempt")]
		NotRentExempt,
		#[error("Account mismatch")]
		ExpectedAmountMismatch,
		#[error("Amount overflowing error")]
		AmountOverflow,
	}

// The From trait allows for a type to define how to create itself from another type, hence providing a 
// very simple mechanism for converting between several types. There are numerous implementations of this
//  trait within the standard library for conversion of primitive and common types.

// The below code allows you to convert FROM< a EscrowERror> to a ProgramError.
// the fn from must be implicated, and it returns self so a ProgramError, which in this case is the conversion of an EE to a u32
// Given the solana custom type, a ProgramError can be returned by ProgramError::Custom(customthing as u 32)

	impl From<EscrowError> for ProgramError {
		fn from(e: EscrowError) -> Self {
		ProgramError::Custom(e as u32)
		}
	}

