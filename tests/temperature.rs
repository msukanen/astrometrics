mod temperature_tests {
    use astrometrics::{AsTemperature, Temperature, TemperatureApprox};

    #[test]
    fn comparison() {
        let a = 1.k();
        let b = 2.k();
        assert!(a < b);
        assert!(b >= a);
    }

    #[test]
    fn operators() {
        let a = 100.k();
        let b = 50.k();
        let c = a - b;
        assert_eq!(50.k(), c);

        let a = 100.k();
        let b = 50.k();
        assert!(a > b);
        assert_ne!(a, b);
        let c: Temperature = a / 2.0;
        assert!(c.approx(&50.k()));
    }
}
