# rest_nethelper

rest_nethelper is a simple REST API that generates an example of a network as an SVG image. 

While there is still room for improvement, such as adding a list of allowed origins for better security, I have chosen to keep it as simple and functional as possible.

This project is not meant to be used in a production environment, but rather as a proof of concept.


## Example query
`http://localhost:3001/ipv4?n=192.168.1.0&r=192.168.1.254&h0=192.168.1.1&h1=192.168.1.253&br=192.168.1.255`