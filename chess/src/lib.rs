
pub mod chess;


#[cfg(test)]
mod tests {
    #[test]
    fn test_chess_piece(){

    }
    fn create_default_chess_piece() -> ChessPiece{
        let chess_piece: ChessPiece = ChessPiece::new(PieceType::);
    }
}
