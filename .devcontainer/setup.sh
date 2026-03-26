#!/bin/bash

# Grant the container user access to /dev/kvm
sudo chown -R $REMOTE_USER:$REMOTE_GROUP $DEVICE
