# Rust RELP Server

An implementation of the Reliable Event Logging Protocol (RELP) in Rust, [wikipedia article.](http://www.rsyslog.com/doc/relp.html)

The protocol is defined [here](http://www.rsyslog.com/doc/relp.html)

The aim here to port a relp server I wrote previously in java & netty as a way to learn Rust & [tokio.io](https://github.com/tokio-rs/tokio-io)

### TODOs
1. It currently uses \n delimiting for message splitting which is wrong.
The regex used to parses messages needs to be open ended and then message split according to datalen parameter
2. Move some of the protocol details from Service to Codec or protocol. I'm still learning rust and tokio.io so I couldnt figure it out. If is moved, this could be used a library where clients implement custom services, 
eg write to a logfile, persist to a db etc. 