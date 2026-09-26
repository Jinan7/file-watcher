use std::{fs::File, io::Write, sync::{Arc, Mutex}};

use rand::{RngExt, distr::Alphanumeric};
use watchexec::Watchexec;
use watchexec_events::filekind::FileEventKind::{Modify, Remove};
use watchexec_signals::Signal;


#[tokio::main]
async fn main() {

    let modify_signal = Arc::new(Mutex::new(false));
    let modify_signal_clone = Arc::clone(&modify_signal);
    let wx = Watchexec::new(move |mut action| {

        println!("New action received");
        for event in action.events.iter() {
            
            for tag in event.tags.iter() {
                if let watchexec_events::Tag::FileEventKind(event_kind) = tag {
                    match event_kind {
                        Modify(_) => {
                            *modify_signal_clone.lock().unwrap() = true;    
                        }
                        Remove(_) => {
                            *modify_signal_clone.lock().unwrap() = true;    
                        }
                        _ => ()
                    }
                }
            }
            
        }

        if action
            .signals()
            .any(|sig| matches!(sig, Signal::Interrupt | Signal::Terminate))
            {

                action.quit();
            }

        action
    })
    .expect("Error");

    wx.config.pathset(["./testfile.txt"]);

    let handle = wx.main();
   
    
    loop {
        
        if *modify_signal.lock().unwrap() == true{
            println!("Modify signal received");
            break;
        }
    
    }
    
    handle.await.unwrap().unwrap();
}


fn create_file(
    key: &str
) -> Result<File, anyhow::Error> {

    let mut file = File::create(key).expect("could not create file");

    
    let rand_string: String = rand::rng()
    .sample_iter(Alphanumeric)
    .take(256)
    .map(char::from)
    .collect();
    
    let return_string = "\n".to_string();

    file.write_all(rand_string.as_ref()).expect("Error writing to file");
    file.write_all(return_string.as_ref()).expect("Error writing to file");
    

    Ok(file)
}
