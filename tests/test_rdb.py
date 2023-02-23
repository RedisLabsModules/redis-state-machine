import json

def test_memory_usage(r):
    r.flushdb()
    r.execute_command("SM.SET", "memfoo", json.dumps({"initial": "aval", "map": {"a": ["b", "c"]}}))
    assert r.memory_usage("memfoo") == 128