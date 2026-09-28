//! Maps locations to approximate coordinates and calculates approximate distances.

use opltypes::states::*;

/// The pair (Latitude, longitude) in degrees.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LatLong(f64, f64);

impl LatLong {
    /// The average radius of the Earth in kilometers.
    const AVERAGE_EARTH_RADIUS_KM: f64 = 6371.009;

    /// Returns the LatLong in radians.
    pub fn to_radians(self) -> LatLong {
        LatLong(self.0.to_radians(), self.1.to_radians())
    }

    /// Approximates the central angle between two points on a sphere using the [Haversine formula].
    ///
    /// The returned angle is measured in radians.
    ///
    /// [Haversine formula]: https://en.wikipedia.org/wiki/Haversine_formula
    pub fn angle_to(self, b: LatLong) -> f64 {
        // This implementation follows the example at https://www.movable-type.co.uk/scripts/latlong.html.
        let (ar, br) = (self.to_radians(), b.to_radians());
        let delta_lat = (br.0 - ar.0).abs();
        let delta_long = (br.1 - ar.1).abs();

        let a = (delta_lat / 2.0).sin() * (delta_lat / 2.0).sin()
            + ar.0.cos() * br.0.cos() * (delta_long / 2.0).sin() * (delta_long / 2.0).sin();
        2.0 * a.sqrt().atan2((1.0 - a).sqrt())
    }

    /// Approximates the spherical distance in kilometers between two Earth coordinates.
    pub fn km_to(self, b: LatLong) -> f64 {
        self.angle_to(b) * Self::AVERAGE_EARTH_RADIUS_KM
    }
}

pub trait Coordinates {
    /// Returns an approximate latitude and longitude of the central point of the region.
    fn latlong(self) -> LatLong;
}

impl Coordinates for State {
    fn latlong(self) -> LatLong {
        match self {
            // State::InArgentina(s) => s.latlong(),
            // State::InAustralia(s) => s.latlong(),
            // State::InBrazil(s) => s.latlong(),
            // State::InCanada(s) => s.latlong(),
            // State::InChile(s) => s.latlong(),
            // State::InChina(s) => s.latlong(),
            // State::InEngland(s) => s.latlong(),
            // State::InGermany(s) => s.latlong(),
            // State::InGreece(s) => s.latlong(),
            // State::InIndia(s) => s.latlong(),
            // State::InMexico(s) => s.latlong(),
            // State::InNetherlands(s) => s.latlong(),
            // State::InNewZealand(s) => s.latlong(),
            // State::InRomania(s) => s.latlong(),
            State::InRussia(s) => s.latlong(),
            State::InSouthAfrica(s) => s.latlong(),
            State::InUAE(s) => s.latlong(),
            State::InUSA(s) => s.latlong(),
            _ => LatLong(0.0, 0.0),
        }
    }
}

