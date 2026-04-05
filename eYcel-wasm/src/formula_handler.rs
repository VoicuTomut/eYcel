use std::collections::HashMap;

use crate::cell_value::CellValue;

/// Maps (row, col) → formula string. Row and col are 0-based.
pub type FormulaMap = HashMap<(usize, usize), String>;

/// Extract all formula cells from a 2D grid of CellValues.
/// Returns a map of (row, col) → formula string.
pub fn extract_formulas(grid: &[Vec<CellValue>]) -> FormulaMap {
    let mut formulas = FormulaMap::new();
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if let CellValue::Formula(f) = cell {
                formulas.insert((row_idx, col_idx), f.clone());
            }
        }
    }
    formulas
}

/// Clear formula cells in the grid (set to Empty) so transforms skip them.
pub fn clear_formula_cells(grid: &mut [Vec<CellValue>], formulas: &FormulaMap) {
    for &(row, col) in formulas.keys() {
        if row < grid.len() && col < grid[row].len() {
            grid[row][col] = CellValue::Empty;
        }
    }
}

/// Reinject formula strings back into their original positions.
/// This is called AFTER all data transforms are complete.
pub fn reinject_formulas(grid: &mut [Vec<CellValue>], formulas: &FormulaMap) {
    for (&(row, col), formula) in formulas.iter() {
        if row < grid.len() && col < grid[row].len() {
            grid[row][col] = CellValue::Formula(formula.clone());
        }
    }
}

/// Verify that all formulas from the original grid are preserved in the processed grid.
pub fn verify_formulas_preserved(
    original_formulas: &FormulaMap,
    processed_grid: &[Vec<CellValue>],
) -> bool {
    let processed_formulas = extract_formulas(processed_grid);
    for (coords, formula) in original_formulas {
        match processed_formulas.get(coords) {
            Some(pf) if pf == formula => {}
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_grid() -> Vec<Vec<CellValue>> {
        vec![
            // Row 0: headers
            vec![
                CellValue::String("Name".into()),
                CellValue::String("Value".into()),
                CellValue::String("Total".into()),
            ],
            // Row 1: data + formula
            vec![
                CellValue::String("Alice".into()),
                CellValue::Int(100),
                CellValue::Formula("=B2*2".into()),
            ],
            // Row 2: data + formula
            vec![
                CellValue::String("Bob".into()),
                CellValue::Int(200),
                CellValue::Formula("=B3*2".into()),
            ],
        ]
    }

    #[test]
    fn test_extract_formulas() {
        let grid = make_grid();
        let formulas = extract_formulas(&grid);
        assert_eq!(formulas.len(), 2);
        assert_eq!(formulas.get(&(1, 2)).unwrap(), "=B2*2");
        assert_eq!(formulas.get(&(2, 2)).unwrap(), "=B3*2");
    }

    #[test]
    fn test_clear_and_reinject() {
        let mut grid = make_grid();
        let formulas = extract_formulas(&grid);

        clear_formula_cells(&mut grid, &formulas);
        assert!(grid[1][2].is_empty());
        assert!(grid[2][2].is_empty());

        reinject_formulas(&mut grid, &formulas);
        assert_eq!(grid[1][2], CellValue::Formula("=B2*2".into()));
        assert_eq!(grid[2][2], CellValue::Formula("=B3*2".into()));
    }

    #[test]
    fn test_verify_preserved() {
        let grid = make_grid();
        let formulas = extract_formulas(&grid);
        assert!(verify_formulas_preserved(&formulas, &grid));

        // Tamper with a formula
        let mut bad_grid = make_grid();
        bad_grid[1][2] = CellValue::Formula("=B2*3".into());
        assert!(!verify_formulas_preserved(&formulas, &bad_grid));
    }
}
