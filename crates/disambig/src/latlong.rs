//! Maps locations to approximate coordinates and calculates approximate distances.

use opltypes::states::*;

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
