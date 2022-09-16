# server-script
## 마인크래프트 서버 쉽게 실행하기 

[![Build](https://github.com/dolphin2410/server-script/actions/workflows/rust.yml/badge.svg)](https://github.com/dolphin2410/server-script/actions/workflows/rust.yml)

### 설정하기
- CLI 및 JSON 기반 설정을 지원합니다

1. [Releases](https://github.com/dolphin2410/server-script/releases) 에서 원하는 아키텍쳐의 빌드 버전을 다운로드 받습니다.

2. `server.conf.json` 파일을 이용해 서버를 설정합니다. ( 필수 x )

3. `server-script[.exe]`를 실행합니다.

### server.conf.json
다음과 비슷하게 설정하세요.
```json
{
    "server": "http://clip.aroxu.me/download?mc_version=1.19.2",
    "debug": true,
    "debug_port": 8080,
    "backup": true,
    "restart": true,
    "memory": 8,
    "plugins": [
        "<PLUGIN1_URL>",
        "<PLUGIN2_URL>",
        "<PLUGIN3_URL>"
    ],
    "jvm_args": [
        "--Dcom.mojang.xxx=true"
    ],
    "no_update": true
}
```

### CLI
***`server.conf.json`에서 설정하지 않고 CLI에서 직접 설정 및 실행할 수 있습니다. `server.conf.json`에 중복되는 설정이 있을때, 갈아엎습니다***
- 서버 URL
```bash
server-script.exe --server http://clip.aroxu.me/download?mc_version=1.19.2 # 서버 jar URL 설정
```
- 메모리
```bash
server-script.exe --memory 8 # 8GB 설정
```
- 디버그
```bash
server-script.exe --debug --debug-port 8080 # 디버그 설정 및 8080 포트 설정
```
- 백업
```bash
server-script.exe --backup # 서버 종료 후 자동 백업
```
- 재시작
```bash
server-script.exe --restart # 서버 종료 후 자동 재시작
```
- no-update
```bash
server-script.exe --no-update # 서버가 존재하지 않는 경우에만 다운로드 받기 (매번 다운로드 하지 않는다)
```

- save-config
```bash
server-script.exe --save-config # cli 설정을 server.conf.json 파일에 저장
```

### Thanks to
* 이 프로젝트는 [monun/server-script](https://github.com/monun/server-script) 의 Rust 버전 포크이며, 동일한 MIT License를 같고 있습니다.
* [aroxu/server-script](https://github.com/aroxu/server-script) 와 같이 바이너리 형태로 배포합니다.

### 다른 구현체들
기본 서버 실행기: [monun](https://github.com/monun/server-script/)

Go언어로 제작된 서버 실행기: [aroxu](https://github.com/aroxu/server-script)

Paper 전용 서버 실행기: [monun](https://github.com/monun/server-script/tree/paper)
