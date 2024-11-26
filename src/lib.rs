
#[macro_export]
macro_rules! try_option {
    ( try {$($t:tt)*} else { $($e:tt)* } ) => {
        {
            (|| Some({
                $($t)*
            }))().unwrap_or_else (|| { $($e)* })
        }
    };
    ( $($i:tt)* ) => {
        {
            (|| Some({
                $($i)*
            }))()
        }
    };
}

#[macro_export]
macro_rules! try_result {
    ( try {$($t:tt)*} else { $($e:tt)* } ) => {
        {
            (|| Ok({
                $($t)*
            }))().unwrap_or_else (|_| { $($e)* })
        }
    };
    ( try {$($t:tt)*} else $err:ident { $($e:tt)* } ) => {
        {
            (|| Ok({
                $($t)*
            }))().unwrap_or_else (|$err| { $($e)* })
        }
    };
    ( try {$($t:tt)*} match { $($e:tt)* } ) => {
        {
            (|| Ok({
                $($t)*
            }))().unwrap_or_else (|e| match e { $($e)* })
        }
    };
    ( $($i:tt)* ) => {
        {
            (|| Ok({
                $($i)*
            }))()
        }
    };
}

#[macro_export]
macro_rules! flow_control {
    ($e:expr) => {
        match $e {
            std::ops::ControlFlow::Break(result) => return result,
            std::ops::ControlFlow::Continue(value) => value,
        }
    };
}