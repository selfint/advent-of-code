#!/bin/bash

year=$1
day=$2

cp -r template ./$year/$day
git checkout -b $year-$day
git add ./$year/$day
git commit -m "init solution template"

code ./$year/$day/solution
