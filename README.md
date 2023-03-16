# redis-state-machine

[![Latest Release](https://img.shields.io/github/v/release/redislabsmodules/redisstatemachine?label=latest)](https://github.com/redislabsmodules/redisstatemachine/releases/latest)

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