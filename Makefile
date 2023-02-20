.PHONY: all run

module_name=libredis_state.so

TARGETBASEDIR=target

ifdef RELEASE
RELEASEFLAGS=--release
TARGETDIR=${TARGETBASEDIR}/release
else
TARGETDIR=${TARGETBASEDIR}/debug
endif

all::
	cargo build ${RELEASEFLAGS}

run: all
	redis-server --loadmodule ${TARGETDIR}/${module_name}

clean:
	rm -rf ${TARGETDIR}/${module_name} dump.rdb

distclean:
	rm -rf ${TARGETBASEDIR} dump.rdb