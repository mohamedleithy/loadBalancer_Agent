
// importing the udp standard library
use std::net::UdpSocket;
use std::fs::File;
use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() -> std::io::Result<()>{
    {

        // thread to initiate requests to servers, upon recieving a request from clients 
        let handler = thread::spawn(move || {
        let socket = UdpSocket::bind("192.168.1.3:5966").unwrap();

            loop {
                println!("Sending message to other servers");
                let mut rng = rand::thread_rng();
                let n = rng.gen_range(0, 3);
                
                
                println!("Random number: {n}");
                
            
                 socket.send_to(b"Request recieved from: ", "192.168.1.3:5960").unwrap();
  

               thread::sleep(Duration::from_millis(1000));
    
            }
            
            });

            // thread to receive replies from servers, and sending back to client 

            let handler1 = thread::spawn(move || {
                let socket = UdpSocket::bind("192.168.1.3:5960").unwrap();
        
                loop {
                        println!("Recieving messages from other servers");
                        let mut buf = [0; 30]; // buffer for recieving 

                        // blocked till Recieving a message from any of the other servers 

                        let (amt, src) = socket.recv_from(&mut buf).unwrap();



                        println!("Message Recieved!");

                        println!("From: {:?}", src);
                        //print the received data as a string 

                        println!("Message: {}", String::from_utf8_lossy(&buf));



                        thread::sleep(Duration::from_millis(1000));
                 
                    }
                        
                });




        handler.join().unwrap();
        handler1.join().unwrap();
    }

    Ok(())
    

}
