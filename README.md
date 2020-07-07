# andrew_control_protocol
acp: a simple reliability layer over UDP utilizing a sliding window

To run:
"make run_receiver"

(in a separate shell):
"make run_sender"

Running the protocol example will copy the infile contents to the outfile via the network protocol.

The makefile provides the following default arguments.
IP: 127.0.0.1
Port: 9999
Starting WIN: 1
Infile: infile.txt
Outfile: outfile.txt

Custom args can be utilized by running the CLI directly and not running the make targets.