impl Coordinates for RussiaState {
    fn latlong(self) -> LatLong {
        match self {
            RussiaState::AD => LatLong(44.64920563569205, 40.07955076320361),
            RussiaState::AL => LatLong(50.68909630970498, 87.05709548028801),
            RussiaState::BA => LatLong(54.654654256125966, 55.980345740196825),
            RussiaState::BU => LatLong(52.896542981329546, 109.30986153952765),
            RussiaState::CE => LatLong(43.31512575296982, 45.73418625881086),
            RussiaState::CU => LatLong(55.701737667866475, 47.06699461203569),
            RussiaState::DA => LatLong(43.275156426771545, 47.021006301773355),
            RussiaState::IN => LatLong(43.28166680850278, 44.888499534488886),
            RussiaState::KB => LatLong(43.506106001149774, 43.512639813733934),
            RussiaState::KK => LatLong(53.66447062140088, 90.33386342699471),
            RussiaState::KL => LatLong(46.318530815629494, 45.1330212129971),
            RussiaState::KC => LatLong(43.85495834364858, 41.55173245792465),
            RussiaState::KR => LatLong(63.72438283695492, 33.49535495412461),
            RussiaState::KO => LatLong(64.33532834327912, 54.68948782447444),
            RussiaState::ME => LatLong(56.626117352939616, 47.917923603978664),
            RussiaState::MO => LatLong(54.25267791842998, 44.4245040678672),
            RussiaState::SA => LatLong(62.51639370840384, 129.82496406590337),
            RussiaState::SE => LatLong(43.28726099878257, 44.04377061925574),
            RussiaState::TA => LatLong(55.73323219228191, 49.54049603507655),
            RussiaState::TY => LatLong(51.57751318871598, 93.95618115566128),
            RussiaState::UD => LatLong(57.23881497550293, 52.843545486825356),
            RussiaState::ALT => LatLong(52.63714793447719, 82.65983259114218),
            RussiaState::KAM => LatLong(59.37472336870791, 162.36023483812752),
            RussiaState::KHA => LatLong(55.51660130569833, 135.60838888292534),
            RussiaState::KDA => LatLong(45.202833988519416, 39.023576038644144),
            RussiaState::KYA => LatLong(62.23339978081602, 94.3091084731444),
            RussiaState::PER => LatLong(58.21384361358635, 56.24238837617634),
            RussiaState::PRI => LatLong(44.85333443415629, 134.85252882235483),
            RussiaState::STA => LatLong(45.041216114640854, 42.13711965902017),
            RussiaState::ZAB => LatLong(52.02613149025336, 114.20490832779774),
            RussiaState::AMU => LatLong(53.768729862852375, 127.4801688038935),
            RussiaState::ARK => LatLong(64.54218154594851, 40.56802962166393),
            RussiaState::AST => LatLong(46.37706453524472, 48.01167876343204),
            RussiaState::BEL => LatLong(50.59986763575048, 36.599621631193315),
            RussiaState::BRY => LatLong(53.18903841713979, 34.15736037270969),
            RussiaState::CHE => LatLong(55.12301585023967, 61.26151111521682),
            RussiaState::CHI => LatLong(52.051452637786, 113.45255813180695),
            RussiaState::IRK => LatLong(52.463751367929845, 104.3435938422369),
            RussiaState::IVA => LatLong(56.99895237719656, 41.04309911629477),
            RussiaState::KGD => LatLong(54.68820601539769, 20.498261554350023),
            RussiaState::KLU => LatLong(54.514746552505386, 36.20535188336597),
            RussiaState::KEM => LatLong(55.18183848812608, 86.61631695862748),
            RussiaState::KIR => LatLong(58.613747272352064, 49.81050481614963),
            RussiaState::KOS => LatLong(58.50277949206878, 43.861945064253554),
            RussiaState::KGN => LatLong(55.43704248324149, 65.20071630343826),
            RussiaState::KRS => LatLong(51.7341508861689, 36.166595226043796),
            RussiaState::LEN => LatLong(59.95159746420251, 32.292883328271515),
            RussiaState::LIP => LatLong(52.61366051678569, 39.52890482091483),
            RussiaState::MAG => LatLong(62.57635672900901, 153.88625598470693),
            RussiaState::MOS => LatLong(55.74605184243877, 37.59848276871538),
            RussiaState::MUR => LatLong(67.94410038853745, 34.586344304793),
            RussiaState::NIZ => LatLong(56.33997888554929, 44.28867687814432),
            RussiaState::NGR => LatLong(58.4106044577676, 32.28549215801839),
            RussiaState::NVS => LatLong(54.96456629877333, 82.71523880025863),
            RussiaState::OMS => LatLong(55.02051696377912, 73.37302069884375),
            RussiaState::ORE => LatLong(51.71711453556147, 55.6093157941067),
            RussiaState::ORL => LatLong(52.95944923754448, 36.09406055928068),
            RussiaState::PNZ => LatLong(53.21916065697386, 44.94862971039492),
            RussiaState::PSK => LatLong(57.4252668077144, 29.037036094486503),
            RussiaState::ROS => LatLong(47.74316059448649, 40.51190628449829),
            RussiaState::RYA => LatLong(54.39433601431987, 40.53127537455453),
            RussiaState::SAK => LatLong(50.68412300880133, 142.94195016075383),
            RussiaState::SAM => LatLong(53.21627142230889, 50.179537107917064),
            RussiaState::SAR => LatLong(51.60898126643701, 46.3695029155857),
            RussiaState::SMO => LatLong(54.88884899186333, 32.78416812263122),
            RussiaState::SVE => LatLong(57.15864775234538, 60.959253299177234),
            RussiaState::TAM => LatLong(52.70809340647992, 41.484139654133415),
            RussiaState::TOM => LatLong(57.22058040620304, 84.32616998030811),
            RussiaState::TUL => LatLong(54.17212392046744, 37.61110266326255),
            RussiaState::TVE => LatLong(56.90307364295253, 35.82158399214958),
            RussiaState::TYU => LatLong(57.303712567695676, 67.74875755053762),
            RussiaState::ULY => LatLong(54.231931013474394, 48.23832692674429),
            RussiaState::VLA => LatLong(56.116268256707684, 40.41148698506672),
            RussiaState::VGG => LatLong(48.92013208143736, 44.27712202382678),
            RussiaState::VLG => LatLong(59.35097183639977, 39.994232314925064),
            RussiaState::VOR => LatLong(51.57219776949941, 39.47497292080911),
            RussiaState::YAR => LatLong(57.71027136190194, 39.69974667724346),
            RussiaState::YEV => LatLong(48.55855714450479, 132.3134896178688),
            RussiaState::AGB => LatLong(51.18670091038764, 115.1798283944045),
            RussiaState::NEN => LatLong(67.77476329621578, 55.326313603305195),
            RussiaState::UOB => LatLong(52.80821380229235, 104.73642749093855),
            RussiaState::KHM => LatLong(61.72632142075269, 70.08575221175774),
            RussiaState::CHU => LatLong(66.52941712892714, 173.81066855145207),
            RussiaState::YAN => LatLong(66.15657840134556, 76.17756915678915),
            RussiaState::MOW => LatLong(55.751905180890866, 37.61305363637575),
            RussiaState::SPE => LatLong(59.92638459198938, 30.338988919224757),
        }
    }
}

