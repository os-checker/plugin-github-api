#!/bin/bash

set -e

if [ -n "$BOT" ]; then
  echo "更新 plugin/githhub-api 目录"

  export branch=$(git branch --show-current)

  echo "bot!"

  git status
  git add .
  echo "正在提交：$(git status --porcelain)"
  git commit -m "[bot] update plugin dir from os-checker-plugin-github-api repo"
  echo "提交成功，正在推送到 database 仓库（分支：$branch）"
  git push
  echo "成功推送到 database 仓库（分支：$branch）"
fi

echo 🎇
