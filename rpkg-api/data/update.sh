#!/bin/sh -eux
cd `dirname $0`
rm pkg.db
sqlite3 pkg.db < update.sql

