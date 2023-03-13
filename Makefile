.PHONY: all run

module_name=libredis_state.so

# override
OSNICK?=ubuntu22.04
ARCH?=x86_64
OS?=Linux
REDISVERSION?=7.0.9-1
VERSION?=99.99.99

TARGETBASEDIR=target
ifdef RELEASE
RELEASEFLAGS=--release
TARGETDIR=${TARGETBASEDIR}/release
S3TARGET=s3://redismodules/redisstatemachine
else
TARGETDIR=${TARGETBASEDIR}/debug
S3TARGET=s3://redismodules/redisstatemachine/snapshots
endif
module_dest=${TARGETDIR}/${module_name}

all::
	cargo build ${RELEASEFLAGS}

run: all
	redis-server --loadmodule ${module_dest}

bgtest: all
	redis/redis-server --daemonize yes --loadmodule ${module_dest}
	pytest --junit-xml=results.xml
	redis/redis-cli shutdown

clean:
	rm -rf ${module_dest} dump.rdb

distclean:
	rm -rf ${TARGETBASEDIR} dump.rdb redis *.zip

deps:
	pip3 install -r tests/requirements.txt
	wget -q https://redismodules.s3.amazonaws.com/redis-stack/dependencies/redis-${REDISVERSION}-${OS}-${OSNICK}-${ARCH}.tgz -O redis.tgz
	tar -xpf redis.tgz
	rm *.tgz
	mv redis* redis
	chmod a+x redis/*

pack:
	ramp pack -m ramp.yml target/release/libredis_state.so -o libredis_state.Linux-${OSNICK}-${ARCH}.${VERSION}.zip
ifdef PUBLISH
	s3cmd --access_key=${AWS_ACCESS_KEY_ID} \
		--secret_key=${AWS_SECRET_ACCESS_KEY} \
		--region=${AWS_REGION} \
		put -P *.zip \
		${S3TARGET}
endif