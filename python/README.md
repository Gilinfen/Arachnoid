# Python

## Build

```sh
python3 \
-m nuitka \
--output-dir=out \
--show-progress \
--show-memory \
--include-module=utils \
./main.py
```

## 微信公众号验证方式

**设置微信公众号**：

- 如果您还没有公众号，您需要先注册一个。
- 在微信公众平台上配置您的公众号，包括设置 API 权限和获取必要的 AppID 和 AppSecret。

**用户关注公众号**：

- 当用户关注您的公众号时，微信会发送一个事件通知到您设置的服务器 URL。
- 您的服务器需要处理这个事件通知，并获取用户的 OpenID。

**使用 OpenID 进行身份验证和权限检查**：

- 使用用户的 OpenID 来识别用户身份。
- 在您的数据库中检查该用户是否有权限使用您的软件。这可能涉及到检查用户是否已购买您的产品或服务。

**反馈给用户**：

- 如果用户有权限，您可以通过公众号向用户发送消息，告知他们如何访问或下载您的软件。
- 如果用户没有权限，您可以提供购买链接或其他相关信息。

## Nuitka

打包指令示例：
python -m nuitka --mingw64 --standalone --show-progress --show-memory
--plugin-enable=qt-plugins --plugin-enable=pylint-warnings
--nofollow-import-to=numpy,matplotlib,scipy,pandas
--output-dir=D:\out main.py

常用参数含义说明：
--mingw64 默认为已经安装的 vs2017 去编译，否则就按指定的比如 mingw(官方建议)
--standalone 独立环境，这是必须的(否则拷给别人无法使用)
--windows-disable-console 没有 CMD 控制窗口
--output-dir=out 生成 exe 到 out 文件夹下面去
--show-progress 显示编译的进度，很直观
--show-memory 显示内存的占用
--include-qt-plugins=sensible,styles 打包后 PyQt 的样式就不会变了
--plugin-enable=qt-plugins 需要加载的 PyQt 插件
--plugin-enable=tk-inter 打包 tkinter 模块的刚需
--plugin-enable=numpy 打包 numpy,pandas,matplotlib 模块的刚需
--plugin-enable=torch 打包 pytorch 的刚需
--plugin-enable=tensorflow 打包 tensorflow 的刚需
--windows-icon-from-ico=你的.ico 软件的图标
--windows-company-name=Windows 下软件公司信息
--windows-product-name=Windows 下软件名称
--windows-file-version=Windows 下软件的信息
--windows-product-version=Windows 下软件的产品信息
--windows-file-description=Windows 下软件的作用描述
--windows-uac-admin=Windows 下用户可以使用管理员权限来安装
--linux-onefile-icon=Linux 下的图标位置
--onefile 像 pyinstaller 一样打包成单个 exe 文件，慎用，问题很多
--remove-output 删除编译临时文件
--include-package=复制比如 numpy,PyQt5 这些带文件夹的叫包或者轮子
--include-module=复制比如 when.py 这些以.py 结尾的叫模块
