import json


def test_memory_usage(r):
    r.flushdb()
    r.execute_command(
        "SM.SET",
        "memfoo",
        json.dumps(
            {"initial": "aval", "map": {"a": ["b", "c"]}, "current": "b", "reason": ""}
        ),
    )
    assert r.memory_usage("memfoo") == 152


def test_copy(r):
    r.flushdb()
    r.execute_command(
        "SM.SET",
        "memfoo",
        json.dumps(
            {"initial": "aval", "map": {"a": ["b", "c"]}, "current": "b", "reason": ""}
        ),
    )
    assert r.copy("memfoo", "barfoo")
    assert r.type("barfoo") == "StateType"
