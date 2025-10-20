#!/bin/bash
# 测试运行脚本 - 启动服务器并等待5秒然后关闭

cd "$(dirname "$0")"

echo "编译项目..."
cargo build 2>&1 | tail -5

if [ $? -ne 0 ]; then
    echo "❌ 编译失败"
    exit 1
fi

echo ""
echo "启动服务器..."
./target/debug/actix-web-example > /tmp/actix-web-example.log 2>&1 &
PID=$!

echo "进程 PID: $PID"
sleep 3

# 检查进程是否还在运行
if ps -p $PID > /dev/null; then
    echo "✅ 服务器启动成功"
    echo ""
    echo "日志输出:"
    head -20 /tmp/actix-web-example.log
    echo ""
    echo "停止服务器..."
    kill $PID
    wait $PID 2>/dev/null
    echo "✅ 测试完成"
else
    echo "❌ 服务器启动失败"
    echo ""
    echo "错误日志:"
    cat /tmp/actix-web-example.log
    exit 1
fi

