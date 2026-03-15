use ruff_text_size::TextSize;

/// Pre-computed index of line-start byte offsets for O(log n) offset-to-line lookups.
pub(crate) struct LineIndex {
    /// Byte offsets where each line starts. `line_starts[0]` is always 0.
    line_starts: Vec<usize>,
}

impl LineIndex {
    pub(crate) fn build(source: &str) -> Self {
        let mut line_starts = vec![0usize];
        for (i, byte) in source.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push(i + 1);
            }
        }
        LineIndex { line_starts }
    }

    /// Return the 1-based line number for a byte offset.
    pub(crate) fn line_from_offset(&self, offset: TextSize) -> usize {
        let offset = offset.to_usize();
        match self.line_starts.binary_search(&offset) {
            Ok(line) => line + 1,
            Err(line) => line, // insertion point = number of line_starts before offset
        }
        .max(1)
    }
}

/// Standalone version for callers that don't have a LineIndex.
#[cfg(test)]
pub(crate) fn line_from_offset(source: &str, offset: TextSize) -> usize {
    LineIndex::build(source).line_from_offset(offset)
}

pub(crate) fn range_text(source: &str, range: ruff_text_size::TextRange) -> String {
    let text = &source[range.start().to_usize()..range.end().to_usize()];
    text.lines().next().unwrap_or("").trim().to_string()
}
