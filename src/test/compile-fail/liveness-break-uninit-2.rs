fn foo() -> int {
    let x: int;
    let i: int;

    while 1 != 2  {
        i = 0;
        break;
        x = 0; //! WARNING unreachable statement
    }

    log(debug, x); //! ERROR use of possibly uninitialized variable: `x`

    ret 17;
}

fn main() { log(debug, foo()); }
