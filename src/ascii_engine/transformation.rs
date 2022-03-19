use super::types::{Vector3};

// I worked out the rotation matrices here for algebra
fn rotatex(input: &Vector3, angler: f32) -> Vector3 {
    Vector3::new(
        input.x,
        angler.cos() * input.y + -angler.sin() * input.z,
        angler.sin() * input.y + angler.cos() * input.z
    )
}
fn rotatey(input: &Vector3, angler: f32) -> Vector3 {
    Vector3::new(
        angler.cos() * input.x + angler.sin() * input.z,
        input.y,
        -angler.sin() * input.x + angler.cos() * input.z
    )
}
fn rotatez(input: &Vector3, angler: f32) -> Vector3 {
    Vector3::new(
        angler.cos() * input.x + -angler.sin() * input.y,
        angler.sin() * input.x + angler.cos() * input.y,
        input.z
    )
}
fn translate(input: &Vector3, translation: &Vector3) -> Vector3 {
    Vector3::new(
        input.x + translation.x,
        input.y + translation.y,
        input.z + translation.z,
    )
}
pub fn compute_transforms_and_projection_point(
    input: &Vector3, rotation: &Vector3,
    translation: &Vector3, camera_orientation: &Vector3,
    camera_position: &Vector3
) -> Vector3 {
    let mut position: Vector3 = input.clone(); // model cord

    //global moves
    // apply rotation
    position = rotatez(&position, rotation.z);
    position = rotatey(&position, rotation.y);
    position = rotatex(&position, rotation.x);
    // move to world cordinates
    position = translate(&position, translation);
    // camera
    // move by camera position
    position = translate(&position, &camera_position.opposite());
    // move by the camra rotation
    let orientation = camera_orientation.opposite();
    position = rotatez(&position, -orientation.z);
    position = rotatey(&position, -orientation.y);
    position = rotatex(&position, -orientation.x);

    // project
    projection::ortho(&position)
}
pub mod projection {
    pub fn ortho(vector: &super::Vector3) -> super::Vector3 {
        super::Vector3::new(
            vector.x,
            vector.y,
            0.0 
        )
    }
}
