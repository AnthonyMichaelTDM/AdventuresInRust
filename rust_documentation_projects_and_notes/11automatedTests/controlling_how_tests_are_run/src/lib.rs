/* there are alot of ways you can run tests with cargo
 *
 * Some command line options go to cargo test, and some go to the resulting test binary.
 * To separate these two types of arguments, you list the arguments that go to cargo test followed by the separator --
 * and then the ones that go to the test binary. Running cargo test --help displays the options you can use with cargo test,
 * and running cargo test -- --help displays the options you can use after the separator --.
 *
 *
 *
 * RUNNING TESTS IN PARALLEL OR CONSECUTIVELY
 * when you run tests, rust will by default run each test in a separate thread, but sometimes this isn't optimal, or even possible
 *
 * when tests depend on each otehr, or share resources (such as a file), it is often better to run the tests one by one, this can be done
 * by adding the --test-threads flag
 * ex:
 * $ cargo test -- --test-threads=1
 *
 * this lets you specify the number of thread rust will use to run tests, setting it to 1, as was done in the example above, means rust
 * will not use any parallelism
 *
 *
 *
 * SHOWING FUNCTION OUTPUT
 * By default, if a test passes, Rust’s test library captures anything printed to standard output.
 * For example, if we call println! in a test and the test passes, we won’t see the println! output in the terminal;
 * we’ll see only the line that indicates the test passed.
 *
 * we can change this with the --show-output flag
 * ex:
 * $ cargo test -- --show-output
 *
 *
 *
 * RUNNING A SUBSET OF TESTS BY NAME
 * sometimes, running all your tests can take a long time, in these cases it is often desired to run 1, or only a few, test(s) at a time
 * rather than all at once.
 * You can choose which tests to run by passing cargo test the name or names of the test(s) you want to run as an argument.
 *
 * Running Single Tests
 * We can pass the name of any test function to cargo test to run only that test:
 * ex
 * $ cargo test one_hundred
 *
 * Filtering to Run Multiple Tests
 * We can specify part of a test name, and any test whose name matches that value will be run. For example, because two of our tests’ names contain add, we can run those two by running cargo test add:
 * ex
 * $ cargo test add
 *
     * Ignoring Some Tests Unless Specifically Requested
 * Sometimes a few specific tests can be very time-consuming to execute,
 * so you might want to exclude them during most runs of cargo test.
 * Rather than listing as arguments all tests you do want to run,
 * you can instead annotate the time-consuming tests using the ignore attribute to exclude them with the
 * #[ignore] attribute
 * If we want to run only the ignored tests, we can use cargo test -- --ignored
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
