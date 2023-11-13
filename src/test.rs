use super::*;

#[test]
fn correct_index_square() {
    let grid = Grid::new(5, 5);
    assert_eq!(1, grid.get_index_from_pos(&Position::new(1, 0)));
    assert_eq!(5, grid.get_index_from_pos(&Position::new(0, 1)));
    assert_eq!(24, grid.get_index_from_pos(&Position::new(4, 4)));
}
#[test]
fn correct_index_rectangle() {
    let grid = Grid::new(3, 4);
    assert_eq!(1, grid.get_index_from_pos(&Position::new(1, 0)));
    assert_eq!(5, grid.get_index_from_pos(&Position::new(1, 1)));
    assert_eq!(9, grid.get_index_from_pos(&Position::new(1, 2)));
}

#[test]
fn valid_positions() {
    let grid = Grid::new(5, 5);
    assert_eq!(true, grid.is_valid_pos(&Position::new(1, 0)));
    assert_eq!(true, grid.is_valid_pos(&Position::new(4, 4)));
    assert_eq!(false, grid.is_valid_pos(&Position::new(-1, 0)));
    assert_eq!(false, grid.is_valid_pos(&Position::new(5, 4)));
    assert_eq!(false, grid.is_valid_pos(&Position::new(4, 5)));
    assert_eq!(false, grid.is_valid_pos(&Position::new(5, 5)));
}

#[test]
fn correct_pos_from_index_square() {
    let grid = Grid::new(5, 5);
    assert_eq!(Position::new(1, 0), grid.get_pos_from_index(1));
    assert_eq!(Position::new(0, 1), grid.get_pos_from_index(5));
    assert_eq!(Position::new(4, 4), grid.get_pos_from_index(24));
}

#[test]
fn correct_pos_from_index_rectangle() {
    let grid = Grid::new(3, 4);
    assert_eq!(Position::new(1, 0), grid.get_pos_from_index(1));
    assert_eq!(Position::new(1, 1), grid.get_pos_from_index(5));
    assert_eq!(Position::new(1, 2), grid.get_pos_from_index(9));

    let grid = Grid::new(5, 10);
    assert_eq!(Position::new(2, 2), grid.get_pos_from_index(22));
    assert_eq!(Position::new(5, 3), grid.get_pos_from_index(35));
    assert_eq!(Position::new(6, 4), grid.get_pos_from_index(46));
}

#[test]
fn correct_neighbours() {
    let grid = Grid::new(5, 5);
    assert_eq!(
        vec![Position::new(1, 0), Position::new(0, 1)],
        grid.get_neighbours(&Position::new(0, 0))
    );
    assert_eq!(
        vec![
            Position::new(2, 1),
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(1, 2),
        ],
        grid.get_neighbours(&Position::new(1, 1))
    );
}

#[test]
fn correct_neighbours_rect() {
    let grid = Grid::new(5, 10);
    assert_eq!(
        vec![Position::new(1, 0), Position::new(0, 1)],
        grid.get_neighbours(&Position::new(0, 0))
    );
    assert_eq!(
        vec![
            Position::new(2, 1),
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(1, 2),
        ],
        grid.get_neighbours(&Position::new(1, 1))
    );
}
