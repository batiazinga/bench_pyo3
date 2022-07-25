# Benchmark PyO3
Estimate the cost of calling a Python function from Rust with PyO3.

## How to run the Python benchmark
It is recommanded to use a virtual environment.

Once the virtual environment is activated, install the dependencies with `pip`
```
pip3 install -r requirements.txt
```

Then, run the benchmark
```
pytest ./bench_py/bench.py
```

Example of output
```
------------------------------------------------------------------------------------------------ benchmark: 4 tests -----------------------------------------------------------------------------------------------
Name (time in us)                   Min                    Max                   Mean              StdDev                 Median                 IQR            Outliers          OPS            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_do_something_00001         22.9510 (1.0)         179.2040 (1.0)          69.6026 (1.0)        3.5769 (1.0)          69.6710 (1.0)        0.1050 (1.0)      214;2000  14,367.2745 (1.0)       12836           1
test_do_something_0001         117.3420 (5.11)        484.6690 (2.70)        158.9896 (2.28)       7.8311 (2.19)        159.3560 (2.29)       0.5300 (5.05)      182;279   6,289.7190 (0.44)       6171           1
test_do_something_001        1,056.4230 (46.03)     1,266.5450 (7.07)      1,130.4608 (16.24)     49.6016 (13.87)     1,158.1445 (16.62)    103.0290 (981.23)      404;0     884.5950 (0.06)        938           1
test_do_something_01        10,095.9550 (439.89)   10,598.2340 (59.14)    10,309.1271 (148.11)   183.5802 (51.32)    10,189.0725 (146.25)   387.9525 (>1000.0)      34;0      97.0014 (0.01)         96           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```

# How to run the Rust benchmark
Make sure Rust and Cargo are installed.

Then, run
```
cargo bench -- --verbose
```
