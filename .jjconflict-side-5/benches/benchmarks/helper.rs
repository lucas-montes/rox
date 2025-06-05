pub fn generate_simple_arithmetic() -> String {
    "print 1 + 2 * 3 - 4 / 5;".to_string()
}

pub fn generate_complex_program() -> String {
    r#"
        class Fibonacci {
            init(n) {
                this.n = n;
            }

            calculate() {
                if (this.n <= 1) {
                    return this.n;
                }

                var a = 0;
                var b = 1;
                var temp;

                for (var i = 2; i <= this.n; i = i + 1) {
                    temp = a + b;
                    a = b;
                    b = temp;
                }

                return b;
            }
        }

        var fib = Fibonacci(10);
        print fib.calculate();
    "#
    .to_string()
}

pub fn generate_expression_statements() -> String {
    r#"
        var x = 1 + 2 * 3 - 4 / 5;
        var y = (x + 1) * (x - 1);
        var z = x == y or x != y and x > y;
        print x + y * z;

        if (x > 0) {
            print "positive";
        }

        var counter = 0;
        {
            var local = counter + 1;
            counter = local * 2;
        }
    "#
    .to_string()
}

pub fn generate_nested_expression(depth: usize) -> String {
    let mut source = "1".to_string();
    for i in 2..=depth {
        source = format!("({} + {})", source, i);
    }
    source.push(';');
    format!("print {}", source)
}

pub fn generate_repeated_pattern(repetitions: usize) -> String {
    let base_expr = "var x = 1 + 2 * 3; print x; ";
    base_expr.repeat(repetitions)
}

pub fn generate_full_program(repetitions: usize) -> String {
    r#"
        fun factorial(n) {
            if (n <= 1) {
                return 1;
            }
            return n * factorial(n - 1);
        }

        class Calculator {
            init() {
                this.result = 0;
            }

            add(x, y) {
                this.result = x + y;
                return this.result;
            }

            multiply(x, y) {
                this.result = x * y;
                return this.result;
            }
        }

        var calc = Calculator();
        var sum = calc.add(5, 3);
        var product = calc.multiply(sum, 2);
        var fact = factorial(5);

        print "Sum: " + sum;
        print "Product: " + product;
        print "Factorial: " + fact;

        if (fact > product) {
            print "Factorial is larger";
        } else {
            print "Product is larger or equal";
        }
    "#
    .repeat(repetitions)
}
