#!/bin/bash

# Function for cleaner code
function run_make_target {
  make "$1" || exit 1  # Exit script on failure (make returns non-zero)
}

# Get the image version from the first argument (if provided)
if [ -n "$1" ]; then
  IMG="registry.aqueous.cloud/public/app-operator:$1"
else
  echo "Error: Please provide an image version as an argument."
  exit 1
fi

# Uninstall the application
#run_make_target uninstall

# Undeploy the application (if applicable)
#run_make_target undeploy

# Build and push the docker image
run_make_target "docker-build docker-push IMG=$IMG"

# Deploy the application (if applicable)
#run_make_target "deploy IMG=$IMG"

#echo "All tasks completed!"
