use crate::source::SourceComposed;
use pyo3::prelude::*;
use songbird::input::Input;
use songbird::tracks::{LoopState, Track};
use uuid::Uuid;

#[pyclass(frozen)]
pub struct IntoTrack {
    source: Py<PyAny>,
    #[pyo3(get)]
    volume: f32,
    #[pyo3(get)]
    is_loop: bool,
    #[pyo3(get)]
    loop_count: Option<usize>,
    #[pyo3(get)]
    uuid: Option<Uuid>,
}

#[pymethods]
impl IntoTrack {
    #[new]
    #[pyo3(signature = (source, volume, is_loop, loop_count=None, uuid=None))]
    fn new(
        source: Py<PyAny>,
        volume: f32,
        is_loop: bool,
        loop_count: Option<usize>,
        uuid: Option<Uuid>,
    ) -> Self {
        Self {
            source,
            volume,
            is_loop,
            loop_count,
            uuid,
        }
    }
}

impl IntoTrack {
    pub fn build(&self) -> PyResult<Track> {
        let input = Python::with_gil(|py| {
            let inner = self.source.call_method0(py, "get_source")?;
            let composed = inner.downcast_bound::<SourceComposed>(py)?;
            PyResult::<Input>::Ok(composed.get().0.input())
        })?;

        let mut track = if let Some(uuid) = self.uuid {
            Track::new_with_uuid(input, uuid)
        } else {
            Track::new(input)
        };

        track = track
            .volume(self.volume)
            .loops(match (self.is_loop, &self.loop_count) {
                (false, _) => LoopState::Finite(0),
                (true, None) => LoopState::Infinite,
                (true, Some(x)) => LoopState::Finite(*x),
            });

        Ok(track)
    }
}
