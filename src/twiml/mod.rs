pub mod voice;
use crate::error::TwilioError;
use xml::writer::EventWriter;

pub trait ToTwiML {
    fn write_xml(&self, writer: &mut EventWriter<Vec<u8>>) -> Result<(), TwilioError>;
}
