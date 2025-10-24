# rust开发环境配置
DEV_IMAGE_NAME := rs-grpc-dev
VERSION := v1.0

# rs-grpc grpc镜像名字配置
RPC_IMAGE_NAME := rs-grpc-proj
RPC_SERVICE := rpc-svc

# rs-grpc gateway配置
GATEWAY_IMAGE_NAME := rs-grpc-gateway
GATEWAY_SERVICE := rs-gateway
CONFIG_DIR := ./

# rust rs-grpc开发环境镜像构建
rust-dev:
	docker build . -f Dockerfile-dev -t ${DEV_IMAGE_NAME}:${VERSION}

# 镜像构建
rpc-build:
	docker build . -f Dockerfile -t ${RPC_IMAGE_NAME}:${VERSION}

# 容器运行
rpc-run:
	docker run --name=${RPC_SERVICE} -p 50051:50051 -p 8090:8090 \
	-v ${CONFIG_DIR}app.yaml:/app/app.yaml -itd ${RPC_IMAGE_NAME}:${VERSION}

rpc-rerun: rpc-stop rpc-run

# 重新构建和运行
rpc-rebuild-run: rpc-build rpc-stop rpc-run

# 删除容器
rpc-stop:
	docker stop ${RPC_SERVICE}
	docker rm ${RPC_SERVICE}

# 重启rpc
rpc-restart: rpc-stop rpc-run

# gateway 镜像构建
gateway-build:
	docker build . -f Dockerfile-gateway -t ${GATEWAY_IMAGE_NAME}:${VERSION}

# gateway 容器运行
gateway-run:
	docker run --name=${GATEWAY_SERVICE} -p 8080:8080 -p 8091:8091 -v ${CONFIG_DIR}app-gw.yaml:/app/app-gw.yaml \
	-itd ${GATEWAY_IMAGE_NAME}:${VERSION}

# 删除gateway容器
gateway-stop:
	docker stop ${GATEWAY_SERVICE}
	docker rm ${GATEWAY_SERVICE}

# 重启gateway
gateway-restart: gateway-stop gateway-run
