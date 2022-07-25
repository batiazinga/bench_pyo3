use pyo3::Py;
use pyo3::PyAny;
use pyo3::PyErr;
use pyo3::Python;

pub fn set_python_path(path: &str) -> Result<(), PyErr> {
    Python::with_gil(|py| {
        let sys_module = py.import("sys")?;
        let python_path = match sys_module.getattr("path") {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        match python_path.call_method1("append", (path,)) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    })
}

pub struct Caller {
    function: Py<PyAny>,
}

impl Caller {
    pub fn init() -> Result<Self, PyErr> {
        Python::with_gil(|py| -> Result<Self, PyErr> {
            let bench_module = py.import("bench_pkg.bench")?;
            let function = bench_module.getattr("do_something")?;
            Ok(Caller {
                function: function.into(),
            })
        })
    }

    pub fn do_something(&self, duration: f64) -> Result<bool, PyErr> {
        Python::with_gil(|py| -> Result<bool, PyErr> {
            let result = self.function.call1(py, (duration,))?;
            result.extract(py)
        })
    }

    pub fn do_something_with_gil(&self, py: Python, duration: f64) -> Result<bool, PyErr> {
        let result = self.function.call1(py, (duration,))?;
        result.extract(py)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::Python;

    #[test]
    fn it_works() {
        set_python_path(".").unwrap();
        let c = Caller::init().unwrap();
        let result = c.do_something(0.01).unwrap();

        assert!(result);
    }

    #[test]
    fn it_works_with_gil() {
        set_python_path(".").unwrap();
        let c = Caller::init().unwrap();

        let mut result = false;
        Python::with_gil(|py| {
            result = c.do_something_with_gil(py, 0.01).unwrap();
        });

        assert!(result);
    }
}
