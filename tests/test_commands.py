from redis.exceptions import ResponseError
import pytest
import json


def genmap():
    """return a n empty hashmap"""
    initial = "begin"
    current = "begin"
    mapstates = {
        "a": ["this", "maps", "states"],
        "b": ["this", "too", "maps", "somewhere"],
        "begin": ["too", "maps"],
        "too": ["b"],
    }

    return {"initial": initial, "map": mapstates, "current": current, "reason": ""}


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

    sm = genmap()
    assert r.execute_command("SM.SET", "bar", json.dumps(sm))

    bar = json.loads(r.execute_command("SM.GET", "bar"))
    assert bar == sm


def test_get_current_state(r):
    r.flushdb()
    assert r.execute_command("SM.SET", "fooforcurrent", json.dumps(genmap()))
    assert r.execute_command("SM.STATE", "fooforcurrent") == [genmap()["current"]]


def test_get_states(r):
    r.flushdb()
    sm = genmap()
    assert r.execute_command("SM.SET", "foostates", json.dumps(sm))
    states = r.execute_command("SM.STATE", "foostates", "list")

    mapkeys = list(sm["map"].keys())
    mapkeys.sort()
    states.sort()
    assert mapkeys == states


def test_get_reason(r):
    r.flushdb()
    sm = genmap()
    sm["reason"] = "I am the reason just because"
    assert r.execute_command("SM.SET", "foostates", json.dumps(sm))
    assert r.execute_command("SM.GET", "foostates", "reason") == sm["reason"]
    assert r.execute_command("SM.GET", "foostates", "REASON") == sm["reason"]


def test_set_del(r):
    r.flushdb()
    assert r.execute_command("SM.SET", "foostates", json.dumps(genmap()))
    assert r.delete("foostates")

    keys = r.keys()
    assert "foostates" not in keys


def test_reset(r):
    r.flushdb()
    sm = genmap()
    assert r.execute_command("SM.SET", "foostates", json.dumps(sm))
    r.execute_command("SM.RESET", "foostates")
    assert r.execute_command("SM.STATE", "foostates") == [sm["initial"]]


def test_create(r):
    r.flushdb()
    key = "foo"
    assert r.execute_command("SM.CREATE", key)
    assert r.execute_command("SM.STATE", key) == [""]
    res = r.execute_command("SM.GET", key)
    val = json.loads(res)
    assert val["initial"] == ""
    assert val["map"] == {}
    assert val["current"] == ""
    assert val["reason"] == ""


def test_template(r):
    res = r.execute_command("SM.TEMPLATE")
    val = json.loads(res)
    assert val["initial"] == ""
    assert val["map"] == {}
    assert val["current"] == ""
    assert val["reason"] == ""


def test_mutate(r):
    r.flushdb()
    sm = genmap()
    assert r.execute_command("SM.SET", "bar", json.dumps(sm))

    # bad state
    with pytest.raises(ResponseError):
        assert r.execute_command("SM.MUTATE", "bar", "smurfy")

    # good state
    assert r.execute_command("SM.MUTATE", "bar", "too")
    assert r.execute_command("SM.MUTATE", "bar", "b", "with a reason!")
    assert r.execute_command("SM.GET", "bar", "reason") == "with a reason!"

    r.flushdb()
    assert r.execute_command("SM.SET", "bar", json.dumps(sm))

    # bad forced state
    with pytest.raises(ResponseError):
        assert r.execute_command("SM.MUTATE", "bar", "smurfy", "FORCE")
        assert r.execute_command("SM.MUTATE", "bar", "smurfy", "force")

    # now the force
    assert r.execute_command("SM.MUTATE", "bar", "b", "this reason!", "force")
    assert r.execute_command("SM.GET", "bar", "reason") == "this reason!"
