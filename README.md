
# mqtt-kafka-rust-bridge👋
This was a one day sprint to port small node repo to start learning rust. It's far from great and please dont use this for anything.

see: [matt-kafka-bridge](https://github.com/nodefluent/mqtt-to-kafka-bridge)

The goal of the original repo was create a small server that would listen to an mqtt broker and forward messages to kafka in a directional manner. This holds true for this sprint and it seems to do so reasonably well. Although it's in rust given it's a pile I imagine it's slower than the node implementation.


###One Day:
- I hope to benchmark it and speed it up
- Setup better testing.
- Incorporate etl logic
- Clean up the threading model and cut down on how much is copied.
- Explore if I could add a bidirectional stream

## Up And Running
### 1. setup an mqtt broker and kafka server
```bash
docker-compose up
```

This will setup mqtt on 1883 and kafka on 9092. You can pick your respective client to see what's going on if you like.

### 2. try a build 
```bash
cargo build . 
```
note that you may need to change the openssl distribution to reference your local. On an M1 vendorded seemed to be the only one that seemed to work.

### 3. try the tests 
```bash
cargo test 
```
The main test is setup to emulate a server so for now you'll need to kill it.


### 4. try main test
```bash
cargo test 
```