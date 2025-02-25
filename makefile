include .env

.PHONY: train data-log autopep8 autoflake isort flake8 pylint format check prepare-commit

#* Git Rules
isort:
	isort --settings-path=$(MAKE_CONFIG_FILE) $(FORMAT_CHECK_SRC)

autoflake:
	autoflake --remove-all-unused-imports --in-place --recursive $(FORMAT_CHECK_SRC)

autopep8:
	autopep8 --in-place --recursive $(FORMAT_CHECK_SRC)

pylint:
	pylint --rcfile=$(PYLINT_CONFIG_FILE) $(FORMAT_CHECK_SRC)

flake8:
	flake8 --config=$(MAKE_CONFIG_FILE) $(FORMAT_CHECK_SRC)

format: autoflake autopep8 isort

check: flake8 pylint

prepare-commit: autoflake autopep8 isort flake8 pylint