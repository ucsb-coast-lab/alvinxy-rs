// $ cargo test -- --no-capture

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn it_works() {
        println!("From a test function");
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn cachuma_coordinates() {
        let (lat0,lon0) = (34.589000, -119.966000);
        let (ox,oy) = latlon2xy(34.589000, -119.966000,lat0,lon0);
        println!("The origin is at: ({},{})",ox,oy);
        assert!((ox == 0.0) && (oy == 0.0));
        // Directly above/North
        let (nx,ny) = latlon2xy(34.59000,-119.966000,lat0,lon0);
        println!("nx,ny = ({},{})",nx,ny);
        assert!(ny > oy);
        // Directly below/South
        let (sx,sy) = latlon2xy(34.58800,-119.966000,lat0,lon0);
        println!("nx,ny = ({},{})",sx,sy);
        assert!(sy < oy);
        // Directly left/West (longitude DECREASES going West)
        let (wx,wy) = latlon2xy(34.589000,-119.967000,lat0,lon0);
        println!("wx,wy = ({},{})",wx,wy);
        assert!(wx < ox);
        // Directly right/East (latitude INCREASES going East)
        let (ex,ey) = latlon2xy(34.589000,-119.96500,lat0,lon0);
        println!("ex,ey = ({},{})",ex,ey);
        assert!(ex > ox);
    }
}
