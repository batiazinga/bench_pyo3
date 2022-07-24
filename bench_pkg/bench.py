from .func import do_something


def test_do_something(benchmark):
    result = benchmark(do_something, duration=0.001)
    assert result
