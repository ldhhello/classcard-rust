# Classcard-rust - 단어배틀 호환 클라이언트

#### 단어배틀을 이제 터미널에서 즐겨보세요!

## 빌드 하는 법
일반적인 러스트 프로젝트와 빌드 방법이 동일합니다.
```bash
cargo build --release
cargo run --release
```
를 실행해 빌드 및 실행할 수 있습니다.

### RUSTLS 기반 빌드를 하고 싶은 경우
```bash
cargo build --release --no-default-features --features rustls
cargo run --release --no-default-features --features rustls
```
를 실행하면 됩니다.

기본값인 native-tls 기반 빌드가 먹히지 않을 때 활용해보세요.
리눅스 환경에서 openssl이 깔려있지 않을 때 유용합니다.

## 리눅스에서 설치하는 법
```bash
wget https://github.com/ldhhello/classcard-rust/releases/download/v1.1.1/classcard-client && chmod +x classcard-client
```
을 실행하면 됩니다.

## 실행 화면
![예시 이미지](example.png)

## 옵션
--buffer-size : 정답 버퍼의 크기를 변경합니다. 기본값은 5입니다.
저걸 1로 바꾸면 답을 입력할 때마다 선생님 화면에 표시되게 할 수 있습니다.
또는 터무니없이 큰 값으로 바꾸면 최종 제출 전까지 선생님 화면에 전혀 표시되지 않습니다.

--correct-score : 맞았을 때 받는 점수를 변경합니다. 기본값은 100입니다.

--fail-score : 틀렸을 때 받는 점수를 변경합니다. 기본값은 0입니다.

## 기타
문제 발생 시 issues 탭에 올려주시거나, 아니면 디미고 3학년 4반에 직접 찾아오시면 됩니다.

이 클라이언트를 사용함으로써 발생한 피해는 책임지지 않습니다. 알아서 잘 사용하세요.
