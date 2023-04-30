# nethelper_rest

nethelper_rest is a simple REST API that mainly generates and returns an SVG of a network.  
While there is still room for improvement, such as adding a list of allowed origins for better security, I have chosen to keep it as simple and functional as possible.  
This project is not meant to be used in a production environment, but rather as a proof of concept.  
## Endpoints
`/ipv4`: Generates and returns the SVG of a network  
`/history`: Returns the saved SVG  
`/save`: Requests to save a generated SVG  
## Example usage
Here is an example query for the `/ipv4` endpoint:
`http://localhost:3001/ipv4?n=192.168.1.0&r=192.168.1.254&h0=192.168.1.1&h1=192.168.1.253&br=192.168.1.255`
## Installation and Setup
To use nethelper_rest, you will need to download [SQLite](https://www.sqlite.org/download.html) and follow these steps:
1. Clone the repository and navigate to the root directory
2. Run `cargo b -r` to build the application
3. Navigate go `.\target\release\`
4. Run `./nethelper_rest.exe` to start the server
5. The sever will create an SQLite database named `history.db` in the same directory where the program is located

## Limitations
1. The file `src/main.rs` contains a hardcoded URL, which means it was designed to be used on localhost only. If you plan on hosting this project on a server, you'll need to change `SocketAddr::from(([127, 0, 0, 1], 3001))` to `SocketAddr::from(([0, 0, 0, 0], 3001))` in `main.rs`. You may also want to consider changing the port.
2. As this project is intended to be a proof of concept, all requests must be made using the `http` protocol. If you try to access the website using `https`, you may encounter a `Mixed Content` error. A temporary workaround for this issue is to enable `Insecure Content` for that page only.

