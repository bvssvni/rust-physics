
pub mod particle;
pub mod element;
pub mod unit;
pub mod constant;

#[cfg(test)]
mod tests {
	#[test]
	fn test_particle_charge() {
		let particle = ::particle::Up;
		let charge = particle.charge();
		assert_eq!(charge, (2, 3));
	}

	#[test]
	fn test_particle_spin() {
		let particle = ::particle::Up;
		let spin = particle.spin();
		assert_eq!(spin, (1, 2));
	}

	#[test]
	fn test_particle_generation() {
		let particle = ::particle::Up;
		assert_eq!(particle.generation(), 1);
	}

	#[test]
	fn test_particle_family() {
		let particle = ::particle::Up;
		assert_eq!(particle.family(), ::particle::Quark);
	}

	#[test]
	fn test_particle_interact_with() {
		let particle = ::particle::Up;
		assert!(particle.interacts_with(::particle::Photon));
	}
}


