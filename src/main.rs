use tts::*;

fn main() -> Result<(), Error> {
    let mut tts = Tts::default()?;
    tts.speak("hello im a voice assistant and im here to help you with a variety of your tasks", false)?;
    tts.speak("hello sir", false)?;
    print!("hello");
    while tts.is_speaking()? == true{
        print!("wait");
    }
    Ok(())
}