use core::fmt::{self, Write};
use limine::{LimineTerminalRequest, LimineTerminalResponse};

static TERMINAL_REQUEST: LimineTerminalRequest = LimineTerminalRequest::new(0);

struct Writer {
    terminals: Option<&'static LimineTerminalResponse>,
}

unsafe impl Send for Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let terminals = match self.terminals {
            None => {
                // Not initialized yet, pull the response from the request.
                let response = TERMINAL_REQUEST.get_response().get().ok_or(fmt::Error)?;
                self.terminals = Some(response);
                response
            }
            Some(response) => response,
        };

        let write = terminals.write().ok_or(fmt::Error)?;
        for terminal in terminals.terminals() {
            write(terminal, s);
        }

        Ok(())
    }
}

static WRITER: spin::Mutex<Writer> = spin::Mutex::new(Writer { terminals: None });

pub fn _print(args: fmt::Arguments) {
    let mut writer = WRITER.lock();
    writer.write_fmt(args).ok();
}

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {
        $crate::writer::_print(format_args!($($t)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::writer::_print(format_args!("\n"))
    };
    ($($t:tt)*) => {
        $crate::writer::_print(format_args!("{}\n", format_args!($($t)*)))
    };
}
