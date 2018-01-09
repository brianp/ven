ven - A virtual event network
=====

<img src="https://raw.githubusercontent.com/brianp/ven/master/ven.png" alt="A Venn Diagram containing nothing at all" align="right" />

## We're here to recreate file system events from a host machine onto a virtual machine. Quickly.
Ever ran a vm with a shared directory? Ever made a change to a file on the host
machine, expecting a fs event to trigger in the VM?

Then *ven* is for you.
(I'm looking at you vagrant shared directory and guard | grunt | gulp | watcher systems.)

## Setup

### It takes two

The program runs in one of two modes. Stream, or Actor. And requires you to run each
mode in a differnt place.

#### Stream

The stream mode binds to a local udp port (broadcast address) and sits around
listening for OS events. When it receives one it broadcasts the event to the
bind address. That's it. Listen, and yell.

#### Actor

The actor hangs around bound to the bind address just waiting for a juicy
event to come in. When it gets one, it tries its hardest to mimic it.
Recreating a local fs event on a different system.

### Setup the Stream on the system you'd like to capture events _from_

`ven --stream`

### Setup the Actor on the system you'd like to to trigger events _on_

`ven --actor`

Now sit back and watch it play out

## Usage Options

```shell
Usage:
    ven [--stream | --actor] -b <bind_address>
    ven [-s | -a]
    ven [-s | -a] -b <bind_address>
    ven (-h | --help)
    ven (-v | --version)

Flags:
    -s, --stream        Stream mode to stream OS events to a bound connection [default]
    -a, --actor         Actor mode for reading events form the port and triggering events
    -h, --help          Prints help information
    -v, --version       Prints version information

Options:
    -b <bind_address>       The address and port you want to send to and listen from [default: 127.0.0.1:34254]
    -r <broadcast_address>  A host socket to send from if broadcasting and receiving from the same machine. Not normally required [default: 127.0.0.1:45243]
```

## Copyright
Copyright (c) 2017-2018 Brian Pearce. See [LICENSE](https://github.com/brianp/ven/blob/master/LICENSE) for further details.
