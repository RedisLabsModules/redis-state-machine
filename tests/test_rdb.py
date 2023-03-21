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
    assert 150 <= r.memory_usage("memfoo") <= 200


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
