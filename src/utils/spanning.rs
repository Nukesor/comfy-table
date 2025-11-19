use std::collections::HashMap;

/// Information about an active rowspan.
#[derive(Debug, Clone)]
struct RowSpanInfo {
    /// Starting row index of the span (also stored in HashMap key for lookup)
    start_row: usize,
    /// Number of rows remaining (decremented as we process rows)
    remaining_rows: u16,
    /// Number of columns this span covers
    colspan: u16,
    /// Cached formatted content lines for this rowspan cell (None for border drawing)
    formatted_content: Option<Vec<String>>,
}

/// Tracks active row spans across rows during table rendering.
#[derive(Debug, Clone, Default)]
pub(crate) struct SpanTracker {
    /// Maps (start_row, start_col) -> RowSpanInfo
    active_spans: HashMap<(usize, usize), RowSpanInfo>,
}

impl SpanTracker {
    /// Create a new empty SpanTracker.
    pub(crate) fn new() -> Self {
        Self {
            active_spans: HashMap::new(),
        }
    }

    /// Check if a position is occupied by a rowspan from a previous row.
    ///
    /// Returns `Some((rowspan_remaining, colspan))` if the position is occupied,
    /// `None` otherwise.
    pub(crate) fn is_occupied(&self, row_index: usize, col_index: usize) -> Option<(u16, u16)> {
        for ((start_row, start_col), info) in &self.active_spans {
            if *start_row < row_index {
                // Check if this position falls within the colspan range
                if *start_col <= col_index && col_index < *start_col + info.colspan as usize {
                    return Some((info.remaining_rows, info.colspan));
                }
            }
        }
        None
    }

    /// Register a new rowspan cell with its formatted content.
    ///
    /// This should be called when processing a cell that has rowspan > 1.
    /// remaining_rows is set to rowspan - 1, meaning it will appear in rowspan - 1 more rows.
    pub(crate) fn register_rowspan(
        &mut self,
        row_index: usize,
        col_index: usize,
        rowspan: u16,
        colspan: u16,
        formatted_content: Option<Vec<String>>,
    ) {
        if rowspan > 1 {
            self.active_spans.insert(
                (row_index, col_index),
                RowSpanInfo {
                    start_row: row_index,
                    remaining_rows: rowspan - 1, // Will appear in rowspan - 1 more rows
                    colspan,
                    formatted_content,
                },
            );
        }
    }

    /// Get the cached formatted content for a rowspan cell.
    ///
    /// Returns the formatted content lines if the position is occupied by a rowspan.
    pub(crate) fn get_rowspan_content(
        &self,
        row_index: usize,
        col_index: usize,
    ) -> Option<&Vec<String>> {
        for ((start_row, start_col), info) in &self.active_spans {
            if *start_row < row_index {
                // Check if this position falls within the colspan range
                if *start_col <= col_index && col_index < *start_col + info.colspan as usize {
                    return info.formatted_content.as_ref();
                }
            }
        }
        None
    }

    /// Decrement rowspan counters and remove expired spans.
    ///
    /// This should be called after processing each row.
    /// A rowspan is removed only after it has been displayed in all its spanned rows.
    /// When remaining_rows reaches 0, it means the span was just displayed in its last row,
    /// so we remove it after that row is processed.
    pub(crate) fn advance_row(&mut self, current_row: usize) {
        // First, remove spans that have expired (remaining_rows == 0 means it was just displayed in its last row)
        self.active_spans.retain(|_, info| info.remaining_rows > 0);

        // Then decrement remaining_rows for all active spans that have been displayed
        // We decrement after the row has been processed, so remaining_rows represents
        // how many more rows the span should appear in
        for info in self.active_spans.values_mut() {
            if info.start_row < current_row && info.remaining_rows > 0 {
                info.remaining_rows -= 1;
            }
        }
    }

    /// Check if a column position is part of any active rowspan.
    pub(crate) fn is_col_occupied_by_rowspan(&self, row_index: usize, col_index: usize) -> bool {
        self.is_occupied(row_index, col_index).is_some()
    }

    /// Get the starting position of a rowspan that occupies the given position.
    ///
    /// Returns `Some((start_row, start_col, colspan))` if the position is occupied,
    /// `None` otherwise.
    pub(crate) fn get_rowspan_start(
        &self,
        row_index: usize,
        col_index: usize,
    ) -> Option<(usize, usize, u16)> {
        for ((start_row, start_col), info) in &self.active_spans {
            if *start_row < row_index {
                // Check if this position falls within the colspan range
                if *start_col <= col_index && col_index < *start_col + info.colspan as usize {
                    return Some((*start_row, *start_col, info.colspan));
                }
            }
        }
        None
    }

    /// Get the starting position of a rowspan that occupies the given position at the given row.
    /// This includes rowspans that started at the current row (for border drawing).
    ///
    /// Returns `Some((start_row, start_col, colspan))` if the position is occupied,
    /// `None` otherwise.
    pub(crate) fn get_rowspan_start_at_row(
        &self,
        row_index: usize,
        col_index: usize,
    ) -> Option<(usize, usize, u16)> {
        for ((start_row, start_col), info) in &self.active_spans {
            // Check if rowspan is active at this row (started at or before this row, and still has remaining rows)
            if *start_row <= row_index && info.remaining_rows > 0 {
                // Check if this position falls within the colspan range
                if *start_col <= col_index && col_index < *start_col + info.colspan as usize {
                    return Some((*start_row, *start_col, info.colspan));
                }
            }
        }
        None
    }
}
