use num_traits::Num;

pub fn gcd<T: Num + Copy>(lhs: T, rhs: T) -> T {
    if rhs.is_zero() {
        lhs
    } else {
        gcd(rhs, lhs % rhs)
    }
}

pub fn lcm<T: Num + Copy>(lhs: T, rhs: T) -> T {
    (lhs * rhs) as T / gcd(lhs, rhs) as T
}
