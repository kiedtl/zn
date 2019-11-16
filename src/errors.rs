//
// errors in Zn.
// (c) Kied Llaentenn
//

use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorCause {
    // BufferOverflowError,        // when buffer size exeeds size 65535
    StackOverflowError,         // when stack size exeeds size 8191
    NilStackPopError,           // when you attempt to pop item off null stack
    InvalidRegisterError,       // when provided token is not a valid register
    InvalidTokenError,          // when provided token is not valid integer
    NilPointerError,            // when you attempt to access a stack item (a pointer) with NULL value.
    InvalidConditionError,      // when you attempt to execute a loop/condition on a NULL condition, (e.g., `|[+@1]`)
    UnexpectedTokenError,
}

impl ErrorCause {
    fn message(&self) -> &str {
        // TODO expand
        "oops"
    }
}

impl fmt::Display for ErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    file:           String,
    reason:         ErrorCause,
    context:        String,
    context_char:   usize,
}

impl Error {
    // print out error
    pub fn throw(&self) {
        // escape character: 0x1B (ASCII 27)
        // used to format text in terminals
        let esc: char = 0x1B as char;

        // make errors bold
        print!("{}[1m", esc);

        // print out error type, file, line, and char, and message.
        print!("{}[31m{}{}[37m@{}: {}[31merror: {}[0m{}\n",
               esc,
               self.reason.to_string(),
               esc,
               self.file,
               esc, esc,
               self.reason.message()
            );

        // print out context
        // TODO: point out problem char in context
        // TODO: e.g.,
        //              UnspecifiedConditionError@main.zn:23:88: error: address not specified:
        //                  23 | ...$&~25|[+@18]...
        //                     |          ^        
        print!("\t| ...{}...\n", self.context);
    }
}
