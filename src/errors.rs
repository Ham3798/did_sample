// 에러를 정의하기 위한 커스텀 에러 타입

pub enum ServerError {
    // 여러 종류의 에러를 정의할 수 있습니다.
    Io(std::io::Error),
    // 추가 에러 타입
}

// 이렇게 에러를 처리하는 함수를 여러 개 정의할 수 있습니다.
impl From<std::io::Error> for ServerError {
    fn from(error: std::io::Error) -> Self {
        ServerError::Io(error)
    }
}
