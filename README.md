# redis-state-machine

[![Latest Release](https://img.shields.io/github/v/release/redislabsmodules/redis-state-machine?label=latest)](https://github.com/redislabsmodules/redis-state-machine/releases/latest)
[![Dockerhub](https://img.shields.io/badge/dockerhub-redislabs/redisstatemachine-blue)](https://hub.docker.com/r/redislabs/redisstatemachine/tags/)

A [Redis module](https://redis.io/docs/modules) that maintains a state machine on the server side.

** Notice, this is experimental, and under active development. **

## Installation

The easiest way to investigate this capability, is to pull the latest docker. The following runs a redis instance on the default port, complete with a state machine.

```docker run -p 6379:6379 -it redislabs/redisstatemachine:edge```

## Basic Usage

A state machine is a JSON document containing key value pairs for machine state transitions. Let's look at the state machine template, via the ```SM.TEMPLATE``` command.

```bash
redis-cli
SM.TEMPLATE
```

A state machine contains a map of states, a current state, and an initial state.
```"{\"current\":\"\",\"map\":{},\"initial\":\"\"}"```

Create a simple machine

```bash
redis-cli
SM.SET mymachine "{\"current\":\"\",\"map\":{},\"initial\":\"foo\"}"
```

Examine the machine via ```SM.GET```.

```bash
redis-cli
SM.GET mymachine
```

As we didn't specify a current state, the initial state is assumed:

```"{\"current\":\"foo\",\"map\":{},\"initial\":\"foo\"}"```

## A practical example

Let's create and interact with a complete state machine, using python, and [redis-py](https://github.com/redis/redis-py). First, let's load the state machine template:

```python

import redis
import json

r = redis.Redis(decode_responses=True)
tmpl = json.loads(r.execute_command("SM.TEMPLATE"))
```

Let's create a simple machine named *mystatemachine*. The default state will be foo, and we'll support states named ```bar```, ```blee```, and ```boo```.  In our state machine, ```bar``` will be able to move to states ```blee``` and ```boo```, but only ```boo``` will be able to return to blee.

```python

tmpl['initial'] = 'bar'
tmpl['map'] = {"bar": ["blee", "boo"], "boo": ["blee"]}
r.execute_command("SM.SET", "mystatemachine", json.dumps(tmpl))
```



If we try to change our statemachine to an invalid state, Redis returns a nil.

```python
x = r.execute_command("SM.MUTATE", "mystatemachine", "notastate")
print(x)
>>> None
```

If we try to change states to a valid state, we receive an ok.

```python
x = r.execute_command("SM.MUTATE", "mystatemachine", "blee")
print(x)
>>> OK
```

Example the state machine, using the redis-cli we can example the machine using ```SM.GET```.

```python
r.execute_command("SM.GET", "mystatemachine")
```

Notice the structure - we now have a map of states and transitions.
```'{"current":"blee","map":{"bar":["blee","boo"],"boo":["blee"]},"initial":"bar"}'```

As this is currently evolving, the canonical command set is available in the [commands.json](./commands.json) file, with usage tests available in the [tests directory](./tests/test_commands.py).

----

## Building and contributing

### Tooling

* [Rust](https://www.rust-lang.org)

* GNU Make

* Python >= [this file](.python-version)

* Redis

**Build and run**

```bash
make run
```

### Testing

Module testing is currently integration based - and done with pytest, and python.  

Set up a virtual environment, with your test dependencies

```
python -m venv .venv
source .venv/bin/activate
pip install -r tests/requirements.txt
```

Run the tests
```
pytest
```
