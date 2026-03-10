use ruff_text_size::TextSize;

pub(crate) fn line_from_offset(source: &str, offset: TextSize) -> usize {
    source[..offset.to_usize()].lines().count().max(1)
}

pub(crate) fn offset_to_line(source: &str, offset: TextSize) -> usize {
    line_from_offset(source, offset)
}

pub(crate) fn range_text(source: &str, range: ruff_text_size::TextRange) -> String {
    let text = &source[range.start().to_usize()..range.end().to_usize()];
    text.lines().next().unwrap_or("").trim().to_string()
}
