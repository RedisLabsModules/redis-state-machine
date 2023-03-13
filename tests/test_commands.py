from redis.exceptions import ResponseError
import pytest
import json


def test_invalid_set_get(r):
    r.flushdb()
    r.set("foo", "bar")
    with pytest.raises(ResponseError):
        r.execute_command("SM.GET", "foo")


def test_set_get(r):
    r.flushdb()

    initial = "begin"
    current = "somewhere"
    mapstates = {
        "a": ["this", "maps", "states"],
        "b": ["this", "too", "maps", "somewhere"],
    }
    invalid_mapstates = {"a": "badmap", "b": ["this", "is", "a", "good", "map"]}
    invalid_no_current = {"initial": initial, "map": mapstates}

    with pytest.raises(ResponseError):
        r.execute_command("SM.SET", "foo", json.dumps(invalid_mapstates))
        r.execute_command(
            "SM.SET", "foo", json.dumps({"initial": initial, "map": invalid_mapstates})
        )
        r.execute_command(
            "SM.SET", "foo", json.dumps({"current": current, "map": invalid_mapstates})
        )
        r.execute_command(
            "SM.SET", "foocurrent", json.dumps(invalid_no_current), current
        )

    valid_with_current = {"initial": initial, "map": mapstates, "current": current}
    assert r.execute_command("SM.SET", "bar", json.dumps(valid_with_current))

    bar = json.loads(r.execute_command("SM.GET", "bar"))
    assert bar == valid_with_current


def test_get_current(r):
    r.flushdb()
    initial = "begin"
    mapstates = {
        "a": ["this", "maps", "states"],
        "b": ["this", "too", "maps", "somewhere"],
    }
    validmap = {"initial": initial, "map": mapstates, "current": "begin"}
    assert r.execute_command("SM.SET", "fooforcurrent", json.dumps(validmap))
    assert r.execute_command("SM.CURRENT", "fooforcurrent") == "begin"


def test_get_states(r):
    r.flushdb()
    initial = "begin"
    mapstates = {
        "a": ["this", "maps", "states"],
        "b": ["this", "too", "maps", "somewhere"],
    }
    validmap = {"initial": initial, "map": mapstates, "current": "too"}
    assert r.execute_command("SM.SET", "foostates", json.dumps(validmap))
    states = r.execute_command("SM.STATES", "foostates")

    mapkeys = list(mapstates.keys())
    mapkeys.sort()
    states.sort()
    assert mapkeys == states


def test_set_del(r):
    r.flushdb()
    initial = "begin"
    mapstates = {
        "a": ["this", "maps", "states"],
        "b": ["this", "too", "maps", "somewhere"],
    }
    validmap = {"initial": initial, "map": mapstates, "current": "shmm"}
    assert r.execute_command("SM.SET", "foostates", json.dumps(validmap))
    assert r.execute_command("SM.DEL", "foostates")

    keys = r.keys()
    assert "foostates" not in keys


def test_reset(r):
    r.flushdb()
    initial = "begin"
    mapstates = {
        "a": ["this", "maps", "states"],
        "b": ["this", "too", "maps", "somewhere"],
    }
    validmap = {"initial": initial, "map": mapstates, "current": "too"}
    assert r.execute_command("SM.SET", "foostates", json.dumps(validmap))
    r.execute_command("SM.RESET", "foostates")
    assert r.execute_command("SM.CURRENT", "foostates") == initial


# def test_force_set(r):
#     r.flushdb()
#     initial = "begin"
#     mapstates = {
#         "a": ["this", "maps", "states"],
#         "b": ["this", "too", "maps", "somewhere"],
#     }
#     validmap = {"initial": initial, "map": mapstates, "current": "maps"}
#     assert r.execute_command("SM.SET", "foostates", json.dumps(validmap))
#     assert r.execute_command("SM.FORCE", "foostates", "too")

#     assert r.execute_command("SM.CURRENT", "foostates") == "too"


def test_create(r):
    r.flushdb()
    key = "foo"
    assert r.execute_command("SM.CREATE", key)
    assert r.execute_command("SM.CURRENT", key) == ""
    res = r.execute_command("SM.GET", key)
    val = json.loads(res)
    assert val["initial"] == ""
    assert val["map"] == {}
    assert val["current"] == ""


#     r.execute_command("SM.FORCE", "foostates", "too")


def test_template(r):
    res = r.execute_command("SM.TEMPLATE")
    val = json.loads(res)
    assert val["initial"] == ""
    assert val["map"] == {}
    assert val["current"] == ""

def test_transition(r):
    r.flushdb()
    initial = "begin"
    current = "begin"
    mapstates = {
        "a": ["this", "maps", "states"],
        "b": ["this", "too", "maps", "somewhere"],
        "begin": ["too", "maps"],
    }
    
    valid_with_current = {"initial": initial, "map": mapstates, "current": current}
    assert r.execute_command("SM.SET", "bar", json.dumps(valid_with_current))
    assert r.execute_command("SM.TRANSITION", "bar", "smurfy") == None
    assert r.execute_command("SM.TRANSITION", "bar", "too")

    # force state
    r.flushdb()
    assert r.execute_command("SM.SET", "bar", json.dumps(valid_with_current))
    assert r.execute_command("SM.TRANSITION", "bar", "banna", "foo") == None
    assert r.execute_command("SM.TRANSITION", "bar", "banna", "F")