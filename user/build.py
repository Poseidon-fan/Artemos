import os

base_address = 0x80400000
step = 0x20000
linker = 'src/linker.ld'

app_id = 0
apps = os.listdir('src/bin')
apps.sort()
for app in apps:
    app = app[:app.find('.')]
    lines = []
    lines_before = []
    with open(linker, 'r') as f:
        # 更改 linker 中 BASE_ADDRESS 对应的地址
        for line in f.readlines():
            lines_before.append(line)
            line = line.replace(hex(base_address), hex(base_address+step*app_id))
            lines.append(line)
    with open(linker, 'w+') as f:
        f.writelines(lines)
    # 构建当前的应用
    os.system('set RUSTFLAGS=-Clink-args=-Tuser/src/linker.ld -Cforce-frame-pointers=true && cargo build --bin %s --release' % app)
    # 使用下面这行只能联系到全局的链接脚本
    # os.system('cargo build --bin %s --release' % app)
    print('[build.py] application %s start with address %s' %(app, hex(base_address+step*app_id)))
    # 还原 linker 脚本，主要变化就是把 BASE_ADDRESS 改回了 0x80400000
    with open(linker, 'w+') as f:
        f.writelines(lines_before)
    app_id = app_id + 1