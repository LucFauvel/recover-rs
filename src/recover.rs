use std::io;
use crate::utils::message_consts::WELCOME_MESSAGE;

pub struct Recover {
    term: console::Term,
}

impl Recover {
    pub fn new() -> Self {
        Recover {
            term: console::Term::stdout(),
        }
    }

    pub fn run(&self) -> io::Result<()> {
        self.show_welcome_message()?;
        self.prompt_disk_select()?;
        Ok(())
    }

    pub fn show_welcome_message(&self) -> io::Result<()> {
        self.term.write_line(WELCOME_MESSAGE)
    }

    pub fn prompt_disk_select(&self) -> io::Result<()> {
        //Plan is to do a layer of abstraction with a struct called
        //Disk or something and use that aggragate all data that is platform dependant
        //The obstacle I'm trying to figure out now is how to access the raw bits of the disks
        //I have an idea for linux/unix platforms but not for windows or for macos
        //so this is a big ass TODO
        //Also, I should start designing algorithms for file reconstruction since that is a big part
        //of file recovery so that's an other big ass TODO
        Ok(())
    }
}