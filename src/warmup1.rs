




pub fn sleepIn(weekday: bool, vacation: bool) -> bool {

    return vacation || !weekday


}

pub fn monkeyTrouble(aSmile: bool, bSmile: bool) -> bool {

   return aSmile && bSmile || !aSmile && !bSmile;
}

pub fn sumDouble(a: i32, b: i32) -> i32 {

    if a == b {

        return (a+b) * 2;
    }

    return a+b;




}

pub fn diff21(n: i32) -> i32 {

    return i32::abs(n - 21) * 2;
}