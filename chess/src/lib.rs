pub mod position;


#[cfg(test)]
mod tests {
    use super::chess::*;
    #[test]
    fn test_knight_piece_movement(){
        let mut chess_piece_x: ChessPiece = create_default_chess_piece(PieceType::Knight);
        chess_piece_x.set_position(Point::new(5, 0));

        let move_point_1: Point = Point::new(4, 2);
        assert_eq!(chess_piece_x.move_piece(&move_point_1), Some(&move_point_1));

        let move_point_2: Point = Point::new(3, 2);
        assert_eq!(chess_piece_x.move_piece(&move_point_2), None);
    }
    fn create_default_chess_piece(piece_type: PieceType) -> ChessPiece{
        let chess_piece: ChessPiece = ChessPiece::new(piece_type);
        chess_piece
    }
}
