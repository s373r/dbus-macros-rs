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

#![allow(let_and_return)]

extern crate dbus;

#[macro_export]
macro_rules! dbus_functions {
    ($factory:expr, $interface:ident,) => {
    };
    ($factory:expr, $interface:ident, fn $func_name:ident () -> $return_type:ty $block:block $($rest:tt)*) => {
        let $interface = $interface.add_m(
            $factory.method(stringify!($func_name), (), move |method| {
                let result = $block;
                Ok(vec!(method.msg.method_return().append1(result)))
            })
                .outarg::<$return_type, _>("result")
        );
        dbus_functions!($factory, $interface, $($rest)*);
    };
    ($factory:expr, $interface:ident, fn $func_name:ident ( $arg1:ident : $arg1_type:ty ) -> $return_type:ty $block:block $($rest:tt)*) => {
        let $interface = $interface.add_m(
            $factory.method(stringify!($func_name), (), move |method| {
                let $arg1: $arg1_type = method.msg.get1().unwrap();
                let result = $block;
                Ok(vec!(method.msg.method_return().append1(result)))
            })
                .inarg::<$arg1_type, _>(stringify!($arg1))
                .outarg::<$return_type, _>("result")
        );
        dbus_functions!($factory, $interface, $($rest)*);
    };
    ($factory:expr, $interface:ident, fn $func_name:ident ( $arg1:ident : $arg1_type:ty, $arg2:ident : $arg2_type:ty ) -> $return_type:ty $block:block $($rest:tt)*) => {
        let $interface = $interface.add_m(
            $factory.method(stringify!($func_name), (), move |method| {
                let ($arg1, $arg2): (Option<$arg1_type>, Option<$arg2_type>) = method.msg.get2();
                let $arg1 = $arg1.unwrap();
                let $arg2 = $arg2.unwrap();
                let result = $block;
                Ok(vec!(method.msg.method_return().append1(result)))
            })
                .inarg::<$arg1_type, _>(stringify!($arg1))
                .inarg::<$arg2_type, _>(stringify!($arg2))
                .outarg::<$return_type, _>("result")
        );
        dbus_functions!($factory, $interface, $($rest)*);
    };
    ($factory:expr, $interface:ident, fn $func_name:ident ( $arg1:ident : $arg1_type:ty, $arg2:ident : $arg2_type:ty, $arg3:ident : $arg3_type:ty) -> $return_type:ty $block:block $($rest:tt)*) => {
        let $interface = $interface.add_m(
            $factory.method(stringify!($func_name), (), move |method| {
                let ($arg1, $arg2, $arg3): (Option<$arg1_type>, Option<$arg2_type>, Option<$arg3_type>) = method.msg.get3();
                let $arg1 = $arg1.unwrap();
                let $arg2 = $arg2.unwrap();
                let $arg3 = $arg3.unwrap();
                let result = $block;
                Ok(vec!(method.msg.method_return().append1(result)))
            })
                .inarg::<$arg1_type, _>(stringify!($arg1))
                .inarg::<$arg2_type, _>(stringify!($arg2))
                .inarg::<$arg3_type, _>(stringify!($arg3))
                .outarg::<$return_type, _>("result")
        );
        dbus_functions!($factory, $interface, $($rest)*);
    };
    ($factory:expr, $interface:ident, fn $func_name:ident ( $arg1:ident : $arg1_type:ty, $arg2:ident : $arg2_type:ty, $arg3:ident : $arg3_type:ty, $arg4:ident : $arg4_type:ty) -> $return_type:ty $block:block $($rest:tt)*) => {
        let $interface = $interface.add_m(
            $factory.method(stringify!($func_name), (), move |method| {
                let ($arg1, $arg2, $arg3, $arg4): (Option<$arg1_type>, Option<$arg2_type>, Option<$arg3_type>, Option<$arg4_type>) = method.msg.get4();
                let $arg1 = $arg1.unwrap();
                let $arg2 = $arg2.unwrap();
                let $arg3 = $arg3.unwrap();
                let $arg4 = $arg4.unwrap();
                let result = $block;
                Ok(vec!(method.msg.method_return().append1(result)))
            })
                .inarg::<$arg1_type, _>(stringify!($arg1))
                .inarg::<$arg2_type, _>(stringify!($arg2))
                .inarg::<$arg3_type, _>(stringify!($arg3))
                .inarg::<$arg4_type, _>(stringify!($arg4))
                .outarg::<$return_type, _>("result")
        );
        dbus_functions!($factory, $interface, $($rest)*);
    };
    ($factory:expr, $interface:ident, fn $func_name:ident ( $arg1:ident : $arg1_type:ty, $arg2:ident : $arg2_type:ty, $arg3:ident : $arg3_type:ty, $arg4:ident : $arg4_type:ty, $arg5:ident : $arg5_type:ty) -> $return_type:ty $block:block $($rest:tt)*) => {
        let $interface = $interface.add_m(
            $factory.method(stringify!($func_name), (), move |method| {
                let ($arg1, $arg2, $arg3, $arg4, $arg5): (Option<$arg1_type>, Option<$arg2_type>, Option<$arg3_type>, Option<$arg4_type>, Option<$arg5_type>) = method.msg.get5();
                let $arg1 = $arg1.unwrap();
                let $arg2 = $arg2.unwrap();
                let $arg3 = $arg3.unwrap();
                let $arg4 = $arg4.unwrap();
                let $arg5 = $arg5.unwrap();
                let result = $block;
                Ok(vec!(method.msg.method_return().append1(result)))
            })
                .inarg::<$arg1_type, _>(stringify!($arg1))
                .inarg::<$arg2_type, _>(stringify!($arg2))
                .inarg::<$arg3_type, _>(stringify!($arg3))
                .inarg::<$arg4_type, _>(stringify!($arg4))
                .inarg::<$arg5_type, _>(stringify!($arg5))
                .outarg::<$return_type, _>("result")
        );
        dbus_functions!($factory, $interface, $($rest)*);
    };
}

