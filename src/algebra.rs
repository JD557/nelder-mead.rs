use std::vec::Vec;

pub fn sum(p1: &Vec<f64>, p2: &Vec<f64>) -> Vec<f64> {
    p1.iter().zip(p2.iter()).map(|(x, y)| x + y).collect()
}
pub fn diff(p1: &Vec<f64>, p2: &Vec<f64>) -> Vec<f64> {
    p1.iter().zip(p2.iter()).map(|(x, y)| x - y).collect()
}
pub fn mult(k: f64, p: &Vec<f64>) -> Vec<f64> {
    p.iter().map(|x| k * x).collect()
}
pub fn avg(ps: &[Vec<f64>]) -> Vec<f64> {
    let head = ps[0].clone();
    mult(
        1.0 / ps.len() as f64,
        &ps.iter().skip(1).fold(head, |x, y| sum(&x, &y)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(
            sum(&vec![1.0, 2.0, 3.0], &vec![5.0, 6.0, 7.0]),
            vec![6.0, 8.0, 10.0]
        );
    }

    #[test]
    fn test_diff() {
        assert_eq!(
            diff(&vec![1.0, 2.0, 3.0], &vec![5.0, 6.0, 7.0]),
            vec![-4.0, -4.0, -4.0]
        );
    }

    #[test]
    fn test_mult() {
        assert_eq!(mult(2.0, &vec![5.0, 6.0, 7.0]), vec![10.0, 12.0, 14.0]);
    }

    #[test]
    fn test_avg() {
        assert_eq!(
            avg(&vec![vec![1.0, 2.0, 3.0], vec![5.0, 6.0, 7.0]]),
            vec![3.0, 4.0, 5.0]
        );
    }

}