impl Coordinates for SouthAfricaState {
    fn latlong(self) -> LatLong {
        match self {
            SouthAfricaState::EC => LatLong(-32.145453961568975, 26.614915392233506),
            SouthAfricaState::FS => LatLong(-28.97343567394844, 26.31439283835097),
            SouthAfricaState::GT => LatLong(-26.11435748453011, 28.065851407441784),
            SouthAfricaState::KZN => LatLong(-29.427454211174773, 30.899817350308457),
            SouthAfricaState::LP => LatLong(-23.731530030183052, 29.49278204334819),
            SouthAfricaState::MP => LatLong(-25.95033119516894, 29.632123391750063),
            SouthAfricaState::NC => LatLong(-29.594477404707195, 18.257966743239393),
            SouthAfricaState::NW => LatLong(-26.38015328186762, 25.51400687659633),
            SouthAfricaState::WC => LatLong(-33.790411537502145, 19.59092107603962),
        }
    }
}

impl Coordinates for UAEState {
    fn latlong(self) -> LatLong {
        // Taken by searching on Google Maps and choosing a near point.
        match self {
            UAEState::AD => LatLong(24.44149569760228, 54.39101139352519),
            UAEState::AJM => LatLong(25.408299468032528, 55.51221229911271),
            UAEState::DXB => LatLong(25.187554288396296, 55.26724626606323),
            UAEState::FUJ => LatLong(25.12290521740282, 56.33461722781934),
            UAEState::RAK => LatLong(25.791849693277648, 55.964140603047404),
            UAEState::SHJ => LatLong(25.356467791356966, 55.426784192052395),
            UAEState::UAQ => LatLong(25.553813980476615, 55.55306945534394),
        }
    }
}

