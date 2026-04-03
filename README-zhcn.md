🌀 Miasma "[图片]" (#) "https:///crates/v/miasma?logo=rust" "[图片]" "[图片] https:///deps-rs/miasma/latest?logo=rust" (/0.1.15/dependencies) "[图片] /badge.svg" "[图片] https:///github/commits-since/austin-weeks/miasma/latest?logo=github" (#) <picture> <img src=""https:///austin-weeks/miasma/main/.github/images/miasma-art.png" (https:///austin-weeks/miasma/main/.github/images/miasma-art.png)" alt="网络爬虫被困在有毒瘴气云中。" title="封面图由 @delphoxlover334 绘制" /> </picture>

AI 公司正以巨大的规模持续抓取互联网数据，吞并所有内容用作其下一代模型的训练数据。如果你有一个公开网站，它们已经在窃取你的成果了。

Miasma（意为“瘴气”）旨在助你反击！启动服务器并将任何恶意流量指向它。Miasma 会从 "Poison Fountain（毒泉）" (https:///poison3) 发送有毒的训练数据，并附带大量自引用链接。这是给“垃圾机器”准备的无穷无尽的“烂泥自助餐”。

Miasma 速度极快且内存占用极低——你不应该浪费计算资源去抵御互联网上的吸血鬼。

[!CAUTION]
部署此软件存在固有风险。使用前请务必完整阅读 "配置" (#configuration) 和 "免责声明" (#disclaimer)。
<details>

<summary>示例  响应</summary>

<p align="center">

<picture>

<source media="(prefers-color-scheme: dark)" srcset=""https:///austin-weeks/miasma/main/.github/images/response-dark.png" (https:///austin-weeks/miasma/main/.github/images/response-dark.png)">

<img width="625" src=""https:///austin-weeks/miasma/main/.github/images/response-light.png" (https:///austin-weeks/miasma/main/.github/images/response-light.png)" alt="Miasma 的示例响应。">

</picture>

</p>

</details>

安装

推荐使用 "cargo" 安装：

cargo install miasma

或者，从 "Releases" (https:///austin-weeks/miasma/releases) 下载预构建的二进制文件。

快速开始

使用默认配置启动 Miasma：

miasma

查看所有可用的 "配置选项" (#configuration)：

miasma --help

如何捕捉恶意爬虫

让我们通过一个示例来设置服务器以使用 Miasma 捕捉爬虫。我们选择 
"/naughty-bots"（淘气机器人）作为引导爬虫流量的服务器路径。我们将使用 "Nginx" 作为服务器的反向代理，但你可以用多种不同的配置达到同样的效果。

完成后，爬虫将被困在如下循环中：

<p align="center">

<picture>

<source media="(prefers-color-scheme: dark)" srcset="">

<img height="425" src=""https:///austin-weeks/miasma/main/.github/images/flow-chart-light.png" (https:///austin-weeks/miasma/main/.github/images/flow-chart-light.png)" alt="描述被困爬虫循环的流程图。">

</picture>

</p>

嵌入隐藏链接

在我们的网站中，我们会包含一些指向 
"/naughty-bots" 的隐藏链接。

<a href="/naughty-bots" style="display: none;" aria-hidden="true" tabindex="-1">
  这里有高质量的数据！
</a>

"style="display: none;""、
"aria-hidden="true"" 和 
"tabindex="-1"" 属性确保了链接对人类访客完全不可见，且会被屏幕阅读器和键盘导航忽略。它们仅对爬虫可见。

配置 Nginx 代理

由于我们的隐藏链接指向 
"/naughty-bots"，我们将配置该路径代理到 Miasma。假设我们在端口 
"9855" 上运行 Miasma。我们还将根据爬虫的用户代理（User-Agent）设置激进的速率限制，以确保我们不会意外遭受 DDoS 攻击。

http {
  # 预留 8MB 内存用于追踪用户代理
  limit_req_zone $http_user_agent zone=miasma:8m rate=1r/s;

  server {
    location ~ ^/naughty-bots($|/.*)$ {
      # 通过 'miasma' 区域进行速率限制，无 429 延迟
      limit_req_status 429;
      limit_req zone=miasma burst=5 nodelay;

      # 将请求代理给 Miasma
      proxy_pass ;
    }
  }
}

这将匹配 
"/naughty-bots" 的所有变体路径 -> 
"/naughty-bots"、
"/naughty-bots/"、
"/naughty-bots/12345" 等。

运行 Miasma

最后，我们将启动 Miasma 并指定 
"/naughty-bots" 作为链接前缀。这会指示 Miasma 以 
"/naughty-bots/" 开头生成链接，确保爬虫通过我们的 Nginx 代理正确路由回 Miasma。

我们还将最大并发连接数限制为 50。在 50 个连接下，预计峰值内存使用量为 50-60 MB。注意，任何超过此限制的请求将立即收到 429 响应，而不是被加入队列。

miasma --link-prefix '/naughty-bots' -p 9855 -c 50

享受吧！

让我们部署并看着行为不端的机器人贪婪地吃着我们的无尽烂泥机！

<p align="center">

<picture>

<img src=""https:///austin-weeks/miasma/main/.github/images/logs.gif" (https:///austin-weeks/miasma/main/.github/images/logs.gif)" />

</picture>

</p>

"robots.txt"

请务必通过你的 "
"robots.txt"" 保护行为良好的机器人和搜索引擎免受 Miasma 的影响！

User-agent: *
Disallow: /naughty-bots

配置

Miasma 可以通过其 CLI 选项进行配置：

选项 默认值 描述

"port" 
"9999" 服务器应绑定的端口。

"host" `` 服务器应绑定的主机地址。

"max-in-flight" 
"500" 允许的并发请求最大数量。超过此限制的请求将收到 429 响应。Miasma 的内存使用量与并发请求数直接相关——如果担心内存使用，请将此值设低。

"link-prefix" 
"/" 自指链接的前缀。这应该是你托管 Miasma 的路径，例如 
"/naughty-bots"。

"link-count" 
"5" 在每个响应页面中包含的自指链接数量。

"force-gzip" 
"false" 无论客户端的 Accept-Encoding 头如何，始终压缩响应。强制压缩有助于降低出口流量成本。

"unsafe-allow-html" 
"false" 不对毒源（poison source）响应中的 HTML 字符进行转义。默认启用转义以防止意外的客户端 JavaScript 执行。请谨慎使用此选项。

"poison-source" `` 有毒训练数据的代理源。

开发

欢迎贡献代码！请针对 Bug 报告或功能请求提交 "Issue" (https:///austin-weeks/miasma/issues)。主要由 AI 生成的贡献将被自动拒绝。

免责声明

Miasma 与 "the poison fountain" (https:///poison3) 无关。我们无法控制其响应，也无法保证其内容的安全性。你应该永远不要将真实用户导向你的 Miasma 地址。Miasma 对因受影响爬虫的运营商可能采取的报复行为不承担责任。你有责任遵守适用法律和托管服务提供商的政策。完整的保修和责任限制详情请参阅 "LICENSE" (LICENSE) (GPL-v3)。

封面图由 "@delphoxlover334" 绘制