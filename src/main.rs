
// importing the udp standard library
use std::net::UdpSocket;
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::process::{self, exit};
use rand::Rng;
use std::sync::{Arc, Mutex};
extern crate local_ip;
use std::io;

use std::io::{prelude::*, BufReader};
use std::path::Path;


struct server{
    ip: String,
    state: bool,
    temperature: u8,
}


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

        // maintaing server info to send to servers and mark down servers 

        let temp_servers: [server; 3] = [server { ip: ip.to_string(), state: true, temperature: 100}, server { ip: "172.20.10.6".to_string(), state: true, temperature: 100 }, server { ip: "172.20.10.3".to_string(), state: true, temperature: 100},];
        
        let server_info = Arc::new(Mutex::new(temp_servers));
        // thread to initiate requests to servers, upon recieving a request from clients 
        // Reserve port 2020 for agent on the machine


    let server_info_1 = Arc::clone(&server_info);
        let handler = thread::spawn(move || {
        // Load Ip Addresses of Servers from configuration file 
        // The random generator is to randomize the starting node of each agent as not to initiate load from all agents to the first server
        // ex: Agent 1: 0 1 2 Agent 2: 2 0 1 Agent 3: 1 2 0 

            // let ipAddresses= loadIpAddresses("/home/g02-f22/Desktop/loadBalancer_Agent/config.txt");
            // let mut rng = rand::thread_rng();
            // let mut n = rng.gen_range(0, ipAddresses.len());
            let clientToAgentMsg = "ClienToAgentMsg::";

        
        let socket = UdpSocket::bind(ip.to_string() + ":2020").unwrap();

        let mut server_info_11 = server_info_1.lock().unwrap();
            // inform all servers that this agent is up. 
            // TODO: LOOP ON ALL SERVERS, use for loop 

            socket.send_to(b"1", ip.to_string() + ":2030").unwrap(); 
            std::mem::drop(server_info_11); 
            
            loop {

                let mut buf = [0; 30]; // buffer for recieving 

                
                // println!("Random number: {n}");

                // recieve from client 
                let (_, src) = socket.recv_from(&mut buf).unwrap();

                thread::sleep(std::time::Duration::from_millis(1000));

                //appending the client ip to the message
                let mut buf1 = src.to_string().into_bytes();
                buf1.append(&mut buf.to_vec());
                

                // send to servers in a round robin fashion TODO: USE N 

                // check if N is not down otherwise, send to N+1 
                socket.send_to(&buf1, "192.168.1.3:2023").unwrap();
                println!("{} Forwarded message from client to server", clientToAgentMsg);

                // move to next in order 
                //  n += 1;
                //  n = n%server_info_11.len(); 
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




                    // thread to update server status

            // receive on send port + 2 (2022)

            let handler2 = thread::spawn(move || {
                let socket = UdpSocket::bind(ip.to_string() +":2022").unwrap();
                let serverToAgentMsg = "ServerToAgentMsg::";  
                loop {
                        let mut buf = [0; 60]; // buffer for recieving 

                        // blocked till Recieving a message from any of the other servers 

                        // recieve from server 
                        let (_, src) = socket.recv_from(&mut buf).unwrap();
                        println!("{} server with ip: {} is down!", serverToAgentMsg, src);


                        // TODO: MARK THIS SERVER AS DOWN OR UP AS PER THE MESSAGE
            
                    }
                        
                });



// UPON TERMINATING INFORM SERVERS THAT THIS AGENT IS OFF 

let handler3 = thread::spawn(move || {
    let socket = UdpSocket::bind(ip.to_string() +":4000").unwrap();
    let agentToServerMsg = "AgentToServerMsg::";  
    loop {
         

            // TODO LOOP ON SERVER TO ANNOUNCE TERMINATION 
        

            let mut user_input = String::new();
            let stdin = io::stdin(); // We get `Stdin` here.
            stdin.read_line(&mut user_input).unwrap();
        if user_input == "exit\n" {
            
           socket.send_to(b"0", ip.to_string() + ":2030").unwrap(); 
           exit(0);
        }
            


            // TODO: MARK THIS SERVER AS DOWN OR UP AS PER THE MESSAGE

        }
            
    });



        handler.join().unwrap();
        handler1.join().unwrap();
        handler2.join().unwrap(); 
        handler3.join().unwrap(); 
    }


    Ok(())
    

}
