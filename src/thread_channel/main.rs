use std::sync::mpsc;
use std::thread;

enum Message {
    Number(i32),
    Text(String),
}

fn receive_message(rx: mpsc::Receiver<Message>)
{
    for received in rx
    {
        match received {
            Message::Number(num) => {
                println!("Received number: {}", num);
            }
            Message::Text(text) => {
                println!("Received text: {}", text);
            }
        }
    }
}

fn main()
{
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // loop
        {
            let message = Message::Number(32);
            tx.send(message).unwrap();
            let message = Message::Text(String::from("Channel Test!"));
            tx.send(message).unwrap();
        }
    });
    receive_message(rx);

    thread::sleep(std::time::Duration::from_secs(1));
}
