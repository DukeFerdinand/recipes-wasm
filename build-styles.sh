#!/bin/bash

# Echoes are here to monitor 
echo " -- Building styles"
sass-rs --watch < styles/index.scss > static/index.css
echo " -- Done"