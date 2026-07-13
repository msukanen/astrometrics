mod spatial_tests {
    use astrometrics::AsSpatialUnit;
    use astrometrics::{Megastructure, SpatialContained};

    #[test]
    fn gr_range_works() {
        let gr = Megastructure::from(((6.ly(), 12.ly()), (15.ly(), 30.ly()), (40.ly(), 42.ly())));
        assert_eq!(Some(SpatialContained::VisibleDisk), gr.contains(&7.0.ly()));
        assert_ne!(Some(SpatialContained::Arms), gr.contains(&(15.0 - f64::EPSILON*10.0).ly()));
        assert_eq!(Some(SpatialContained::Arms), gr.contains(&(15.0 + f64::EPSILON*10.0).ly()));
    }

    #[test]
    fn comparison() {
        let a = 1.ly();
        assert!(a < 2);
        assert!(2 > a);
    }
}