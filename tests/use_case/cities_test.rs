// This test isn't mocked
// use super::get_test_db_pool;
// use trander_rust::repository::google_place_ids::MockGooglePlaceIdsRepository;
// use trander_rust::response::cities::Response;
// use trander_rust::service::location;
// use trander_rust::use_case::cities::{CitiesUseCase, GetParams, ImplCitiesUseCase};
//
// #[actix_rt::test]
// async fn test_get_ok() {
//     let mut mock_repo = MockGooglePlaceIdsRepository::new();
//     let pool = get_test_db_pool().await;
//     let mut conn = pool.get().unwrap();
//     let use_case = ImplCitiesUseCase;
//     let params = GetParams {
//         lng: 139.740053,
//         lat: 35.652104,
//         distance: 100.0,
//         direction_type: location::DirectionType::East,
//     };
//
//     mock_repo.expect_upsert().return_once(move |_, _| Ok(1));
//     let result = use_case.get(&mock_repo, &mut conn, params).await.unwrap();
//     let response = Response {
//         name: "Tokyo Shiba".to_string(),
//         distance: 0.0,
//         direction: "North".to_string(),
//         country_code: "JP".to_string(),
//         icon: "https://maps.gstatic.com/mapfiles/place_api/icons/v1/png_71/geocode-71.png"
//             .to_string(),
//         rating: 0.0,
//         vicinity: "Minato City".to_string(),
//         user_ratings_total: 0,
//         price_level: 0,
//         lat: 35.6496397,
//         lng: 139.7498754,
//         place_id: "ChIJi2VRV7aLGGARM7o0DjAmtag".to_string(),
//     };
//     assert_eq!(result, response);
// }
