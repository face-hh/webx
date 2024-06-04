#!/usr/bin/env bash

status=$(git status)

# Check if the status contains the phrase "Your branch is behind"
if [[ $status == *"Your branch is behind"* ]]; then
    echo "Your branch is behind the remote repository."
    echo "Pulling the latest changes..."
    git pull origin $(git rev-parse --abbrev-ref HEAD)
elif [[ $status == *"Your branch is up to date"* ]]; then
    echo "Your branch is up to date with the remote repository."
else
    echo "Failed to determine the repository status."
fi
