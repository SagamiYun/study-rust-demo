use std::fmt;

// AppError 是自定义错误类型，它可以是当前包中定义的任何类型，在这里为了简化，我们使用了单元结构体作为例子。
// 为 AppError 自动派生 Debug 特征
// #[derive(Debug)]
// struct AppError;

struct AppError {
    code: usize,
    message: String,
}

// 为 AppError 实现 std::fmt::Display 特征
// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "An Error Occurred, Please Try Again!") // user-facing output
//     }
// }


// 根据错误码显示不同的错误信息
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            500 => "The Server inner error!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppError {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}

// 一个示例函数用于产生 AppError 错误
// fn produce_error() -> Result<(), AppError> {
//     Err(AppError)
// }

fn produce_error() -> Result<(), AppError> {
    Err(AppError {
        code: 500,
        message: String::from("Page not found"),
    })
}

pub fn a_easy_error() {
    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("No error"),
    }

    eprintln!("{:?}", produce_error()); // Err({ file: src/main.rs, line: 17 })
}

pub fn a_depth_error() {
    match produce_error() {
        Err(e) => eprintln!("{}", e), // 抱歉，未找到指定的页面!
        _ => println!("No error"),
    }

    eprintln!("{:?}", produce_error()); // Err(AppError { code: 404.html, message: Page not found })

    eprintln!("{:#?}", produce_error());
    // Err(
    //     AppError { code: 404.html, message: Page not found }
    // )
}