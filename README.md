# route-guardian

## What?
Allows you do declaratively define the routing table for a host and be confident that those routes will stay defined

## Why?
If you define static routes you want to be sure that they are ALWAYS present. Even if they are mistakenly deleted.

## Modes of operation

### Dictator

Manage all routes for all routing tables

### Strict

Manage all routes for routing tables that exist in configuration file (`/etc/route-guardian/config.yaml`)

### Normal (Default)

Only manage routes that appear in the configuration file (`/etc/route-guardian/config.yaml`)

## How?

### On start of route-guardian

1. Load configuration file
    a. Read /etc/iproute2/rt_tables and create table name to index mapping
    b. Read /etc/route-guardian/config.yaml
2. Read current routing table
3. (In strict Mode) Remove any routes not in configuration file that are in routing table
4. Add any route in configuration file that aren't in routing table

### Periodically

1. Read current routing table
2. (In strict Mode) Remove any routes not in configuration file that are in routing table
3. Add any route in configuration file that aren't in routing table

### On change of configuration file

1. Load configuration file
    a. Read /etc/iproute2/rt_tables and create table name to index mapping
    b. Read /etc/route-guardian/config.yaml
2. Read current routing table
3. (In strict Mode) Remove any routes not in configuration file that are in routing table
4. Add any route in configuration file that aren't in routing table

### On Route change (Route Addition)

1. (In Strict Mode) Delete added route if not in configuration file

### On Route change (Route Deletion)

1. Re-add deleted route if in configuration file


## Usage

### Docker Container

Launch a docker container to manage routes on the host
`docker run --cap-add=NET_ADMIN --name=route-guardian --rm --network=host -it route-guardian`

