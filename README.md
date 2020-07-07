# andrew_control_protocol
ACP: a simple reliability layer over UDP utilizing a sliding window

To run:
"make run_receiver"

(in a separate shell):
"make run_sender"

Running the protocol example will copy the infile contents to the outfile via the network protocol.

The makefile provides the following default arguments. <br/>
IP: 127.0.0.1 <br/>
Port: 9999 <br/>
Starting WIN: 1 <br/>
Infile: infile.txt <br/>
Outfile: outfile.txt <br/> 

Custom args can be utilized by running the CLI directly and not running the make targets.

You can view the example output logs without running the program itself. Simply view "example_receiver_log.txt" and "example_sender_log.txt" files.
