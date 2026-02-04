use crate::MdRequest;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct MdRequestList(Vec<MdRequest>);

impl MdRequestList {
    pub(crate) fn new(requests: Vec<MdRequest>) -> Self {
        Self(requests)
    }

    /// returns the number of requests in the list
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// determines if the list is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// fetches the first request in the list, if any
    pub fn first(&self) -> Option<&MdRequest> {
        self.0.first()
    }

    /// represents the list as a slice
    pub fn slice(&self) -> &[MdRequest] {
        &self.0
    }

    /// attempts to find a request at the specified line number (starts at 1)
    pub fn at_line(&self, line: usize) -> Option<&MdRequest> {
        self.0
            .iter()
            .find(|req| req.data.position.contains_line(line))
    }

    /// iterates over the requests in the list
    pub fn iter(&self) -> ::std::slice::Iter<'_, MdRequest> {
        self.0.iter()
    }

    /// mutably iterates over the requests in the list
    pub fn iter_mut(&mut self) -> ::std::slice::IterMut<'_, MdRequest> {
        self.0.iter_mut()
    }
}

impl IntoIterator for MdRequestList {
    type Item = MdRequest;
    type IntoIter = ::std::vec::IntoIter<MdRequest>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a MdRequestList {
    type Item = &'a MdRequest;
    type IntoIter = ::std::slice::Iter<'a, MdRequest>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
