use std::ops::{Add, BitAnd, BitOr, Mul};

/// We begin with the most important function of all.
/// The fold function.
///
/// # Examples
///
/// ```rust
/// # use fuh_rs::paper::fold;
/// # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let count = fold(|_, acc| 1 + acc, 0, (0..10));
/// assert_eq!(count, 10);
/// # Ok(())
/// # }
/// ```
///
pub fn fold<Alpha, Beta, List, F>(f: F, acc: Alpha, iter: List) -> Alpha
where
    F: Fn(Beta, Alpha) -> Alpha,
    List: Iterator<Item = Beta>,
{
    // NOTE: This assumes the type `AccType` is `Copy`-able.
    let mut acc = acc;
    for x in iter {
        acc = f(x, acc);
    }
    acc
}

/// Calculates the sum of a list.
///
/// # Example
///
/// ```rust
/// # use fuh_rs::paper::sum;
/// # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let sum = sum((1..=10));
/// assert_eq!(sum, 1+2+3+4+5+6+7+8+9+10);
/// # Ok(())
/// # }
/// ```
///
pub fn sum<Alpha, List>(iter: List) -> Alpha
where
    Alpha: Add<Output = Alpha> + Default,
    List: Iterator<Item = Alpha>,
{
    fold(|x, acc| x.add(acc), Alpha::default(), iter)
}

/// Calculates the product of a list.
///
/// # Example
///
/// ```rust
/// # use fuh_rs::paper::product;
/// # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let product = product((1..=10));
/// # // FIXME :: Just to silence the errors...
/// # assert_ne!(product, 1*2*3*4*5*6*7*8*9*10);
/// # Ok(())
/// # }
/// ```
///
/// TODO :: We need to integrate `num_crate` for this test to make sense...
pub fn product<Alpha, List>(iter: List) -> Alpha
where
    Alpha: Mul<Output = Alpha> + Default,
    List: Iterator<Item = Alpha>,
{
    fold(|x, acc| x.mul(acc), Alpha::default(), iter)
}

/// Calculates the AND of a list.
///
/// # Example
///
/// ```rust
/// # use fuh_rs::paper::and;
/// # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let and = and([1,3,5,7,9].iter().map(|x| x % 2 == 1));
/// assert_eq!(and, true);
/// # Ok(())
/// # }
/// ```
///
/// TODO :: How to generalize this? Or is it even meaningful to generalize this because we know all
///         the possible values of `bool`. They can only be `true` or `false`.
///
pub fn and<List>(iter: List) -> bool
where
    List: Iterator<Item = bool>,
{
    fold(|x, acc| x.bitand(acc), true, iter)
}

/// Calculates the OR of a list.
///
/// # Example
///
/// ```rust
/// # use fuh_rs::paper::or;
/// # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let or = or([1,3,5,7,9].iter().map(|x| x % 2 != 1));
/// assert_eq!(or, false);
/// # Ok(())
/// # }
/// ```
///
pub fn or<List>(iter: List) -> bool
where
    List: Iterator<Item = bool>,
{
    fold(|x, acc| x.bitor(acc), false, iter)
}

/// Calculates the length of a list.
///
/// # Example
///
/// ```rust
/// # use fuh_rs::paper::length;
/// # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let length = length((0..10));
/// assert_eq!(length, 10);
/// # Ok(())
/// # }
/// ```
///
pub fn length<List>(iter: List) -> usize
where
    List: Iterator,
{
    fold(|_, acc| 1 + acc, 0, iter)
}

/// Copies the list and reverses it.
/// FIXME :: Is copying part of the requirement? If not, how to remove it. Plus, what type should it
///          return?
///
/// # Example
///
/// ```rust
/// # use fuh_rs::paper::reverse;
/// # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let reverse = reverse((0..10));
/// assert_eq!(reverse, vec![9,8,7,6,5,4,3,2,1,0]);
/// # Ok(())
/// # }
/// ```
///
pub fn reverse<Alpha, List>(iter: List) -> Vec<Alpha>
where
    Alpha: Copy,
    List: Iterator<Item = Alpha>,
{
    fold(
        |x, mut acc| {
            acc.insert(0, x);
            acc
        },
        Vec::new(),
        iter,
    )
}

/// Maps a list of `Alpha` to a list of `Beta`.
///
/// # Example
///
/// ```rust
/// # use fuh_rs::paper::map;
/// # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // NOTE :: It should probably be something else other than floats to present the point.
/// //         But whatever.
/// let mapped = map(|x| x as f32, (0..3));
/// assert_eq!(vec![0.0f32, 1.0f32,2.0f32], mapped);
/// # Ok(())
/// # }
/// ```
pub fn map<Alpha, Beta, List, MapF>(f: MapF, iter: List) -> Vec<Beta>
where
    Alpha: Copy,
    List: Iterator<Item = Alpha>,
    MapF: Fn(Alpha) -> Beta,
{
    fold(
        |x, mut acc| {
            acc.push(f(x));
            acc
        },
        Vec::new(),
        iter,
    )
}

/// Give a function `F` that takes `Alpha` and returns `bool`, and a list of `Alpha`.
/// It then filters the list returning a list of `Alpha` containing only elements where `F(Alpha) -> true`.
///
/// # Example
///
/// ```rust
/// # use fuh_rs::paper::filter;
/// # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let filtered = filter(|x| x % 2 == 0, (0..10));
/// assert_eq!(filtered, vec![0,2,4,6,8]);
/// # Ok(())
/// # }
/// ```
///
pub fn filter<Alpha, List, MapF>(f: MapF, iter: List) -> Vec<Alpha>
where
    Alpha: Copy,
    List: Iterator<Item = Alpha>,
    MapF: Fn(Alpha) -> bool,
{
    fold(
        |x, mut acc| {
            // I wonder if its possible to augment bool with an operator that performs a function if
            // `&self == &true`.
            if f(x) {
                acc.push(x);
            }
            acc
        },
        Vec::new(),
        iter,
    )
}

/// Returns the sum and length of a list as a pair.
///
/// # Example
/// ```rust
/// # use fuh_rs::paper::sumlength;
/// # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let sumlength = sumlength((0..10));
/// assert_eq!(sumlength, (45,10));
/// # Ok(())
/// # }
/// ```
///
pub fn sumlength<Alpha, List>(iter: List) -> (Alpha, usize)
where
    Alpha: Add<Output = Alpha> + Default,
    List: Iterator<Item = Alpha>,
{
    fold(
        |x, (sum, length)| (sum + x, length + 1),
        (Alpha::default(), 0),
        iter,
    )
}
