
#[deriving(Eq)]
pub enum Particle {
	Up, Down,
	Charm, Strange,
	Top, Bottom,
	Electron, ElectronNeutrino,
	Muon, MuonNeutrino,
	Tau, TauNeutrino,
	Gluon, Photon, ZBoson, WPlusBoson, WMinusBoson,
	HiggsBoson,
}

#[deriving(Eq)]
pub enum Family {
	Quark,
	Lepton,
	Boson,
}

impl Particle {
	pub fn charge(self) -> (int, int) {
		match self {
			Up | Charm | Top => (2, 3),
			Down | Strange | Bottom => (-1, 3),
			Electron | Muon | Tau => (-1, 1),
			ElectronNeutrino | MuonNeutrino | TauNeutrino => (1, 2),
			Gluon | Photon | ZBoson => (0, 1),
			WPlusBoson => (1, 1),
			WMinusBoson => (-1, 1),
			HiggsBoson => (0, 1),
		}
	}

	pub fn spin(self) -> (int, int) {
		match self {
			Up | Down | Charm | Strange | Top | Bottom
			| Electron | ElectronNeutrino
			| Muon | MuonNeutrino
			| Tau | TauNeutrino => (1, 2),
			Gluon | Photon | ZBoson | WPlusBoson | WMinusBoson => (1, 1),
			HiggsBoson => (0, 1),
		}
	}

	pub fn generation(self) -> int {
		match self {
			Up | Down | Electron | ElectronNeutrino => 1,
			Charm | Strange | Muon | MuonNeutrino => 2,
			Top | Bottom | Tau | TauNeutrino => 3,
			Gluon | WPlusBoson | WMinusBoson | ZBoson | Photon | HiggsBoson => 1,
		}
	}

	pub fn got_antiparticle(self) -> bool {
		match self {
			Up | Down | Charm | Strange | Top | Bottom
			| Electron | ElectronNeutrino
			| Muon | MuonNeutrino
			| Tau | TauNeutrino
			| WPlusBoson | WMinusBoson => true,
			Gluon | ZBoson | Photon | HiggsBoson => false,
		}
	}

	pub fn family(self) -> Family {
		match self {
			Up | Down | Charm | Strange | Top | Bottom => Quark,
			Electron | ElectronNeutrino
			| Muon | MuonNeutrino
			| Tau | TauNeutrino => Lepton,
			Gluon | Photon | ZBoson
			| WPlusBoson | WMinusBoson | HiggsBoson => Boson,
		}
	}

	pub fn interacts_with(self, p: Particle) -> bool {
		match self {
			Up | Down | Charm | Strange
			| Top | Bottom => match p {
				Up | Down | Charm | Strange
				| Top | Bottom | Electron | ElectronNeutrino
				| Muon | MuonNeutrino
				| Tau | TauNeutrino => false,
				Gluon | Photon | ZBoson
				| WPlusBoson | WMinusBoson
				| HiggsBoson => true,
			},
			Electron | Muon | Tau => match p {
				Up | Down | Charm | Strange | Top | Bottom 
				| Electron | Muon | Tau | Gluon => false,
				ElectronNeutrino | MuonNeutrino | TauNeutrino
				| Photon | ZBoson
				| WPlusBoson | WMinusBoson | HiggsBoson => true,
			},
			ElectronNeutrino | MuonNeutrino | TauNeutrino => match p {
				Up | Down | Charm | Strange | Top | Bottom
				| ElectronNeutrino | MuonNeutrino | TauNeutrino
				| Gluon | Photon | HiggsBoson => false,
				Electron | Muon | Tau
				| ZBoson | WPlusBoson | WMinusBoson => true,
			},
			Gluon => match p {
				Up | Down | Charm | Strange | Top | Bottom
				| Gluon => true,
				Electron | ElectronNeutrino
				| Muon | MuonNeutrino
				| Tau | TauNeutrino
				| Photon | ZBoson
				| WPlusBoson | WMinusBoson | HiggsBoson => false,
			},
			Photon => match p {
				Up | Down | Charm | Strange | Top | Bottom
				| Electron | Muon | Tau
				| WPlusBoson | WMinusBoson => true,
				ElectronNeutrino | MuonNeutrino | TauNeutrino
				| Gluon | ZBoson | Photon | HiggsBoson => false,
			},
			ZBoson => match p {
				Up | Down | Charm | Strange | Top | Bottom
				| Electron | ElectronNeutrino 
				| Muon | MuonNeutrino
				| Tau | TauNeutrino
				| WPlusBoson | WMinusBoson | HiggsBoson => true,
				Gluon | Photon | ZBoson => false,
			},
			WPlusBoson | WMinusBoson => match p {
				Up | Down | Charm | Strange | Top | Bottom
				| Electron | ElectronNeutrino
				| Muon | MuonNeutrino
				| Tau | TauNeutrino
				| Photon | ZBoson
				| WPlusBoson | WMinusBoson | HiggsBoson => true,
				Gluon => false,
			},
			HiggsBoson => match p {
				Up | Down | Charm | Strange | Top | Bottom
				| Electron | Muon | Tau
				| ZBoson | WPlusBoson | WMinusBoson
				| HiggsBoson => true,
				ElectronNeutrino | MuonNeutrino | TauNeutrino
				| Gluon | Photon => false,
			},
		}
	}
}


