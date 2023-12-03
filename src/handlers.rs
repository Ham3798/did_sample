use warp::http::StatusCode;

use crate::blockchain::BlockchainClient;
use crate::crypto;
use crate::models::{DIDDocument, KeyPair};

pub async fn handle_create_did() -> Result<impl warp::Reply, warp::Rejection> {
    // 블록체인 클라이언트 인스턴스 생성
    let blockchain_client = BlockchainClient::new();

    // 키 쌍 생성
    let key_pair = crypto::generate_key_pair();

    // DID 생성
    let did = blockchain_client.create_did(&key_pair.public_key);

    // 여기에서, 필요하다면 DID를 블록체인에 기록할 수 있습니다.
    // 예: blockchain_client.record_did(&did, &key_pair.public_key);

    // DID 문서 생성
    let did_document = DIDDocument {
        context: "https://www.w3.org/ns/did/v1".to_string(),
        id: did,
        public_key: key_pair.public_key,
    };

    // DID 문서를 JSON 형식으로 반환
    Ok(warp::reply::with_status(
        warp::reply::json(&did_document),
        StatusCode::OK,
    ))
}
