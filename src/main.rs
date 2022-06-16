macro_rules! fail {
    () => {
        fail = true;
    };
}

macro_rules! expect_equal {
    ( $x:expr, $y:expr ) => {
        if $x != $y {
            fail!();
        }
    };
}

macro_rules! eval_body {
    { $( $stmt:stmt );* } => {
        $(
            $stmt
            if fail {
                println!("{}: FAIL", $test_case);
            }
        )*
    };
}

macro_rules! test {
    ( $test_case:expr, $body:block ) => {
        let fail = false;
        eval_body!($body);
        println!("{}: PASS", $test_case);
    };
}

fn main() {
    test!("fails when first statement passes but second fails", {
        expect_equal!(2, 1 + 1);
        expect_equal!(3, 1 + 1);
    });
    test!("fails when first statement fails", {
        expect_equal!(3, 1 + 1);
        expect_equal!(2, 1 + 1);
    });
    test!("passes when all statements pass", {
        expect_equal!(2, 1 + 1);
        expect_equal!(2, 1 + 1);
    });
}
