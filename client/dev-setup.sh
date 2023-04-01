#!/usr/bin/env bash

# Output coloring
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NO_COLOR='\033[0m'

ENV_FILE=./.env
ENV_EXAMPLE_FILE=./.env.example

if [ ! -f "$ENV_FILE" ]; then
    echo -e "${YELLOW} $ENV_FILE does not exist yet. Copying example file... ${NO_COLOR}"
	cp $ENV_EXAMPLE_FILE $ENV_FILE
fi

echo -e "${RED} You are ready to use your dev environment, enjoy hacking. ${NO_COLOR}"