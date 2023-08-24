# Copyright © Diem Foundation
# SPDX-License-Identifier: Apache-2.0

test:
	poetry run python -m unittest discover -s diem_sdk/ -p '*.py' -t ..

test-coverage:
	poetry run python -m coverage run -m unittest discover -s diem_sdk/ -p '*.py' -t ..
	poetry run python -m coverage report

fmt:
	find ./examples ./diem_sdk *.py -type f -name "*.py" | xargs poetry run autoflake -i -r --remove-all-unused-imports --remove-unused-variables --ignore-init-module-imports
	poetry run isort diem_sdk examples
	poetry run black diem_sdk examples

lint:
	- poetry run mypy diem_sdk
	- poetry run flake8 diem_sdk examples

examples:
	poetry run python -m examples.async-read-aggregator
	poetry run python -m examples.async-simple-nft
	poetry run python -m examples.async-simulate-transfer-coin
	poetry run python -m examples.async-transfer-coin
	poetry run python -m examples.async-transfer-two-by-two
	poetry run python -m examples.read-aggregator
	poetry run python -m examples.simple-nft
	poetry run python -m examples.simulate-transfer-coin
	poetry run python -m examples.transfer-coin
	poetry run python -m examples.transfer-two-by-two

.PHONY: examples fmt lint test
