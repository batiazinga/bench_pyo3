from .do import do_something


def test_do_something_00001(benchmark):
    result = benchmark(do_something, duration=0.00001)
    assert result


def test_do_something_0001(benchmark):
    result = benchmark(do_something, duration=0.0001)
    assert result


def test_do_something_001(benchmark):
    result = benchmark(do_something, duration=0.001)
    assert result


def test_do_something_01(benchmark):
    result = benchmark(do_something, duration=0.01)
    assert result
