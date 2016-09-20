/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#[macro_use]
extern crate dbus_macros;

fn main() {
    dbus_interface!("com.dbus.test", "com.dbus.test", interface Hello {
        fn hello() -> String;
        fn hello_with_name(name: &str) -> String;
        fn greeting(greeting: &str, name: &str) -> String;
        fn greeting_with_separator(greeting: &str, separator: &str, name: &str) -> String;
        fn greeting_with_separator_and_end(greeting: &str, separator: &str, name: &str, end: &str) -> String;
        fn to_string5(arg1: &str, arg2: &str, arg3: &str, arg4: &str, arg5: &str) -> String;
        fn int_to_string(int: i32) -> String;
        fn get_variable() -> i32;
    });

    println!("{}", hello());
    println!("{}", hello_with_name("World"));
    println!("{}", greeting("Hi", "Me"));
    println!("{}", greeting_with_separator("Salut", " - ", "Toi"));
    println!("{}", greeting_with_separator_and_end("Salut", " - ", "Toi", "?"));
    println!("{}", to_string5("arg1 ", "arg2 ", "arg3 ", "arg4 ", "arg5"));
    let string: String = int_to_string(42);
    println!("{}", string);
    println!("{}", get_variable());
}