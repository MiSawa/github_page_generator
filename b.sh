#!/bin/bash
bundle exec middleman build --verbose 2>&1 | awk '!/obsolete/'

