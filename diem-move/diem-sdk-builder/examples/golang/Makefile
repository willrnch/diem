#Copyright © Diem Foundation
# SPDX-License-Identifier: Apache-2.0

clean:
	- rm -rf go.mod go.sum diemstdlib diemtypes

test:
	cargo run -p diem-framework release
	cargo run -p diem-sdk-builder -- \
		--language go --module-name diemstdlib \
		--package-name testing \
		--with-diem-types "../../../../testsuite/generate-format/tests/staged/diem.yaml" \
		--target-source-dir . \
		"../../../framework/diem-framework/build/DiemFramework"
	- go mod init testing
	go mod edit -replace testing=`pwd`
	go get github.com/aptos-labs/serde-reflection/serde-generate/runtime/golang/bcs
	go get github.com/aptos-labs/serde-reflection/serde-generate/runtime/golang/serde
	go run stdlib_demo.go

.PHONY: clean test
