// use ray::vec3;
use hell::vec3::Vec3;

#[test]
fn rust_test() {
    let a = Vec3{x: 1.0, y: 1.0, z: 1.0};
    assert_eq!(a.x, 1.0);
    assert_eq!(a.squared_length() , 3.0);
}


#[test]
fn test_add() {
    let a = Vec3{x: 1.0, y: 1.0, z: 1.0};
    let b = Vec3{x: 2.0, y: 2.0, z: 2.0};
    let c = a + b;
    assert_eq!(c.x , 3.0);
}

#[test]
fn test_sub() {
    let a = Vec3{x: 1.0, y: 1.0, z: 1.0};
    let b = Vec3{x: 2.0, y: 2.0, z: 2.0};
    let c = a - b;
    assert_eq!(c.x , -1.0);
}

#[test]
fn test_mul() {
    let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    let b =  a * 3.0;
    assert_eq!(b.x , 3.0);
    assert_eq!(b.y , 6.0);
    assert_eq!(b.z , 9.0);
}

#[test]
fn test_div() {
    let a = Vec3{x: 3.0, y: 6.0, z: 9.0};
    let b =  a / 3.0;
    assert_eq!(b.x , 1.0);
    assert_eq!(b.y , 2.0);
    assert_eq!(b.z , 3.0);
}