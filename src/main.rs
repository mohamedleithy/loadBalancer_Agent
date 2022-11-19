
// importing the udp standard library
use std::net::UdpSocket;
use std::fs::File;
use std::thread;
use std::time::Duration;
use rand::Rng;
extern crate local_ip;

use std::io::{prelude::*, BufReader};
use std::path::Path;


// fn loadIpAddresses(filename: impl AsRef<Path>) -> Vec<String> {
//     let file = File::open(filename).expect("no such file");
//     let buf = BufReader::new(file);
//     buf.lines()
//         .map(|l| l.expect("Could not parse line"))
//         .collect()
// }


fn main() -> std::io::Result<()>{
    {
        
       

        let ip = local_ip::get().unwrap();

        // thread to initiate requests to servers, upon recieving a request from clients 
        // Reserve port 2020 for agent on the machine
        let handler = thread::spawn(move || {
        // Load Ip Addresses of Servers from configuration file 
        // The random generator is to randomize the starting node of each agent as not to initiate load from all agents to the first server
        // ex: Agent 1: 0 1 2 Agent 2: 2 0 1 Agent 3: 1 2 0 

            // let ipAddresses= loadIpAddresses("/home/g02-f22/Desktop/loadBalancer_Agent/config.txt");
            // let mut rng = rand::thread_rng();
            // let mut n = rng.gen_range(0, ipAddresses.len());
            let clientToAgentMsg = "ClienToAgentMsg::";

        let socket = UdpSocket::bind(ip.to_string() + ":2020").unwrap();
            loop {



                let mut buf = [0; 30]; // buffer for recieving 

                
                // println!("Random number: {n}");

                // recieve from client 
                let (amt, src) = socket.recv_from(&mut buf).unwrap();

                thread::sleep(std::time::Duration::from_millis(1000));

                //appending the client ip to the message
                let mut buf1 = src.to_string().into_bytes();
                buf1.append(&mut buf.to_vec());
                
                // send to servers in a round robin fashion 
                socket.send_to(&buf1, "192.168.1.3:2023").unwrap();
                println!("{} Forwarded message from client to server", clientToAgentMsg);

                // move to next in order 
                //  n += 1;
                //  n = n%ipAddresses.len(); 
    
            }
            
            });

            // thread to receive replies from servers, and sending back to client 

            // receive on send port + 1 (2021)

            let handler1 = thread::spawn(move || {
                let socket = UdpSocket::bind(ip.to_string() +":2021").unwrap();
                let agentToClientMsg = "AgentToClientMsg::";  
                loop {
                        
                        let mut buf = [0; 60]; // buffer for recieving 

                        // blocked till Recieving a message from any of the other servers 

                        // recieve from server 
                        let (amt, src) = socket.recv_from(&mut buf).unwrap();
                        println!("{} Message Recieved!", agentToClientMsg);



                        // extract src from buf1
                        let mut src1 = [0; 30];
                        src1.copy_from_slice(&buf[30..60]);
                        src1.reverse();
                        let src1 = String::from_utf8((&src1).to_vec()).unwrap();
                        let src1 = src1.trim_matches(char::from(0));
                        println!("client ip: {}", src1);


                        thread::sleep(std::time::Duration::from_millis(1000));
                        //convert src1 To Socket Address
                        let src1 = src1.parse::<std::net::SocketAddr>().unwrap();

                        // send to client
                        socket.send_to(&buf[0..30], src1).unwrap();

                        // TODO 
                        /*Requires engineering how to get the ip address of the client*/
                        println!("{}From: {:?}", agentToClientMsg, src1);
                        //print the received data as a string 

                        println!("Message: {}", String::from_utf8_lossy(&buf[0..30]));
                 
                    }
                        
                });



        handler.join().unwrap();
        handler1.join().unwrap();
    }


    Ok(())
    

}
