#!/bin/bash

# 関数定義: ランダムなファイルまたはディレクトリを作成
create_random_files_dirs() {
    local current_depth=$1
    local max_depth=$2
    local parent_dir=$3

    # ランダムにファイルまたはディレクトリを0から5個作成
    local count=$((RANDOM % 6))
    for i in $(seq 1 $count); do
        if [ $((RANDOM % 2)) -eq 0 ]; then
            # ランダムなファイルを作成
            local filename="file$(od -An -N8 -x /dev/urandom | tr -d ' ' | head -c 16).txt"
            touch "${parent_dir}/${filename}"
        else
            # ランダムなディレクトリを作成
            local dirname="dir$(od -An -N8 -x /dev/urandom | tr -d ' ' | head -c 16)"
            local new_dir="${parent_dir}/${dirname}"
            mkdir -p "$new_dir"
            if [ $current_depth -lt $max_depth ]; then
                # 再帰的にディレクトリ内部にファイル/ディレクトリを作成
                create_random_files_dirs $(($current_depth + 1)) $max_depth "$new_dir"
            fi
        fi
    done
}

# 初期ディレクトリパス
base_dir="./sample"

# ベースディレクトリを作成（存在しない場合）
mkdir -p "$base_dir"

# ランダムファイルとディレクトリを生成する関数を呼び出す
create_random_files_dirs 1 3 "$base_dir"
