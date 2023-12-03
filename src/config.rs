// 환경 변수나 설정 파일로부터 서버 설정을 로드하기 위한 구조체 및 함수

pub struct Config {
    // 서버 설정 필드
    pub server_address: String,
    // 추가 설정 필드
}

impl Config {
    pub fn new() -> Self {
        // 환경 변수나 설정 파일에서 설정을 로드하는 로직 구현
        Self {
            server_address: "127.0.0.1:8080".to_string(),
        }
    }
}
