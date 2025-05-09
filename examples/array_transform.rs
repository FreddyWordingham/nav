use nav::Transform;
use ndarray::arr2;

fn main() {
    // Create a 2D array
    let arr = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

    // Print the transformed array
    println!("{}\n", &arr);
    println!("Identity\n{}\n", Transform::Identity * &arr);
    println!("Rotate90\n{}\n", Transform::Rotate90 * &arr);
    println!("Rotate180\n{}\n", Transform::Rotate180 * &arr);
    println!("Rotate270\n{}\n", Transform::Rotate270 * &arr);
    println!("FlipHorizontal\n{}\n", Transform::FlipHorizontal * &arr);
    println!("FlipDiagonal\n{}\n", Transform::FlipDiagonal * &arr);
    println!("FlipVertical\n{}\n", Transform::FlipVertical * &arr);
    println!("FlipAntiDiagonal\n{}\n", Transform::FlipAntiDiagonal * &arr);
}
