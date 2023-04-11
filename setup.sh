#!/usr/bin/env bash

# Output coloring
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Check if docker is installed and working
if ! command -v docker &> /dev/null
then
    echo -e "${RED}ERROR: Docker is not installed or not in the system path${NC}"
    exit 1
fi
if ! docker ps &> /dev/null
then
    echo -e "${RED}ERROR: Docker is installed but not working properly${NC}"
    exit 1
fi

# Check if docker-compose is installed and working
if ! command -v docker-compose &> /dev/null
then
    echo -e "${RED}ERROR: Docker Compose is not installed or not in the system path${NC}"
    exit 1
fi
if ! docker-compose ps &> /dev/null
then
    echo -e "${RED}ERROR: Docker Compose is installed but not working properly${NC}"
    exit 1
fi

# Check if at least one argument is given
if [[ $# -lt 1 ]]; then
  echo -e "${RED}Error: at least one argument is required!${NC}"
  exit 1
fi

# Check if the first argument is start or stop
if [[ "$1" != "start" && "$1" != "stop" ]]; then
  echo -e "${RED}Error: the first argument must be 'start' or 'stop'!${NC}"
  exit 1
fi

# Check if there is no second argument for stop
if [[ "$1" == "stop" ]] && [[ $# -gt 1 ]]; then
  echo -e "${RED}Error: 'stop' do not accept a second argument!${NC}"
  exit 1
fi

# Check if there is a second argument for start
if [[ "$1" == "start" && $# -lt 2 ]]; then
  echo -e "${RED}Error: 'start' requires a second argument ('client', 'server', 'database')!${NC}"
  exit 1
fi

# Check if the second argument is client, server, or database for start
if [[ "$1" == "start" && "$2" != "client" && "$2" != "server" && "$2" != "database" ]]; then
  echo -e "${RED}Error: the second argument must be 'client', 'server', or 'database' for 'start'!${NC}"
  exit 1
fi

# Start or stop the service according to the first argument
case "$1" in
  start)
    if [[ "$2" == "database" ]]; then
      service="database mailhog"
    else
      service="$2"
    fi
    echo -e "${GREEN}Starting $service...${NC}"
    eval "docker-compose up -d --build $service"
    ;;
  stop)
    echo -e "${GREEN}Stopping all services...${NC}"
    docker-compose down
    ;;
esac