#[macro_export]
macro_rules! dbus_class {
    ($bus_name:expr, $interface_name:expr, class $class_name:ident { $($functions:tt)* }) => {{
        extern crate dbus;

        let connection = dbus::Connection::get_private(dbus::BusType::Session).unwrap();
        connection.register_name($bus_name, dbus::NameFlag::ReplaceExisting as u32).unwrap();

        let factory = dbus::tree::Factory::new_fn::<()>();
        let class = factory.tree().add(factory.object_path(format!("/{}", stringify!($class_name)), ()).introspectable().add({
            let interface = factory.interface($interface_name, ());
            dbus_functions!(factory, interface, $($functions)*);
            interface
        }));
        class.set_registered(&connection, true).unwrap();

        move || {
            for _ in class.run(&connection, connection.iter(1000)) {
            }
        }
    }};
}

#[macro_export]
macro_rules! dbus_prototypes {
    ($connection:expr, $name:expr, $interface_name:expr, $class_name:ident, ) => {
    };
    ($connection:expr, $name:expr, $interface_name:expr, $class_name:ident, fn $func_name:ident () -> $return_type:ty; $($rest:tt)*) => {
        let $func_name = || -> $return_type {
            let message = dbus::Message::new_method_call($name, &format!("/{}", stringify!($class_name)), $interface_name, stringify!($func_name)).unwrap();
            let response = $connection.send_with_reply_and_block(message, 2000).unwrap();
            response.get1().unwrap()
        };
        dbus_prototypes!($connection, $name, $interface_name, $class_name, $($rest)*);
    };
    ($connection:expr, $name:expr, $interface_name:expr, $class_name:ident, fn $func_name:ident ($arg1:ident : $arg1_type:ty) -> $return_type:ty; $($rest:tt)*) => {
        let $func_name = |$arg1: $arg1_type| -> $return_type {
            let message = dbus::Message::new_method_call($name, &format!("/{}", stringify!($class_name)), $interface_name, stringify!($func_name)).unwrap();
            let message = message.append1($arg1);
            let response = $connection.send_with_reply_and_block(message, 2000).unwrap();
            response.get1().unwrap()
        };
        dbus_prototypes!($connection, $name, $interface_name, $class_name, $($rest)*);
    };
    ($connection:expr, $name:expr, $interface_name:expr, $class_name:ident, fn $func_name:ident ($arg1:ident : $arg1_type:ty, $arg2:ident : $arg2_type:ty) -> $return_type:ty; $($rest:tt)*) => {
        let $func_name = |$arg1: $arg1_type, $arg2: $arg2_type| -> $return_type {
            let message = dbus::Message::new_method_call($name, &format!("/{}", stringify!($class_name)), $interface_name, stringify!($func_name)).unwrap();
            let message = message.append2($arg1, $arg2);
            let response = $connection.send_with_reply_and_block(message, 2000).unwrap();
            response.get1().unwrap()
        };
        dbus_prototypes!($connection, $name, $interface_name, $class_name, $($rest)*);
    };
    ($connection:expr, $name:expr, $interface_name:expr, $class_name:ident, fn $func_name:ident ($arg1:ident : $arg1_type:ty, $arg2:ident : $arg2_type:ty, $arg3:ident : $arg3_type:ty) -> $return_type:ty; $($rest:tt)*) => {
        let $func_name = |$arg1: $arg1_type, $arg2: $arg2_type, $arg3: $arg3_type| -> $return_type {
            let message = dbus::Message::new_method_call($name, &format!("/{}", stringify!($class_name)), $interface_name, stringify!($func_name)).unwrap();
            let message = message.append3($arg1, $arg2, $arg3);
            let response = $connection.send_with_reply_and_block(message, 2000).unwrap();
            response.get1().unwrap()
        };
        dbus_prototypes!($connection, $name, $interface_name, $class_name, $($rest)*);
    };
    ($connection:expr, $name:expr, $interface_name:expr, $class_name:ident, fn $func_name:ident ($arg1:ident : $arg1_type:ty, $arg2:ident : $arg2_type:ty, $arg3:ident : $arg3_type:ty, $arg4:ident : $arg4_type:ty) -> $return_type:ty; $($rest:tt)*) => {
        let $func_name = |$arg1: $arg1_type, $arg2: $arg2_type, $arg3: $arg3_type, $arg4: $arg4_type| -> $return_type {
            let message = dbus::Message::new_method_call($name, &format!("/{}", stringify!($class_name)), $interface_name, stringify!($func_name)).unwrap();
            let message = message.append($arg1);
            let message = message.append($arg2);
            let message = message.append($arg3);
            let message = message.append($arg4);
            let response = $connection.send_with_reply_and_block(message, 2000).unwrap();
            response.get1().unwrap()
        };
        dbus_prototypes!($connection, $name, $interface_name, $class_name, $($rest)*);
    };
    ($connection:expr, $name:expr, $interface_name:expr, $class_name:ident, fn $func_name:ident ($arg1:ident : $arg1_type:ty, $arg2:ident : $arg2_type:ty, $arg3:ident : $arg3_type:ty, $arg4:ident : $arg4_type:ty, $arg5:ident : $arg5_type:ty) -> $return_type:ty; $($rest:tt)*) => {
        let $func_name = |$arg1: $arg1_type, $arg2: $arg2_type, $arg3: $arg3_type, $arg4: $arg4_type, $arg5: $arg5_type| -> $return_type {
            let message = dbus::Message::new_method_call($name, &format!("/{}", stringify!($class_name)), $interface_name, stringify!($func_name)).unwrap();
            let message = message.append($arg1);
            let message = message.append($arg2);
            let message = message.append($arg3);
            let message = message.append($arg4);
            let message = message.append($arg5);
            let response = $connection.send_with_reply_and_block(message, 2000).unwrap();
            response.get1().unwrap()
        };
        dbus_prototypes!($connection, $name, $interface_name, $class_name, $($rest)*);
    };
}

#[macro_export]
macro_rules! dbus_interface {
    ($name:expr, $interface_name:expr, interface $class_name:ident { $($prototypes:tt)* }) => {
        extern crate dbus;

        let connection = dbus::Connection::get_private(dbus::BusType::Session).unwrap();
        dbus_prototypes!(connection, $name, $interface_name, $class_name, $($prototypes)*)
    };
}