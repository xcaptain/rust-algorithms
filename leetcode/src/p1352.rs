/// https://leetcode-cn.com/problems/product-of-the-last-k-numbers/
/// a faster way is to use prefix product, next k product equal to
/// total product / pre n-k product
struct ProductOfNumbers {
    inner: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn add(&mut self, num: i32) {
        self.inner.push(num);
    }

    fn get_product(&self, k: i32) -> i32 {
        let l = self.inner.len();
        let kk = k as usize;
        assert!(kk <= l);
        let start = l - kk;
        let mut product = 1;
        for i in start..l {
            product *= self.inner[i];
        }
        product
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut product = ProductOfNumbers::new();
        product.add(3);
        product.add(0);
        product.add(2);
        product.add(5);
        product.add(4);

        assert_eq!(20, product.get_product(2));
        assert_eq!(40, product.get_product(3));
        assert_eq!(0, product.get_product(4));

        product.add(8);
        assert_eq!(32, product.get_product(2));
    }
}