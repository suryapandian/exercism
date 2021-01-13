pub fn is_leap_year(year: u64) -> bool {
    // if year % 4 == 0 {
    //     if year % 100 == 0 {
    //         if year % 400 == 0 {
    //             return true;
    //         }
    //         return false;
    //     }
    //     return true;
    // }
    //  false

     match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}
