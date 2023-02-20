from redis.exceptions import ResponseError
import os
import json


def test_check_registered_commands(r):
    cm = r.info("modules")
    found = False
    for m in cm.get("modules"):
        if m.get("name") == "redisstate":
            found = True
            break
        
    assert found
    
def test_commands_registered(r):
    here = os.path.dirname(__file__)
    cmdsjson = os.path.abspath(os.path.join(here, "..", "commands.json"))
    cmds = json.load(open(cmdsjson))
    for c in cmds.keys():
        try:
            r.execute_command(c)
        except ResponseError as e:
            assert str(e).find("wrong number of arguments") != -1