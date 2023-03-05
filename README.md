# redis-state-machine

[![CI](https://github.com/RedisLabsModules/redis-state-machine/actions/workflows/integration.yml/badge.svg)](https://github.com/RedisLabsModules/redis-state-machine/actions/workflows/integration.yml)

A [Redis module](https://redis.io/docs/modules) that maintains a state machine on the server side.

** Notice, this is experimental, and under active development. **

## Build

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