pub fn get_deltas(precision: u8) -> (f64, f64) {
    // calculate number of bits for latitude and longitude
    let lat_bits = (2 * precision) + (precision as f32 / 2.0).floor() as u8;
    let long_bits = (2 * precision) + (precision as f32 / 2.0).ceil() as u8;

    // calculate deltas
    let lat_delta = 180.0 / 2_u32.pow(lat_bits as u32) as f64;
    let long_delta = 260.0 / 2_u32.pow(long_bits as u32) as f64;

    (lat_delta, long_delta)
}

pub fn get_bounds(lat_min: f64, lat_max: f64, long_min: f64,
        long_max: f64, precision: u8) -> Vec<(f64, f64, f64, f64, f64)> {
    // calculate indices for minimum and maximum coordinates
    let (lat_delta, long_delta) = get_deltas(precision);

    let lat_min_index = (lat_min / lat_delta).floor() as i32;
    let lat_max_index = (lat_max / lat_delta).ceil() as i32;

    let long_min_index = (long_min / long_delta).floor() as i32;
    let long_max_index = (long_max / long_delta).ceil() as i32;

    // calculate geohash bounds
    let geohash_area = lat_delta * long_delta;

    let mut geohash_bounds = Vec::new();
    for lat_index in lat_min_index..lat_max_index {
        let lat_index = lat_index as f64;
        for long_index in long_min_index..long_max_index {
            let long_index = long_index as f64;

            // calculate geohash bounds
            let bound_lat_min = lat_index * lat_delta;
            let bound_lat_max = (lat_index + 1.0) * lat_delta;

            let bound_long_min = long_index * long_delta;
            let bound_long_max = (long_index + 1.0) * long_delta;

            // calculate image area percentage
            let image_lat_delta = lat_max.min(bound_lat_max)
                - lat_min.max(bound_lat_min);
            let image_long_delta = long_max.min(bound_long_max)
                - long_min.max(bound_long_min);
            let area_percentage =
                (image_lat_delta * image_long_delta) / geohash_area;

            // add to geohash bounds
            geohash_bounds.push((bound_lat_min, bound_lat_max,
                bound_long_min, bound_long_max, area_percentage));
        }
    }

    geohash_bounds
}

#[cfg(test)]
mod tests {
    #[test]
    fn coordinate_delta() {
        assert_eq!(super::get_deltas(1), (45.0, 32.5));
        assert_eq!(super::get_deltas(2), (5.625, 8.125));
        assert_eq!(super::get_deltas(3), (1.40625, 1.015625));
        assert_eq!(super::get_deltas(4), (0.17578125, 0.25390625));
        assert_eq!(super::get_deltas(5), (0.0439453125, 0.03173828125));
        assert_eq!(super::get_deltas(6), (0.0054931640625, 0.0079345703125));
    }

    #[test]
    fn bounds() {
        // TODO - figure out how to unit test
        let bounds = super::get_bounds(-80.0, -70.0, 70.0, 80.0, 3);
        for bound in bounds {
            println!("{:?}", bound);
        }
    }
}
