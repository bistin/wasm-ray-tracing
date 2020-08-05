// use ray::vec3;
use hell::vec3::Vec3;

#[test]
fn rust_test() {
    let a = Vec3{x: 1.0, y: 1.0, z: 1.0};
    assert_eq!(a.x, 1.0);
}
