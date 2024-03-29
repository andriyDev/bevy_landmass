pub fn bevy_vec3_to_landmass_vec3(
  bevy_vec: bevy::prelude::Vec3,
) -> landmass::Vec3 {
  landmass::Vec3::new(bevy_vec.x, bevy_vec.y, bevy_vec.z)
}

pub fn landmass_vec3_to_bevy_vec3(
  landmass_vec: landmass::Vec3,
) -> bevy::prelude::Vec3 {
  bevy::prelude::Vec3::new(landmass_vec.x, landmass_vec.y, landmass_vec.z)
}