impl Coordinates for USAState {
    // Taken from https://awkwardhugs.com/state-latitudes-longitudes.
    fn latlong(self) -> LatLong {
        match self {
            USAState::AK => LatLong(61.370716, -152.404419),
            USAState::AL => LatLong(32.806671, -86.791130),
            USAState::AR => LatLong(34.969704, -92.373123),
            USAState::AZ => LatLong(33.729759, -111.431221),
            USAState::CA => LatLong(36.116203, -119.681564),
            USAState::CO => LatLong(39.059811, -105.311104),
            USAState::CT => LatLong(41.597782, -72.755371),
            USAState::DC => LatLong(38.897438, -77.026817),
            USAState::DE => LatLong(39.318523, -75.507141),
            USAState::FL => LatLong(27.766279, -81.686783),
            USAState::GA => LatLong(33.040619, -83.643074),
            USAState::HI => LatLong(21.094318, -157.498337),
            USAState::IA => LatLong(42.011539, -93.210526),
            USAState::ID => LatLong(44.240459, -114.478828),
            USAState::IL => LatLong(40.349457, -88.986137),
            USAState::IN => LatLong(39.849426, -86.258278),
            USAState::KS => LatLong(38.526600, -96.726486),
            USAState::KY => LatLong(37.668140, -84.670067),
            USAState::LA => LatLong(31.169546, -91.867805),
            USAState::MA => LatLong(42.230171, -71.530106),
            USAState::MD => LatLong(39.063946, -76.802101),
            USAState::ME => LatLong(44.693947, -69.381927),
            USAState::MI => LatLong(43.326618, -84.536095),
            USAState::MN => LatLong(45.694454, -93.900192),
            USAState::MO => LatLong(38.456085, -92.288368),
            USAState::MS => LatLong(32.741646, -89.678696),
            USAState::MT => LatLong(46.921925, -110.454353),
            USAState::NC => LatLong(35.630066, -79.806419),
            USAState::ND => LatLong(47.528912, -99.784012),
            USAState::NE => LatLong(41.125370, -98.268082),
            USAState::NH => LatLong(43.452492, -71.563896),
            USAState::NJ => LatLong(40.298904, -74.521011),
            USAState::NM => LatLong(34.840515, -106.248482),
            USAState::NV => LatLong(38.313515, -117.055374),
            USAState::NY => LatLong(42.165726, -74.948051),
            USAState::OH => LatLong(40.388783, -82.764915),
            USAState::OK => LatLong(35.565342, -96.928917),
            USAState::OR => LatLong(44.572021, -122.070938),
            USAState::PA => LatLong(40.590752, -77.209755),
            USAState::RI => LatLong(41.680893, -71.511780),
            USAState::SC => LatLong(33.856892, -80.945007),
            USAState::SD => LatLong(44.299782, -99.438828),
            USAState::TN => LatLong(35.747845, -86.692345),
            USAState::TX => LatLong(31.054487, -97.563461),
            USAState::UT => LatLong(40.150032, -111.862434),
            USAState::VT => LatLong(44.045876, -72.710686),
            USAState::VA => LatLong(37.769337, -78.169968),
            USAState::WA => LatLong(47.400902, -121.490494),
            USAState::WI => LatLong(44.268543, -89.616508),
            USAState::WV => LatLong(38.491226, -80.954453),
            USAState::WY => LatLong(42.755966, -107.302490),
            USAState::GU => LatLong(13.459940, 144.788805),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sanity check: the distance from a point to itself should be zero.
    #[test]
    fn self_distance_zero() {
        assert_eq!(USAState::CA.latlong().km_to(USAState::CA.latlong()), 0.0);
        assert_eq!(USAState::CO.latlong().km_to(USAState::CO.latlong()), 0.0);
    }

    #[test]
    fn near_distances() {
        assert!(USAState::NY.latlong().km_to(USAState::NJ.latlong()) < 500.0);
        assert!(USAState::NY.latlong().km_to(USAState::NH.latlong()) < 500.0);
        assert!(USAState::VT.latlong().km_to(USAState::NH.latlong()) < 500.0);
        assert!(USAState::SC.latlong().km_to(USAState::NC.latlong()) < 500.0);
        assert!(USAState::SD.latlong().km_to(USAState::ND.latlong()) < 500.0);
        assert!(USAState::WA.latlong().km_to(USAState::OR.latlong()) < 500.0);
    }

    #[test]
    fn far_distances() {
        assert!(USAState::NY.latlong().km_to(USAState::CO.latlong()) > 500.0);
        assert!(USAState::CA.latlong().km_to(USAState::HI.latlong()) > 500.0);
        assert!(USAState::WA.latlong().km_to(USAState::AK.latlong()) > 500.0);
        assert!(USAState::FL.latlong().km_to(USAState::TX.latlong()) > 500.0);
        assert!(USAState::FL.latlong().km_to(USAState::GU.latlong()) > 500.0);
    }
}
