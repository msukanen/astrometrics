mod mass_tests {
    use astrometrics::AsMass;

    #[test]
    fn comparison() {
        let a = 1.kg();
        let b = 1.5.kg();
        let c = 1.0.kg();
        assert!(a < b);
        assert!(a == c);
        assert!(b > c);
        assert!(a < 2.0);
    }

    #[test]
    fn operators() {
        let a = 1.kg();
        let b = 0.5.kg();
        let a_b = &a + &b;
        assert_eq!(1.5.kg(), a_b);
        assert!(1.5.kg() == a_b);// see that Ord impl works
    }
}
