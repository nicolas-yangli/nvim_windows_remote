use std::error::Error;
use std::io::Write;

extern crate rmp;

pub struct Session {
    msgid: u32,
}

impl Session {
    pub fn new() -> Session {
        Session {
            msgid: 0,
        }
    }

    pub fn notify<'a, W: Write>(&mut self, buf: &'a mut W) -> Notify<'a, W> {
        Notify {
            state: NotifyState::WaitMethod,
            buf,
        }
    }

    pub fn request<'a, W: Write>(&mut self, buf: &'a mut W) -> Request<'a, W> {
        self.msgid += 1;
        Request {
            msgid: self.msgid,
            state: RequestState::WaitMethod,
            buf,
        }
    }
}

pub struct Notify<'a, W: Write> {
    state: NotifyState,
    buf: &'a mut W,
}

#[derive(Debug, PartialEq)]
pub enum NotifyState {
    WaitMethod,
    WaitParamCount,
    WaitParam(u8),
    Sent,
}

pub struct Request<'a, W: Write> {
    msgid: u32,
    state: RequestState,
    buf: &'a mut W,
}

#[derive(Debug, PartialEq)]
pub enum RequestState {
    WaitMethod,
    WaitParamCount,
    WaitParam(u8),
    Sent,
}

impl<'a, W: Write> Notify<'a, W> {
    pub fn method(&mut self, method: &str) -> Result<(), Box<dyn Error>> {
        assert_eq!(NotifyState::WaitMethod, self.state);
        rmp::encode::write_array_len(self.buf, 3)?;
        rmp::encode::write_uint(self.buf, 2)?;
        rmp::encode::write_str(self.buf, method)?;
        self.state = NotifyState::WaitParamCount;
        Ok(())
    }
    pub fn param_count(&mut self, count: u8) -> Result<(), Box<dyn Error>> {
        assert_eq!(NotifyState::WaitParamCount, self.state);
        rmp::encode::write_array_len(self.buf, count.into())?;
        self.state = NotifyState::WaitParam(count);
        Ok(())
    }
    pub fn param(&mut self, param: &str) -> Result<(), Box<dyn Error>> {
        if let NotifyState::WaitParam(pending_param_count) = self.state {
            assert!(pending_param_count > 0);
            rmp::encode::write_str(self.buf, param)?;
            self.state = if pending_param_count > 1 {
                NotifyState::WaitParam(pending_param_count - 1)
            } else {
                NotifyState::Sent
            }
        } else {
            panic!("Invalid state {:?}.", self.state)
        }
        Ok(())
    }
}

impl<'a, W: Write> Request<'a, W> {
    pub fn method(&mut self, method: &str) -> Result<(), Box<dyn Error>> {
        assert_eq!(RequestState::WaitMethod, self.state);
        rmp::encode::write_array_len(self.buf, 4)?;
        rmp::encode::write_uint(self.buf, 0)?;
        rmp::encode::write_uint(self.buf, self.msgid.into())?;
        rmp::encode::write_str(self.buf, method)?;
        self.state = RequestState::WaitParamCount;
        Ok(())
    }
    pub fn param_count(&mut self, count: u8) -> Result<(), Box<dyn Error>> {
        assert_eq!(RequestState::WaitParamCount, self.state);
        rmp::encode::write_array_len(self.buf, count.into())?;
        self.state = RequestState::WaitParam(count);
        Ok(())
    }
    pub fn param(&mut self, param: &str) -> Result<(), Box<dyn Error>> {
        if let RequestState::WaitParam(pending_param_count) = self.state {
            assert!(pending_param_count > 0);
            rmp::encode::write_str(self.buf, param)?;
            self.state = if pending_param_count > 1 {
                RequestState::WaitParam(pending_param_count - 1)
            } else {
                RequestState::Sent
            }
        } else {
            panic!("Invalid state {:?}.", self.state)
        }
        Ok(())
    }
}
