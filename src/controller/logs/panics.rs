#[cfg(feature = "with-backtrace")]
extern crate backtrace;

use log::error;
use std::fmt;
use std::panic;
use std::thread;
use chrono::Utc;

use backtrace::Backtrace;

#[cfg(not(feature = "with-backtrace"))]
mod backtrace {
    #[derive(Clone)]
    pub struct Backtrace;

    impl Backtrace {
        pub fn new() -> Backtrace {
            Backtrace
        }
    }
}

struct Shim(Backtrace);

impl fmt::Debug for Shim {
    #[cfg(feature = "with-backtrace")]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "\n{:?}", self.0)
    }

    #[cfg(not(feature = "with-backtrace"))]
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

pub fn manual_log(error_type: String, details: String, error_message: String) {
    use crate::models::{NewSysLog};
    use crate::schema::{syslog};
    use diesel::prelude::*;
    
    match diesel::insert_into(syslog::table)
        .values(NewSysLog {
            syslog_creation: Utc::now().naive_utc(),
            syslog_content: format!("[{}] details: '{}' - error_message: '{}'",
                error_type,
                details,
                error_message
            ),
        })
        .execute(&crate::establish_connection()) {
            Ok(_) => {}
            Err(_) => {}
        }
}


/* Initializes a panic hook to panic! exceptions */
pub fn init() {
    /* Defining the hook function */
    panic::set_hook(Box::new(|info| {
        let backtrace = Backtrace::new();
        /* Thread information */
        let thread = thread::current();
        let thread = thread.name().unwrap_or("unnamed");

        /* Getting the message */
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &**s,
                None => "Box<Any>",
            },
        };

        use crate::models::{NewSysLog};
        use crate::schema::{syslog};
        use diesel::prelude::*;
        /* If has location information about the exception */
        match info.location() {
            Some(location) => {
                /* Inserting the exception information with information about the location in the database */
                diesel::insert_into(syslog::table)
                    .values(NewSysLog {
                        syslog_creation: Utc::now().naive_utc(),
                        syslog_content: format!("[Panic]-thread '{0}' panicked at '{1}': {2}:{3}:{4} {5:?}",
                            thread,
                            msg,
                            location.file(),
                            location.line(),
                            location.column(), 
                            Shim(backtrace.clone())
                        ),
                    })
                    .execute(&crate::establish_connection())
                    .unwrap();
                /* Showing the exception message on console */    
                error!(
                    target: "panic", "thread '{}' panicked at '{}': {}:{}:{} {:?}",
                    thread,
                    msg,
                    location.file(),
                    location.line(),
                    location.column(),
                    Shim(backtrace)
                );
            }
            None => {
                /* Inserting the exception information without information about the location in the database */
                diesel::insert_into(syslog::table)
                    .values(NewSysLog {
                        syslog_creation: Utc::now().naive_utc(),
                        syslog_content: format!("[Panic]-thread '{}' panicked at '{}' {:?}",
                            thread,
                            msg,
                            Shim(backtrace.clone())
                        ),
                    })
                    .execute(&crate::establish_connection()).unwrap();
                /* Showing the exception message on cosole */        
                error!(
                    target: "panic",
                    "thread '{}' panicked at '{}'{:?}",
                    thread,
                    msg,
                    Shim(backtrace)
                )
            }
        }
    }));
}