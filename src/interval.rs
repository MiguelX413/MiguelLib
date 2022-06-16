use pyo3::exceptions::PyValueError;

use crate::misc::a_else_b;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn merge_sub_intervals(sub_intervals: &mut Vec<(bool, f64, f64, bool)>) {
    sub_intervals.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut index = 0;
    for i in 1..sub_intervals.len() {
        if (sub_intervals[index].2 > sub_intervals[i].1)
            || ((sub_intervals[index].2 == sub_intervals[i].1)
                && ((sub_intervals[index].3) || (sub_intervals[i].0)))
        {
            if (sub_intervals[i].2 > sub_intervals[index].2)
                || ((sub_intervals[i].2 == sub_intervals[index].2) && (sub_intervals[i].3))
            {
                sub_intervals[index].2 = sub_intervals[i].2;
                sub_intervals[index].3 = sub_intervals[i].3;
            }
        } else {
            index += 1;
            sub_intervals[index] = sub_intervals[i];
        }
    }
    sub_intervals.truncate(index + 1);
}

/// A class used to represent intervals.
#[pyclass]
pub(crate) struct Interval {
    sub_intervals: Vec<(bool, f64, f64, bool)>,
}

#[pymethods]
impl Interval {
    #[new]
    fn py_new(sub_intervals: Option<Vec<(bool, f64, f64, bool)>>) -> PyResult<Self> {
        match sub_intervals {
            Some(mut some_sub_intervals) => {
                let mut index = 0;
                for i in 0..some_sub_intervals.len() {
                    if some_sub_intervals[i].1.is_nan() || some_sub_intervals[i].2.is_nan() {
                        return Err(PyValueError::new_err("Sub-interval points cannot be NaN"));
                    }
                    if (some_sub_intervals[i].1.is_infinite() && some_sub_intervals[i].0)
                        || (some_sub_intervals[i].2.is_infinite() && some_sub_intervals[i].3)
                    {
                        return Err(PyValueError::new_err("Interval cannot contain inf"));
                    }
                    if some_sub_intervals[i].1 > some_sub_intervals[i].2 {
                        return Err(PyValueError::new_err(
                            "Start point of sub-interval cannot be greater than its end point",
                        ));
                    }

                    if !((some_sub_intervals[i].1 == some_sub_intervals[i].2)
                        && (!some_sub_intervals[i].0 || !some_sub_intervals[i].3))
                    {
                        some_sub_intervals[index] = some_sub_intervals[i];
                        index += 1;
                    }
                }
                some_sub_intervals.truncate(index);

                merge_sub_intervals(&mut some_sub_intervals);
                Ok(Interval {
                    sub_intervals: some_sub_intervals,
                })
            }
            None => Ok(Interval {
                sub_intervals: vec![],
            }),
        }
    }
    #[args(others = "*")]
    fn union(&self, others: &PyTuple) -> PyResult<Interval> {
        let mut output = self.clone();
        output.union_update(others)?;
        Ok(output)
    }
    #[args(others = "*")]
    fn union_update(&mut self, others: &PyTuple) -> PyResult<()> {
        let inputs: Vec<Interval> = others.extract()?;
        self.sub_intervals
            .extend(inputs.iter().flat_map(|f| &f.sub_intervals));
        if !inputs.is_empty() {
            merge_sub_intervals(&mut self.sub_intervals);
        }
        Ok(())
    }
    fn __contains__(&self, item: f64) -> bool {
        self.sub_intervals
            .iter()
            .any(|&f| (f.1 < item && item < f.2) || ((item == f.1 && f.0) || (item == f.2 && f.3)))
    }
    fn __repr__(&self) -> String {
        format!(
            "Interval([{}])",
            self.sub_intervals
                .iter()
                .map(|&f| format!(
                    "({}, {}, {}, {})",
                    a_else_b(f.0, "True", "False"),
                    f.1,
                    f.2,
                    a_else_b(f.3, "True", "False")
                ))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
    fn __str__(&self) -> String {
        if !self.sub_intervals.is_empty() {
            self.sub_intervals
                .iter()
                .map(|&f| {
                    format!(
                        "{}{}, {}{}",
                        a_else_b(f.0, "[", "("),
                        f.1,
                        f.2,
                        a_else_b(f.3, "]", ")"),
                    )
                })
                .collect::<Vec<String>>()
                .join(" ∪ ")
        } else {
            "∅".to_string()
        }
    }
    fn __or__(&self, other: &Interval) -> Interval {
        let mut output = self.clone();
        output.__ior__(other);
        output
    }
    fn __ior__(&mut self, other: &Interval) {
        self.sub_intervals.append(&mut other.sub_intervals.clone());
        merge_sub_intervals(&mut self.sub_intervals);
    }
}

impl Clone for Interval {
    fn clone(&self) -> Interval {
        Interval {
            sub_intervals: self.sub_intervals.clone(),
        }
    }
}
