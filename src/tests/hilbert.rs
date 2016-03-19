use super::super::hilbert::*;

/// Check turning the Direction enum clockwise and counterclockwise.
#[test]
fn test_turn() {
    let mut directions_rotating_right = vec![Left];

    // Perform a full clockwise step-by-step rotation, saving all intermediate
    // directions:
    for _ in 0..4 {
        let rotated_right = turn(*directions_rotating_right.last().unwrap(),
                                 Turn::Right);
        directions_rotating_right.push(rotated_right);
    }
    assert!(directions_rotating_right == vec![Left, Up, Right, Down, Left]);

    // Rotate counterclockwise, starting facing just where the previous
    // rotation ended:
    let mut directions_rotating_left = vec![Left];
    for _ in 0..4 {
        let rotated_left = turn(*directions_rotating_left.last().unwrap(),
                                Turn::Left);
        directions_rotating_left.push(rotated_left);
    }
    // The counterclockwise rotation should be the clockwise one in reverse.
    directions_rotating_right.reverse();
    assert!(directions_rotating_left == directions_rotating_right);
}
