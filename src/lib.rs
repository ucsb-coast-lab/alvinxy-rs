use std::f32;

mod test;

pub fn latlon2xy(lat: f32, lon: f32, lat0: f32, lon0: f32) -> (f32,f32) {
    let x = (lon - lon0) * mdeglon(lat0);
    let y = (lat-lat0) * mdeglat(lat0);
    (x,y)
}

pub fn xy2latlon(x: f32, y: f32, lat0:f32, lon0:f32) -> (f32,f32) {
    let lon = x / mdeglon(lat0) + lon0;
    let lat = y / mdeglat(lat0) + lat0;
    (lat,lon)
}

fn mdeglon(lat0: f32) -> f32 {
    let lat0rad = lat0.to_radians();
    111415.13 * lat0rad.cos() - (94.55 * (3.0*lat0rad).cos()) - (0.12 * (5.0*lat0rad).cos())
}

fn mdeglat(lat0: f32) -> f32 {
    let lat0rad = lat0.to_radians();
    111132.09 - (566.05 * (2.0*lat0rad).cos()) + (1.20 * (4.0 * lat0rad).cos()) - (0.002 * (6.0*lat0rad).cos())
}
