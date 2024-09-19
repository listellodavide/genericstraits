mod primes {
    use num_traits::int::PrimInt;

    // This isn't an easy topic in Rust - if you're still
    // learning and some of this doesn't make sense, don't
    // worry about it. Rust can have a steep learning curve,
    // and traits (especially like the ones I'm using here)
    // can be quite weird at first.

    // I've changed the function to not take a reference,
    // but take a value of type `N`.
    // `N` can be any integer type, because of the
    // `<N: PrimInt>` declaration.
    fn is_prime<N: PrimInt>(number: N) -> bool {
        // Since we don't know what the type of the number is,
        // we can't just write `3` or any other number -
        // we have to use the trait's methods to create
        // these numbers ourself.
        let zero = N::zero();
        let one = N::one();
        let two = one + one;
        let three = one + two;

        // An unfortunate side effect of defining the variables
        // like this is that we can't do pattern matching so
        // easily, since variables can't be used in patterns.
        // So I expanded the `match` statement into some
        // `if` blocks.

        // One is not a prime number, so I changed the semantics
        // here such that `is_prime(1) == false`.
        if number <= one {
            return false;
        }
        // This is an easy way of checking if something is equal
        // to a small list of variables.
        // This is equivalent to `number == two || number == three`.
        if [two, three].contains(&number) {
            return true;
        }
        if number % two == zero {
            return false;
        }

        // You don't need the `: i32` here.
        // I think it was only required before because `number`
        // was a reference and not a value, and the compiler
        // wasn't quite sure what types everything was supposed
        // to be.
        //
        // If you pass in a reference to something like an integer,
        // you can write `*number` to get the original integer back.
        // So you could have written
        //    let try_limit = *number / 2
        // instead of
        //    let try_limit: i32 = number / 2
        let try_limit = number / two;
        let mut n = three;
        loop {
            if number % n == zero {
                return false;
            }
            if n <= try_limit {
                // The trait AddAssign allows you to write `n += ...`
                // But we haven't made `N: AddAssign`, so we can use this instead.
                // The compiler will likely optimise it into the same code anyway.
                n = n + two;
            } else {
                return true;
            }
        }
    }

    pub fn gen_primes(number: i32) -> Vec<i32> {
        let mut primes = Vec::<i32>::new();
        for i in 1..=number {
            if self::is_prime(i) {
                primes.push(i);
            }
        }
        primes
    }
}

fn main() {
    println!("{:#?}", primes::gen_primes(10_000));
}