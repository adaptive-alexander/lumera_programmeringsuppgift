# Rust Solution

## Quickstart

#### If Installation is Needed (Linux and Windows binaries supplied in ./target/release)
* install the rust toolchain (suggest using rustup)
* `cd ./programmeringsuppgift_rust`
* `cargo build --release`

#### Running Tests
- cargo test

#### Using the Application
* Run application and pass file path in interactive prompt

Example:
* `target/release/programmeringsuppgift_rust` (add .exe on Windows)
* `./test_data/Exempelfil_betalningsservice.txt`

Example (cross-platform) with source:
* `cargo run --release`
* `./test_data/Exempelfil_betalningsservice.txt`

The program will run a parser on the file dependent on the file ending.

The std output is a simulated call to the java-interface, with the assumption that FFI or compiling static
libraries is out of the scope of the task. 

The code is timed as an extra final output to make sure implementation is sufficiently efficient. Time given 
in micro seconds. The test files run in the area of 2000 µs, which includes reading, parsing and generating output 
for the file.

#### Library Development
Adding new formats requires the following steps:
* Write a new parser for the file format as a separate public function in the parsers mod.
  * The function takes as argument a line buffer of the file.
  * The function returns a tuple of `(StartPaymentData, Vec<PaymentData>)` (see payment_file_handler module).
* Add a clause to if statement in function payment_file_handler to match the file ending of your new file calling your parser.

## Continued Development

### Unstable Areas

Data passed on has static types ensuring correctness. The data parsed, however, is only tested for files with correct format.

There is a good bit of edge-case handling that would be a good start in order to improve the current parser code. For this
release to be considered stable the assumption has to be made that files are correctly formatted according to the specification.

There is currently no user-friendly refutation of files with good developer facing error messages on files being incorrect,
with the exception of parser errors describing which type could not be parsed. Lines, file name, specific error type etc 
should be handled better using custom error structs passed up the stack to make the stacktrace simple to debug.

Currently the program panics on errors in file path. This should be changed to soft errors passed up the stack so that the
application does not need to be restarted on faulty input. This should most likely be handled using custom error types.

### Optimizations 

There are several areas the code could be optimized to run faster, particularilly on large files.
Multi-processing would probably be the place to start, and is only a few lines of code with `Rayon`. This, however,
is only meaningful if the files start getting large (at least a few thousand lines). A further optimization could be to
pass a folder path instead of a file, to then read files async (assuming networking is involved, takes a few lines with 
`Tokio`) and process them in parallel.

### Development Scalability

In order to make writing the code even more scalable the current structure allows for a relatively simple redefinition of the
parsers through structs that can then be serialized (`serde`). The upside is that writing or calling new parsers take very
little code, as serialization can be derived. The downside is that files have to follow a rigid structure and low customization 
available. Another option is to create a `parser Trait` and enforce parsers to implement the required methods. This is sort of a 
compromise between the two, reducing the amount of code needed but allowing for customization. There is no meaningful performance 
difference between the approaches as long as the trait method does not use `trait objects`, since this would require dynamic dispatch.

### Deployment Scalability

The target could be easily containerized with a few lines of docker. This implementation would require an interface of some sort,
out of which the easiest is probably a simple REST-API in `actix-web` (async and parallel by default) with a single endpoint. This
would require an additional 30-40 lines of code to include authentication, CORS etc. 

As the application is stateless, (near) infinite scalability could be handled using any available Kubernetes scaling 
(replicas, resource requests, node size, nodes). The protocol for complete files should probably follow ACID rules, 
and simply fail if all parts do not succeed considering the small computation time of each file. Scheduling could otherwise
be handled with a message queue (RabbitMQ/Google PubSub etc) and files considered failed unless ack'd messages on last operation.
This would require a simple k8 deployment and service (probably of type NodePort or Load Balancer).

For redundancy this solution should have a check from the caller that data is cleaned on fail, alternatively include known
unique database identifiers for files run a second time to ensure no double deliveries.

Deployment could probably use a relatively standard Blue/Green strategy. When stability has been ensured, unit- and live tests
passed, shut down the blue version and announce updated features. If downtime is allowed (no customer activity over weekend?)
a simpler recreate strategy might be used to keep costs down.
