use axum::{
    routing::get,
    Router,
    response::IntoResponse};
use axum::extract::Path;
use http::StatusCode;
use s2::cell::Cell;
use s2::cellid::CellID;
use google_maps::{GoogleMapsClient, LatLng, PlaceType};

pub fn get_routes() -> Router {

    Router::new()
        .route("/21", get(StatusCode::OK))
        .route("/21/coords/:binary", get(s2_coordinates_convert))
        .route("/21/country/:binary", get(s2_coordinates_to_country))

}

async fn s2_coordinates_convert(Path(binary): Path<String>) -> impl IntoResponse  {

    let id = u64::from_str_radix(&binary, 2).expect("Not a binary number!");
    let cell = Cell::from(CellID(id));

    let lat = cell.center().latitude().deg();
    let long = cell.center().longitude().deg();

    let latitude = format!("{}°{}'{:.3}''{}",
                           lat.abs() as u8, (lat.fract() * 60.0).abs() as u8,
                           ((lat.fract() * 60.0).fract() * 60.0).abs(),
                           if lat > 0.0 { "N" } else { "S" }
    );
    let longitude = format!("{}°{}'{:.3}''{}",
                            long.abs() as u8, (long.fract() * 60.0).abs() as u8,
                            ((long.fract() * 60.0).fract() * 60.0).abs(),
                            if long > 0.0 { "E" } else { "W" });

    format!("{} {}", latitude, longitude)
}

async fn s2_coordinates_to_country(Path(binary): Path<String>) -> impl IntoResponse  {

    let id = u64::from_str_radix(&binary, 2).expect("Not a binary number!");
    let cell = Cell::from(CellID(id));

    let lat = cell.center().latitude().deg();
    let long = cell.center().longitude().deg();

    let api_key =  std::env::var("GOOGLE_MAPS_API_KEY").unwrap().to_string();
    let google_maps_client = GoogleMapsClient::new(&api_key);

    let location = google_maps_client.reverse_geocoding(
        LatLng::try_from_f64(lat, long).unwrap()
    )
        .with_result_type(PlaceType::Country)
        .execute()
        .await
        .unwrap();

    let country = location.results[0].address_components[0].long_name.clone();

    country
}
