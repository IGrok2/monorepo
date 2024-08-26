#!/bin/bash
cd src/backend || exit 1
golangci-lint run --fix
