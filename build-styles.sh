#!/bin/bash

# Echoes are here to monitor 
echo " -- Building styles"
sass styles/index.scss:static/index.css
echo " -- Done"