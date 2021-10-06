#![allow(unused_macros)]

mod average {
    #[macro_export]
    macro_rules! get_mean {
        ($vector: expr) => {{
            println!("averages that return rust f64s will be rounded to become a rust usize");
            let vector_: Vec<usize> = $vector;
            let length_: usize = vector_.len();
            let mut counter_ = 0;
            let mut temp: usize;
            let mut total: usize = 0;
            for _i in &vector_ {
                temp = vector_[counter_];
                total = total + temp;
                counter_ = counter_ + 1;
            }
            let final_ = total / length_;
            final_
        }};
    }

    #[macro_export]
    macro_rules! get_range {
        ($vector: expr) => {{
            let vector_: Vec<isize> = $vector;
            let min_value_: &isize = vector_.iter().min().unwrap();
            let max_value_: &isize = vector_.iter().max().unwrap();
            // println!("min value in range: {}", min_value_);
            // println!("max value in range: {}", max_value_);
            let range_: isize = max_value_ - min_value_;
            range_
        }};
    }

    #[macro_export]
    macro_rules! get_median {
        ($vector: expr) => {{
            let vector_: Vec<isize> = $vector;
            let middle_ = vector_.len() / 2;
            let median_ = vector_[middle_];
            //println!("median: {}", median_);
            median_
        }};
    }
}