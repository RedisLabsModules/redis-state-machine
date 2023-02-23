import pytest
import redis


redis_url = "redis://localhost:6379/0"

def pytest_addoption(parser):
    parser.addoption(
        "--redis-url",
        default=redis_url,
        action="store",
        help="Redis connection string, defaults to %(default)s",
    )
    
def _get_client(
    cls, request, flushdb=True, from_url=None,
):
    if from_url is None:
        redis_url = request.config.getoption("--redis-url")
        
    client = redis.from_url(redis_url, decode_responses=True)
    if flushdb:
        client.flushdb()
    return client
        

@pytest.fixture()
def r(request):
    with _get_client(redis.Redis, request) as client:
        yield client