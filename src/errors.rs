//
// errors in Zn.
// (c) Kied Llaentenn
//

#[derive(Debug, Copy, Clone)]
pub enum ErrorCause {
    BufferOverflowError,        // when buffer size exeeds size 65535
    StackOverflowError,         // when stack size exeeds size 8191
    NilStackPopError,           // when you attempt to pop item off null stack
    UnspecifiedValueError,      // when you attempt to use an unspecified value (e.g., `#~255_`)
    NilPointerError,            // when you attempt to access a stack item (a pointer) with NULL value.
    UnspecifiedConditionError,  // when you attempt to execute a loop/condition on a NULL condition, (e.g., `|[+@1]`)
    UnspecifiedAddressError,    // when you attempt to set a pointer to NULL address. (e.g., `&@~`)
}

impl fmt::Display for ErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Error {
    lineno:         usize,
    charno:         usize,
    reason:         ErrorCause,
    context:        String,
    context_char:   usize,
}

impl Error {
    fn throw(&self) {
    
    }
}
