use super::*;

#[test]
fn test_binary_search() {
    let b: [i32; 0] = [];
    assert_eq!(custom_binary_search_1(&b, &5), Err(0));

    let b = [4];
    assert_eq!(custom_binary_search_1(&b, &3), Err(0));
    assert_eq!(custom_binary_search_1(&b, &4), Ok(0));
    assert_eq!(custom_binary_search_1(&b, &5), Err(1));

    let b = [1, 2, 4, 6, 8, 9];
    assert_eq!(custom_binary_search_1(&b, &5), Err(3));
    assert_eq!(custom_binary_search_1(&b, &6), Ok(3));
    assert_eq!(custom_binary_search_1(&b, &7), Err(4));
    assert_eq!(custom_binary_search_1(&b, &8), Ok(4));

    let b = [1, 2, 4, 5, 6, 8];
    assert_eq!(custom_binary_search_1(&b, &9), Err(6));

    let b = [1, 2, 4, 6, 7, 8, 9];
    assert_eq!(custom_binary_search_1(&b, &6), Ok(3));
    assert_eq!(custom_binary_search_1(&b, &5), Err(3));
    assert_eq!(custom_binary_search_1(&b, &8), Ok(5));

    let b = [1, 2, 4, 5, 6, 8, 9];
    assert_eq!(custom_binary_search_1(&b, &7), Err(5));
    assert_eq!(custom_binary_search_1(&b, &0), Err(0));

    let b = [1, 3, 3, 3, 7];
    assert_eq!(custom_binary_search_1(&b, &0), Err(0));
    assert_eq!(custom_binary_search_1(&b, &1), Ok(0));
    assert_eq!(custom_binary_search_1(&b, &2), Err(1));
    assert!(match custom_binary_search_1(&b, &3) {
        Ok(1..=3) => true,
        _ => false,
    });
    assert!(match custom_binary_search_1(&b, &3) {
        Ok(1..=3) => true,
        _ => false,
    });
    assert_eq!(custom_binary_search_1(&b, &4), Err(4));
    assert_eq!(custom_binary_search_1(&b, &5), Err(4));
    assert_eq!(custom_binary_search_1(&b, &6), Err(4));
    assert_eq!(custom_binary_search_1(&b, &7), Ok(4));
    assert_eq!(custom_binary_search_1(&b, &8), Err(5));
}

#[test]
// Test implementation specific behavior when finding equivalent elements.
// It is ok to break this test but when you do a crater run is highly advisable.
fn test_binary_search_implementation_details() {
    let b = [1, 1, 2, 2, 3, 3, 3];
    assert_eq!(custom_binary_search_1(&b, &1), Ok(1));
    assert_eq!(custom_binary_search_1(&b, &2), Ok(3));
    assert_eq!(custom_binary_search_1(&b, &3), Ok(5));
    let b = [1, 1, 1, 1, 1, 3, 3, 3, 3];
    assert_eq!(custom_binary_search_1(&b, &1), Ok(4));
    assert_eq!(custom_binary_search_1(&b, &3), Ok(7));
    let b = [1, 1, 1, 1, 3, 3, 3, 3, 3];
    assert_eq!(custom_binary_search_1(&b, &1), Ok(2));
    assert_eq!(custom_binary_search_1(&b, &3), Ok(4));
}
