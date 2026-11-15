use super::*;

mod ohlcv_tests {
    use super::*;

    #[test]
    fn test_ohlcv_ctr() {
        let open = 2.6; 
        let high = 65.0; 
        let low = 12.5; 
        let close = 50.5; 
        let volume = 100.5;
        let open_time = DateTime::from_timestamp_millis(123).unwrap();
        let close_time = DateTime::from_timestamp_millis(124).unwrap();

        let ohlcv = OHLCV::new(open, high, low, close, volume, open_time, close_time).unwrap();

        assert_eq!(open, ohlcv.open());
        assert_eq!(high, ohlcv.high());
        assert_eq!(low, ohlcv.low());
        assert_eq!(close, ohlcv.close());
        assert_eq!(volume, ohlcv.volume());
        assert_eq!(open_time, ohlcv.open_time());
        assert_eq!(close_time, ohlcv.close_time());
    }

    #[test]
    #[should_panic]
    fn test_ohlcv_ctr_fail_high_low() {
        let open = 2.6; 
        let high = 1.0; 
        let low = 12.5; 
        let close = 50.5; 
        let volume = 100.5;
        let open_time = DateTime::from_timestamp_millis(123).unwrap();
        let close_time = DateTime::from_timestamp_millis(124).unwrap();

        let ohlcv = OHLCV::new(open, high, low, close, volume, open_time, close_time).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ohlcv_ctr_fail_open_close_time() {
        let open = 2.6; 
        let high = 16.0; 
        let low = 12.5; 
        let close = 50.5; 
        let volume = 100.5;
        let open_time = DateTime::from_timestamp_millis(125).unwrap();
        let close_time = DateTime::from_timestamp_millis(124).unwrap();

        let ohlcv = OHLCV::new(open, high, low, close, volume, open_time, close_time).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ohlcv_ctr_fail_invalid_volume_1() {
        let open = 2.6; 
        let high = 16.0; 
        let low = 12.5; 
        let close = 50.5; 
        let volume = -100.5;
        let open_time = DateTime::from_timestamp_millis(123).unwrap();
        let close_time = DateTime::from_timestamp_millis(124).unwrap();

        let ohlcv = OHLCV::new(open, high, low, close, volume, open_time, close_time).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ohlcv_ctr_fail_invalid_volume_2() {
        let open = 2.6; 
        let high = 16.0; 
        let low = 12.5; 
        let close = 50.5; 
        let volume = 0.0;
        let open_time = DateTime::from_timestamp_millis(123).unwrap();
        let close_time = DateTime::from_timestamp_millis(124).unwrap();

        let ohlcv = OHLCV::new(open, high, low, close, volume, open_time, close_time).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ohlcv_ctr_fail_invalid_volume_3() {
        let open = 2.6; 
        let high = 16.0; 
        let low = 12.5; 
        let close = 50.5; 
        let volume = f64::INFINITY;
        let open_time = DateTime::from_timestamp_millis(123).unwrap();
        let close_time = DateTime::from_timestamp_millis(124).unwrap();

        let ohlcv = OHLCV::new(open, high, low, close, volume, open_time, close_time).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ohlcv_ctr_fail_invalid_volume_4() {
        let open = 2.6; 
        let high = 16.0; 
        let low = 12.5; 
        let close = 50.5; 
        let volume = f64::NAN;
        let open_time = DateTime::from_timestamp_millis(123).unwrap();
        let close_time = DateTime::from_timestamp_millis(124).unwrap();

        let ohlcv = OHLCV::new(open, high, low, close, volume, open_time, close_time).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ohlcv_ctr_fail_invalid_volume_5() {
        let open = 2.6; 
        let high = 16.0; 
        let low = 12.5; 
        let close = 50.5; 
        let volume = f64::NEG_INFINITY;
        let open_time = DateTime::from_timestamp_millis(123).unwrap();
        let close_time = DateTime::from_timestamp_millis(124).unwrap();

        let ohlcv = OHLCV::new(open, high, low, close, volume, open_time, close_time).unwrap();
    }
}
